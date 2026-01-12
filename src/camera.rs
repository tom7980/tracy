use crate::bvh::BvhTree;
use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

use indicatif::{MultiProgress, ProgressBar};
use rand::prelude::*;
use rayon::prelude::*;

use std::fs::File;
use std::io::Write;
use std::io::{self, BufWriter};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Camera {
    image_height: u64,
    image_width: u64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    aspect_ratio: f64,
    samples_per_pixel: i32,
    sample_scale_factor: f64,
    out_file: BufWriter<File>,
    max_depth: u32,

    vfov: f64,

    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    focus_angle: f64,

    rng_src: Arc<Mutex<SmallRng>>,
    background: Colour,
}

impl Camera {
    pub fn new<P>(
        aspect_ratio: f64,
        image_width: u64,
        vfov: f64,
        center: Point3,
        look_at: Point3,
        up_vec: Vec3,
        focus_distance: f64,
        focus_angle: f64,
        filename: P,
    ) -> Result<Camera, io::Error>
    where
        P: AsRef<Path>,
    {
        let image_height: u64 = {
            let x = image_width as f64 / aspect_ratio;
            if x < 1.0 {
                1
            } else {
                x as u64
            }
        };

        // Default to 90 degree FOV at first
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h * focus_distance;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        let w = unit_vector(Vec3::from(center - look_at));
        let u = unit_vector(cross(up_vec, w));
        let v = cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            (center - (focus_distance * w)) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let samples_per_pixel = 10;
        let sample_scale_factor = 1.0 / samples_per_pixel as f64;
        let file = File::create(filename)?;
        let bufwriter = BufWriter::new(file);

        let defocus_radius = focus_distance * (focus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Ok(Camera {
            image_height,
            image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            aspect_ratio,
            samples_per_pixel,
            sample_scale_factor,
            out_file: bufwriter,
            max_depth: 10,
            vfov,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
            focus_angle,

            rng_src: Arc::new(Mutex::new(SmallRng::from_os_rng())),
            background: Colour::new(0.0, 0.0, 0.0),
        })
    }

    pub fn set_samples_per_pixel(&mut self, samples: i32) {
        self.samples_per_pixel = samples;
        self.sample_scale_factor = 1.0 / samples as f64;
    }

    pub fn set_max_depth(&mut self, depth: u32) {
        self.max_depth = depth;
    }

    pub fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    pub fn render(&mut self, world: &BvhTree) -> io::Result<()> {
        write!(
            self.out_file,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;

        let mp = MultiProgress::new();

        let bar_j = mp.add(ProgressBar::new(self.image_height));

        (0..self.image_height).for_each(|j| {
            bar_j.inc(1);
            let bar_i = mp.add(ProgressBar::new(self.image_width));
            let pixel_colours: Vec<_> = (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    bar_i.inc(1);
                    let mut avg_colour = Colour::new(0.0, 0.0, 0.0);
                    (0..self.samples_per_pixel).for_each(|_| {
                        let r = self.make_ray(i, j);
                        avg_colour += self.ray_colour(&r, self.max_depth, &world);
                    });
                    avg_colour
                })
                .collect();
            for pix in pixel_colours {
                self.out_file
                    .write_fmt(format_args!("{}", pix * self.sample_scale_factor))
                    .unwrap();
            }
            bar_i.finish();
            mp.remove(&bar_i);
        });

        bar_j.finish();
        self.out_file.flush()
    }

    fn ray_colour(&self, ray: &Ray, depth: u32, world: &BvhTree) -> Colour {
        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        if let Some(record) = world.hit(ray, 0.001, f64::INFINITY) {
            let emitted = record
                .material_ref()
                .emit(record.u, record.v, &record.hit_pos())
                .unwrap_or(Colour::new(0.0, 0.0, 0.0));

            if let Some(scatter) = record.material_ref().scatter(ray, &record) {
                let scatter_pdf =
                    record
                        .material_ref()
                        .scatter_pdf(ray, &record, scatter.scattered_ref());
                let pdf_val = scatter_pdf;

                let scatter_colour =
                    (Colour::from(self.ray_colour(scatter.scattered_ref(), depth - 1, world))
                        * scatter.attenuation()
                        * scatter_pdf)
                        / pdf_val;

                return scatter_colour + emitted;
            } else {
                return emitted;
            }
        }

        self.background

        // let direction = unit_vector(ray.direction());
        // let scale = 0.5 * (direction.y() + 1.0);
        // (1.0 - scale) * Colour::new(1.0, 1.0, 1.0) + scale * Colour::new(0.5, 0.7, 1.0)
    }

    fn sample_square(&self) -> Vec3 {
        let mut guard = self.rng_src.lock().expect("Poisoned");

        Vec3::new(
            guard.random::<f64>() - 0.5,
            guard.random::<f64>() - 0.5,
            0.0,
        )
    }

    fn make_ray(&self, i: u64, j: u64) -> Ray {
        let offset = self.sample_square();

        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.focus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = Vec3::from(pixel_sample - ray_origin);
        let ray_time = self
            .rng_src
            .lock()
            .expect("Poisoned RNG source")
            .random::<f64>();
        Ray::new(ray_origin, ray_direction, ray_time)
    }
}

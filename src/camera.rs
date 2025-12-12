use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

use indicatif::{MultiProgress, ProgressBar};
use rand::prelude::*;

use std::fs::File;
use std::io::Write;
use std::io::{self, BufWriter};
use std::path::Path;

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
}

impl Camera {
    pub fn new<P>(aspect_ratio: f64, image_width: u64, filename: P) -> Result<Camera, io::Error>
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

        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        let focal_length: f64 = 1.0;
        let center: Point3 = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            (center - Vec3::new(0.0, 0.0, focal_length)) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let samples_per_pixel = 10;
        let sample_scale_factor = 1.0 / samples_per_pixel as f64;
        let file = File::create(filename)?;
        let bufwriter = BufWriter::new(file);

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
        })
    }

    pub fn set_samples_per_pixel(&mut self, samples: i32) {
        self.samples_per_pixel = samples;
        self.sample_scale_factor = 1.0 / samples as f64;
    }

    pub fn set_max_depth(&mut self, depth: u32) {
        self.max_depth = depth;
    }

    pub fn render(&mut self, world: &HittableList) -> io::Result<()> {
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
            (0..self.image_width).for_each(|i| {
                bar_i.inc(1);
                let mut avg_colour = Colour::new(0.0, 0.0, 0.0);
                (0..self.samples_per_pixel).for_each(|_| {
                    let r = self.make_ray(i, j);
                    avg_colour += self.ray_colour(&r, self.max_depth, &world);
                });
                self.out_file
                    .write_fmt(format_args!("{}", avg_colour * self.sample_scale_factor))
                    .unwrap();
            });
            bar_i.finish();
            mp.remove(&bar_i);
        });

        // for j in 0..self.image_height {
        //     bar_j.inc(1);
        //     let bar_i = mp.add(ProgressBar::new(self.image_width));
        //     for i in 0..self.image_width {
        //         bar_i.inc(1);
        //         let mut avg_colour = Colour::new(0.0, 0.0, 0.0);
        //         for _ in 0..self.samples_per_pixel {
        //             let r = self.make_ray(i, j);
        //             avg_colour += self.ray_colour(&r, self.max_depth, &world);
        //         }
        //         write!(self.out_file, "{}", avg_colour * self.sample_scale_factor)?;
        //     }
        //     bar_i.finish();
        //     mp.remove(&bar_i);
        // }

        bar_j.finish();
        self.out_file.flush()
    }

    fn ray_colour(&self, ray: &Ray, depth: u32, world: &HittableList) -> Colour {
        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        if let Some(record) = world.hit(ray, 0.001, f64::INFINITY) {
            let direction = record.normal() + Vec3::random_unit_vector();
            return Colour::from(
                self.ray_colour(&Ray::new(record.hit_pos(), direction), depth - 1, world) * 0.5,
            );
        }

        let direction = unit_vector(ray.direction());
        let scale = 0.5 * (direction.y() + 1.0);
        (1.0 - scale) * Colour::new(1.0, 1.0, 1.0) + scale * Colour::new(0.5, 0.7, 1.0)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.0)
    }

    fn make_ray(&self, i: u64, j: u64) -> Ray {
        let offset = self.sample_square();

        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = Vec3::from(pixel_sample - ray_origin);

        Ray::new(ray_origin, ray_direction)
    }
}

use crate::{hittable::*, ray::*, texture::*, vec3::*};
use core::f64;
use rand::Rng;
use std::sync::Arc;

pub struct ScatterRecord {
    attenuation: Colour,
    scattered: Ray,
}

impl ScatterRecord {
    pub fn attenuation_ref(&self) -> &Colour {
        &self.attenuation
    }

    pub fn attenuation(&self) -> Colour {
        self.attenuation
    }

    pub fn scattered_ref(&self) -> &Ray {
        &self.scattered
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;

    fn emit(&self, u: f64, v: f64, p: &Point3) -> Option<Colour> {
        None
    }

    fn scatter_pdf(&self, ray: &Ray, hit_record: &HitRecord, scatter_ray: &Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn as_arc(albedo: Arc<dyn Texture>) -> Arc<Lambertian> {
        Arc::new(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit_record.normal() + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal();
        }
        Some(ScatterRecord {
            attenuation: self
                .albedo
                .value(hit_record.u, hit_record.v, hit_record.hit_pos()),
            scattered: Ray::new(hit_record.hit_pos(), scatter_direction, ray.time()),
        })
    }

    fn scatter_pdf(&self, ray: &Ray, hit_record: &HitRecord, scatter_ray: &Ray) -> f64 {
        let cos_theta = dot(hit_record.normal(), unit_vector(scatter_ray.direction()));
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / f64::consts::PI
        }
    }
}

pub struct Metalic {
    albedo: Colour,
    fuzz: f64,
}

impl Metalic {
    pub fn new(albedo: Colour, fuzz: f64) -> Metalic {
        Metalic {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }

    pub fn as_arc(albedo: Colour, fuzz: f64) -> Arc<Metalic> {
        Arc::new(Metalic {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        })
    }
}

impl Material for Metalic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray.direction().reflect(&hit_record.normal())
            + (self.fuzz * Vec3::random_unit_vector());

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.hit_pos(), reflected, ray.time()),
        })
    }
}

pub struct Dielectric {
    refractive_index: f64,
    albedo: Colour,
}

impl Dielectric {
    pub fn new(refractive_index: f64, albedo: Colour) -> Dielectric {
        Dielectric {
            refractive_index,
            albedo,
        }
    }

    pub fn as_arc(refractive_index: f64, albedo: Colour) -> Arc<Dielectric> {
        Arc::new(Dielectric {
            refractive_index,
            albedo,
        })
    }

    fn reflectance(&self, cosine: f64) -> f64 {
        let mut r0 = (1.0 - self.refractive_index) / (1.0 + self.refractive_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let ri = if hit_record.front_face() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = unit_vector(ray.direction());
        let cos_theta = dot(-unit_direction, hit_record.normal()).min(1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cant_refract = (ri * sin_theta) > 1.0;

        let direction;
        let mut rng = rand::rng();
        if cant_refract || self.reflectance(cos_theta) > rng.random() {
            direction = unit_direction.reflect(&hit_record.normal());
        } else {
            direction = unit_direction.refract(&hit_record.normal(), ri)
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.hit_pos(), direction, ray.time()),
        })
    }
}

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { texture }
    }

    pub fn from_colour(colour: Colour) -> DiffuseLight {
        DiffuseLight {
            texture: Arc::new(SolidColour::new(colour)),
        }
    }

    pub fn as_arc_from_colour(colour: Colour) -> Arc<DiffuseLight> {
        Arc::new(DiffuseLight {
            texture: SolidColour::as_arc(colour),
        })
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emit(&self, u: f64, v: f64, p: &Point3) -> Option<Colour> {
        Some(self.texture.value(u, v, *p))
    }
}

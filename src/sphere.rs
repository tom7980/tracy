use crate::hittable::*;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc: Vec3 = (self.center - ray.origin()).into();
        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (h - sqrtd) / a;

        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;

        let mut hit_record = HitRecord::new(p, normal.into(), root, self.mat.clone());
        hit_record.set_face_normal(ray, normal.into());

        Some(hit_record)
    }
}

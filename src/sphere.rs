use crate::bounding::*;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

use core::f64;
use std::sync::Arc;

pub struct Sphere {
    movement: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bounding: BoundingBox,
}

impl Sphere {
    pub fn new(movement: Ray, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);

        let box1 = BoundingBox::new(movement.at(0.0) - rvec, movement.at(0.0) + rvec);
        let box2 = BoundingBox::new(movement.at(1.0) - rvec, movement.at(1.0) + rvec);

        let movement_bounds = BoundingBox::box_between(&box1, &box2);

        println!("Bounds of Sphere: {:?}", movement_bounds);
        Sphere {
            movement,
            radius,
            mat,
            bounding: movement_bounds,
        }
    }

    pub fn get_sphere_uv(&self, p: &Point3) -> (f64, f64) {
        let theta = f64::acos(-p.axis(1));
        let phi = f64::atan2(-p.axis(2), p.axis(0)) + f64::consts::PI;

        (phi / (2.0 * f64::consts::PI), theta / f64::consts::PI)
    }
}

impl Hittable for Sphere {
    fn bounding_box(&self) -> &BoundingBox {
        &self.bounding
    }

    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let current_position = self.movement.at(ray.time());
        let oc: Vec3 = (current_position - ray.origin()).into();
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
        let normal = (p - current_position) / self.radius;

        let (u, v) = self.get_sphere_uv(&normal);

        let mut hit_record = HitRecord::new(p, normal.into(), root, self.mat.clone(), u, v);
        hit_record.set_face_normal(ray, normal.into());

        Some(hit_record)
    }
}

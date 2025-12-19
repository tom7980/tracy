use crate::bounding::*;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

use std::ops::Range;
use std::sync::Arc;

pub struct Quad<F>
where
    F: Fn(String),
{
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Arc<dyn Material>,
    bounds: BoundingBox,

    normal: Vec3,
    d: f64,
    w: Vec3,

    f: F,
}

impl<F: Fn(String)> Quad<F> {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>, f: F) -> Quad<F> {
        let bound1 = BoundingBox::new(q, q + u + v);
        let bound2 = BoundingBox::new(q + u, q + v);

        let full_bounds = BoundingBox::box_between(&bound1, &bound2);

        println!("Quad Bounds {:?}", full_bounds);

        let n = cross(u, v);
        let normal = unit_vector(n);

        let d = dot(normal, q.into());

        let w = n / dot(n, n);
        Quad {
            q,
            u,
            v,
            mat,
            bounds: full_bounds,
            normal,
            d,
            w,
            f,
        }
    }

    pub fn is_interior(&self, a: &f64, b: &f64) -> Option<(f64, f64)> {
        let range: Range<f64> = 0.0..1.0;

        if !range.contains(a) || !range.contains(b) {
            None
        } else {
            Some((*a, *b))
        }
    }
}

impl<F: Fn(String) + Send + Sync> Hittable for Quad<F> {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let denom = dot(self.normal, r.direction());

        if f64::abs(denom) < 1e-8 {
            return None;
        }

        let t = (self.d - dot(self.normal, r.origin().into())) / denom;
        if ray_tmin > t || t > ray_tmax {
            return None;
        }

        let intersection = r.at(t);
        let planar_hit_vec = intersection - self.q;

        let alpha = dot(self.w, cross(planar_hit_vec.into(), self.v));
        let beta = dot(self.w, cross(self.u, planar_hit_vec.into()));

        if let Some((u, v)) = self.is_interior(&alpha, &beta) {
            // (self.f)(format_args!("Intersection with Quad at: {:?}", intersection).to_string());
            let mut record = HitRecord::new(intersection, self.normal, t, self.mat.clone(), u, v);
            record.set_face_normal(r, self.normal);

            // (self.f)(format_args!("Face normal: {:?}", record.normal()).to_string());
            Some(record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounds
    }
}

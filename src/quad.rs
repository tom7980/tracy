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

    pub fn boxed(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>, f: F) -> Box<Quad<F>> {
        Box::new(Quad::new(q, u, v, mat, f))
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

pub struct Cube {
    sides: Box<HittableList>,
}

impl Cube {
    pub fn new(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Cube {
        let mut sides = Box::new(HittableList::new());

        let min = a.most_minimum(b);
        let max = a.most_maximum(b);

        let dx = Vec3::new(max.axis(0) - min.axis(0), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.axis(1) - min.axis(1), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.axis(2) - min.axis(2));

        sides.add(Quad::boxed(
            Point3::new(min.axis(0), min.axis(1), max.axis(2)),
            dx,
            dy,
            mat.clone(),
            |_| {},
        ));
        sides.add(Quad::boxed(
            Point3::new(max.axis(0), min.axis(1), max.axis(2)),
            -dz,
            dy,
            mat.clone(),
            |_| {},
        ));
        sides.add(Quad::boxed(
            Point3::new(max.axis(0), min.axis(1), min.axis(2)),
            -dx,
            dy,
            mat.clone(),
            |_| {},
        ));
        sides.add(Quad::boxed(
            Point3::new(min.axis(0), min.axis(1), min.axis(2)),
            dz,
            dy,
            mat.clone(),
            |_| {},
        ));
        sides.add(Quad::boxed(
            Point3::new(min.axis(0), max.axis(1), max.axis(2)),
            dx,
            -dz,
            mat.clone(),
            |_| {},
        ));
        sides.add(Quad::boxed(
            Point3::new(min.axis(0), min.axis(1), min.axis(2)),
            dx,
            dz,
            mat.clone(),
            |_| {},
        ));

        Cube { sides }
    }

    pub fn boxed(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Box<Cube> {
        Box::new(Cube::new(a, b, mat))
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        self.sides.hit(r, ray_tmin, ray_tmax)
    }

    fn bounding_box(&self) -> &BoundingBox {
        self.sides.bounding_box()
    }
}

use crate::bounding::*;
use crate::material;
use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;
use core::f64;
use std::sync::Arc;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    pub t: f64,
    front_face: bool,
    material: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f64,
        material: Arc<dyn Material>,
        u: f64,
        v: f64,
    ) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face: false,
            material,
            u,
            v,
        }
    }

    pub fn material_ref(&self) -> &dyn Material {
        self.material.as_ref()
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn update_record(&mut self, p: Point3, normal: Vec3, t: f64) {
        self.p = p;
        self.normal = normal;
        self.t = t;
    }

    pub fn set_face_normal(&mut self, ray: &Ray, out_normal: Vec3) {
        self.front_face = dot(ray.direction(), out_normal) < 0.0;
        self.normal = if self.front_face {
            out_normal
        } else {
            -out_normal
        };
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn hit_pos(&self) -> Point3 {
        self.p
    }
}

pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
    bounds: BoundingBox,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            hittables: Vec::new(),
            bounds: BoundingBox::empty(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        let bounds = BoundingBox::box_between(&self.bounds, object.bounding_box());
        self.bounds = bounds;
        self.hittables.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = t_max;

        self.hittables.iter().for_each(|x| {
            if let Some(last_record) = x.hit(ray, t_min, closest_so_far) {
                closest_so_far = last_record.t;
                record = Some(last_record);
            }
        });
        record
    }

    fn bounding_box(&self) -> &BoundingBox {
        &self.bounds
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;

    fn bounding_box(&self) -> &BoundingBox;
}

pub struct Translate {
    object: Box<dyn Hittable>,
    bounds: BoundingBox,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: &Vec3) -> Translate {
        let bounds = object.bounding_box() + offset;

        Translate {
            object,
            bounds,
            offset: *offset,
        }
    }

    pub fn boxed(object: Box<dyn Hittable>, offset: &Vec3) -> Box<Translate> {
        Box::new(Translate::new(object, offset))
    }
}

impl Hittable for Translate {
    fn bounding_box(&self) -> &BoundingBox {
        &self.bounds
    }

    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let offset_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if let Some(mut hit) = self.object.hit(&offset_r, ray_tmin, ray_tmax) {
            hit.p += self.offset;
            return Some(hit);
        } else {
            None
        }
    }
}

pub struct RotateY {
    cos_theta: f64,
    sin_theta: f64,
    object: Box<dyn Hittable>,
    bounds: BoundingBox,
}

impl RotateY {
    pub fn new(object: Box<dyn Hittable>, angle: f64) -> RotateY {
        let radians = angle.to_radians();
        let cos_theta = f64::cos(radians);
        let sin_theta = f64::sin(radians);

        let obj_box = object.bounding_box();

        let bounds = obj_box.rotate_y(cos_theta, sin_theta);

        RotateY {
            cos_theta,
            sin_theta,
            object,
            bounds,
        }
    }

    pub fn boxed(object: Box<dyn Hittable>, angle: f64) -> Box<RotateY> {
        Box::new(RotateY::new(object, angle))
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> &BoundingBox {
        &self.bounds
    }

    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let origin = Point3::new(
            (self.cos_theta * r.origin().axis(0)) - (self.sin_theta * r.origin().axis(2)),
            r.origin().axis(1),
            (self.sin_theta * r.origin().axis(0)) + (self.cos_theta * r.origin().axis(2)),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(mut hit) = self.object.hit(&rotated_r, ray_tmin, ray_tmax) {
            hit.p = Point3::new(
                (self.cos_theta * hit.p.axis(0)) + (self.sin_theta * hit.p.axis(2)),
                hit.p.axis(1),
                (-self.sin_theta * hit.p.axis(0)) + (self.cos_theta * hit.p.axis(2)),
            );

            hit.normal = Vec3::new(
                (self.cos_theta * hit.normal.axis(0)) + (self.sin_theta * hit.normal.axis(2)),
                hit.normal.axis(1),
                (-self.sin_theta * hit.normal.axis(0)) + (self.cos_theta * hit.normal.axis(2)),
            );

            return Some(hit);
        } else {
            None
        }
    }
}

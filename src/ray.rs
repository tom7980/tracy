use crate::vec3::*;

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn at(&self, t: f64) -> Point3 {
        Point3::from(self.origin() + self.direction * t)
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}

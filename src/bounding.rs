use core::f64;
use std::f64::{INFINITY, NEG_INFINITY};
use std::ops::{Add, AddAssign};

use crate::ray::*;
use crate::vec3::*;

#[derive(Debug)]
pub struct BoundingBox {
    lower: Point3,
    upper: Point3,
}

pub struct IntersectionRecord {
    pub tmin: f64,
    pub tmax: f64,
}

impl BoundingBox {
    pub fn new(a: Point3, b: Point3) -> BoundingBox {
        let upper_x = if a.axis(0) > b.axis(0) {
            a.axis(0)
        } else {
            b.axis(0)
        };
        let upper_y = if a.axis(1) > b.axis(1) {
            a.axis(1)
        } else {
            b.axis(1)
        };
        let upper_z = if a.axis(2) > b.axis(2) {
            a.axis(2)
        } else {
            b.axis(2)
        };
        let lower_x = if a.axis(0) < b.axis(0) {
            a.axis(0)
        } else {
            b.axis(0)
        };
        let lower_y = if a.axis(1) < b.axis(1) {
            a.axis(1)
        } else {
            b.axis(1)
        };
        let lower_z = if a.axis(2) < b.axis(2) {
            a.axis(2)
        } else {
            b.axis(2)
        };

        let upper = Point3::new(upper_x, upper_y, upper_z);
        let lower = Point3::new(lower_x, lower_y, lower_z);

        let mut out = BoundingBox { lower, upper };

        out.pad_minimum();

        out
    }

    pub fn empty() -> BoundingBox {
        BoundingBox {
            lower: Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            upper: Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    pub fn longest_axis(&self) -> usize {
        let lengths = [
            self.axis_length(0),
            self.axis_length(1),
            self.axis_length(2),
        ];

        if let Some((idx, _)) = lengths
            .iter()
            .enumerate()
            .max_by(|(_idx1, val1), (_idx2, val2)| val1.partial_cmp(val2).unwrap())
        {
            idx
        } else {
            0
        }
    }

    fn pad_minimum(&mut self) {
        const PADDING: f64 = 0.0001;
        for axis in 0..3 {
            if self.axis_length(axis) < PADDING {
                self.upper.modify_axis(axis, |val| val + PADDING / 2.0);
                self.lower.modify_axis(axis, |val| val - PADDING / 2.0);
            }
        }
    }

    pub fn axis_length(&self, axis: usize) -> f64 {
        let lower = self.lower.axis(axis);
        let upper = self.upper.axis(axis);

        f64::abs(upper - lower)
    }

    pub fn box_between(a: &BoundingBox, b: &BoundingBox) -> BoundingBox {
        let lower = a.lower.most_minimum(b.lower);
        let upper = a.upper.most_maximum(b.upper);

        BoundingBox { lower, upper }
    }

    pub fn rotate_y(&self, cos_theta: f64, sin_theta: f64) -> BoundingBox {
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = self.upper.axis(0) * i as f64 + self.lower.axis(0) * (1.0 - i as f64);
                    let y = self.upper.axis(1) * j as f64 + self.lower.axis(1) * (1.0 - j as f64);
                    let z = self.upper.axis(2) * k as f64 + self.lower.axis(2) * (1.0 - k as f64);

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let test = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min.modify_axis(c, |val| f64::min(val, test.axis(c)));
                        max.modify_axis(c, |val| f64::max(val, test.axis(c)));
                    }
                }
            }
        }

        BoundingBox {
            lower: min,
            upper: max,
        }
    }

    pub fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<IntersectionRecord> {
        let origin = ray.origin();
        let direction = ray.direction();

        let mut tmin_out = t_min;
        let mut tmax_out = t_max;

        for axis in 0..3 {
            let ax_min = self.lower.axis(axis);
            let ax_max = self.upper.axis(axis);

            let adinv = 1.0 / direction.axis(axis);

            let t0 = (ax_min - origin.axis(axis)) * adinv;
            let t1 = (ax_max - origin.axis(axis)) * adinv;

            if t0 < t1 {
                if t0 > t_min {
                    tmin_out = t0;
                }
                if t1 < t_max {
                    tmax_out = t1;
                }
            } else {
                if t1 > t_min {
                    tmin_out = t1;
                }
                if t0 < t_max {
                    tmax_out = t0;
                }
            }

            if tmax_out <= tmin_out {
                return None;
            }
        }

        Some(IntersectionRecord {
            tmin: tmin_out,
            tmax: tmax_out,
        })
    }
}

impl Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Self) -> Self::Output {
        BoundingBox {
            lower: self.lower + rhs.lower,
            upper: self.upper + rhs.upper,
        }
    }
}

impl AddAssign for BoundingBox {
    fn add_assign(&mut self, rhs: Self) {
        self.upper = self.upper + rhs.upper;
        self.lower = self.lower + rhs.lower;
    }
}

impl Add<Vec3> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Vec3) -> Self::Output {
        BoundingBox {
            lower: self.lower + rhs,
            upper: self.upper + rhs,
        }
    }
}

impl Add<&Vec3> for &BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: &Vec3) -> Self::Output {
        BoundingBox {
            upper: self.upper + *rhs,
            lower: self.lower + *rhs,
        }
    }
}

impl AddAssign<Vec3> for BoundingBox {
    fn add_assign(&mut self, rhs: Vec3) {
        self.lower = self.lower + rhs;
        self.upper = self.upper + rhs;
    }
}

impl Add<BoundingBox> for Vec3 {
    type Output = BoundingBox;

    fn add(self, rhs: BoundingBox) -> Self::Output {
        BoundingBox {
            lower: rhs.lower + self,
            upper: rhs.upper + self,
        }
    }
}

use core::f64;

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
        let upper_x = if a.offset(0) > b.offset(0) {
            a.offset(0)
        } else {
            b.offset(0)
        };
        let upper_y = if a.offset(1) > b.offset(1) {
            a.offset(1)
        } else {
            b.offset(1)
        };
        let upper_z = if a.offset(2) > b.offset(2) {
            a.offset(2)
        } else {
            b.offset(2)
        };
        let lower_x = if a.offset(0) < b.offset(0) {
            a.offset(0)
        } else {
            b.offset(0)
        };
        let lower_y = if a.offset(1) < b.offset(1) {
            a.offset(1)
        } else {
            b.offset(1)
        };
        let lower_z = if a.offset(2) < b.offset(2) {
            a.offset(2)
        } else {
            b.offset(2)
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
        let lower = self.lower.offset(axis);
        let upper = self.upper.offset(axis);

        f64::abs(upper - lower)
    }

    pub fn box_between(a: &BoundingBox, b: &BoundingBox) -> BoundingBox {
        let lower = a.lower.most_minimum(b.lower);
        let upper = a.upper.most_maximum(b.upper);

        BoundingBox { lower, upper }
    }

    pub fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<IntersectionRecord> {
        let origin = ray.origin();
        let direction = ray.direction();

        let mut tmin_out = t_min;
        let mut tmax_out = t_max;

        for axis in 0..3 {
            let ax_min = self.lower.offset(axis);
            let ax_max = self.upper.offset(axis);

            let adinv = 1.0 / direction.offset(axis);

            let t0 = (ax_min - origin.offset(axis)) * adinv;
            let t1 = (ax_max - origin.offset(axis)) * adinv;

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

use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rand::Rng;

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn axis(&self, x: usize) -> f64 {
        self.e[x]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.e[0]) < s) && (f64::abs(self.e[1]) < s) && (f64::abs(self.e[2]) < s)
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2.0 * dot(*self, *normal) * *normal
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = dot(-*self, *normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * *normal;
        r_out_parallel + r_out_perp
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::rng();
        Vec3 {
            e: [rng.random(), rng.random(), rng.random()],
        }
    }

    pub fn random_with_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::rng();
        Vec3 {
            e: [
                rng.random_range(min..max),
                rng.random_range(min..max),
                rng.random_range(min..max),
            ],
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_with_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if lensq <= 1.0 && lensq > 1e-160 {
                return p / f64::sqrt(lensq);
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if dot(on_unit_sphere, *normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::rng();
        loop {
            let p = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.e[0]
            .partial_cmp(&other.e[0])
            .and(self.e[1].partial_cmp(&other.e[1]))
            .and(self.e[2].partial_cmp(&other.e[2]))
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.e[0].eq(&other.e[0]) && self.e[1].eq(&other.e[1]) && self.e[2].eq(&other.e[2])
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Vec3 {
        Vec3::new(self.x() + other, self.y() + other, self.z() + other)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.x() / other, self.y() / other, self.z() / other)
    }
}

// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.data = self.data + rhs;
    }
}

// Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = *self * t;
    }
}

// Vec3 /= f64
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = *self / t;
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

impl From<Point3> for Vec3 {
    fn from(point: Point3) -> Self {
        point.data
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Point3 {
    data: Vec3,
}

impl PartialOrd for Point3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl PartialEq for Point3 {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {
            data: Vec3::new(x, y, z),
        }
    }

    pub fn axis(&self, i: usize) -> f64 {
        self.data.axis(i)
    }

    pub fn modify_axis<F>(&mut self, axis: usize, fun: F)
    where
        F: Fn(f64) -> f64,
    {
        let current_val = self.data.axis(axis);
        let updated = fun(current_val);
        self.data.e[axis] = updated
    }

    pub fn most_minimum(&self, other: Point3) -> Point3 {
        let x = if self.axis(0) <= other.axis(0) {
            self.axis(0)
        } else {
            other.axis(0)
        };
        let y = if self.axis(1) <= other.axis(1) {
            self.axis(1)
        } else {
            other.axis(1)
        };
        let z = if self.axis(2) <= other.axis(2) {
            self.axis(2)
        } else {
            other.axis(2)
        };

        Point3 {
            data: Vec3::new(x, y, z),
        }
    }

    pub fn most_maximum(&self, other: Point3) -> Point3 {
        let x = if self.axis(0) >= other.axis(0) {
            self.axis(0)
        } else {
            other.axis(0)
        };
        let y = if self.axis(1) >= other.axis(1) {
            self.axis(1)
        } else {
            other.axis(1)
        };
        let z = if self.axis(2) >= other.axis(2) {
            self.axis(2)
        } else {
            other.axis(2)
        };

        Point3 {
            data: Vec3::new(x, y, z),
        }
    }
}

impl From<Vec3> for Point3 {
    fn from(data: Vec3) -> Self {
        Point3 { data }
    }
}

impl Sub for Point3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Point3 {
        Point3::from(self.data - other.data)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self {
        Point3::from(self.data - other)
    }
}

impl Div<f64> for Point3 {
    type Output = Point3;

    fn div(self, other: f64) -> Point3 {
        Point3::from(self.data / other)
    }
}

impl Add for Point3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Point3::from(self.data + other.data)
    }
}

impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self {
        Point3::from(self.data + other)
    }
}

#[derive(Copy, Clone, Default)]
pub struct Colour {
    data: Vec3,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour {
            data: Vec3::new(r, g, b),
        }
    }

    pub fn r(&self) -> f64 {
        self.data.e[0]
    }

    pub fn g(&self) -> f64 {
        self.data.e[1]
    }

    pub fn b(&self) -> f64 {
        self.data.e[2]
    }

    pub fn gamma_corrected(&self) -> Colour {
        let r = Colour::correct_component(self.r());
        let g = Colour::correct_component(self.g());
        let b = Colour::correct_component(self.b());

        Colour::new(r, g, b)
    }

    fn correct_component(component: f64) -> f64 {
        if component > 0.0 {
            f64::sqrt(component)
        } else {
            0.0
        }
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Colour::from(self.data + other.data)
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.data += rhs.data
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, other: f64) -> Colour {
        Colour::from(self.data * other)
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, other: Colour) -> Colour {
        Colour::from(other.data * self)
    }
}

impl Mul for Colour {
    type Output = Colour;

    fn mul(self, other: Colour) -> Colour {
        Colour::from(self.data * other.data)
    }
}

impl Div<f64> for Colour {
    type Output = Colour;

    fn div(self, rhs: f64) -> Self::Output {
        Colour::from(self.data / rhs)
    }
}

impl From<Vec3> for Colour {
    fn from(data: Vec3) -> Colour {
        Colour { data }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let corrected = self.gamma_corrected();

        let rbyte: i32 = (256.0 * corrected.r().clamp(0.0, 0.999)) as i32;
        let gbyte: i32 = (256.0 * corrected.g().clamp(0.0, 0.999)) as i32;
        let bbyte: i32 = (256.0 * corrected.b().clamp(0.0, 0.999)) as i32;

        write!(f, "{} {} {}\n", rbyte, gbyte, bbyte)
    }
}

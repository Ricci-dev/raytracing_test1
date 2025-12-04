use std::ops::{Add, Div, Mul, Sub};

use crate::{point3::Point3, rtweekend::{get_random_f64, get_random_f64_range}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(f64, f64, f64);

pub trait Vec3Trait {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

    fn length_squared(&self) -> f64 {
        self.x()*self.x() + self.y()*self.y() + self.z()*self.z()
    }

    fn len(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn dot(&self, other: impl Vec3Trait) -> f64 {
        self.x()*other.x() + self.y()*other.y() + self.z()*other.z()
    }

    fn vec3(&self) -> Vec3 {
        Vec3::new(self.x(), self.y(), self.z())
    }

    fn point3(&self) -> Point3 {
        Point3::new(self.x(), self.y(), self.z())
    }
}
impl Vec3Trait for Vec3{
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn nowhere() -> Self {
        Self(0., 0., 0.)
    }

    pub fn cross(&self, other: impl Vec3Trait) -> Self {
        Self(
            self.y()*other.z() - self.z()*other.y(),
            self.z()*other.x() - self.x()*other.z(),
            self.x()*other.y() - self.y()*other.x(),
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Vec3::random_range(-1., 1.);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1. {return p / lensq.sqrt();}
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        // Resultierenden Vektor begrenzen, damit winkel zur normale nicht größer als 90° in beide Richtungen ist
        if on_unit_sphere.dot(normal) > 0. {on_unit_sphere} else {on_unit_sphere*(-1.)}
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n*v.dot(n)*2.
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min((uv*(-1.)).dot(n), 1.0);
        let r_out_perp = (uv + n*cos_theta) * etai_over_etat;
        let r_out_parallel = n * (1. - r_out_perp.length_squared()).abs().sqrt() * (-1.);
        r_out_perp + r_out_parallel
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn vec3(self) -> Self {
        self
    }

    pub fn random() -> Self {
        Self::new(get_random_f64(), get_random_f64(), get_random_f64())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(get_random_f64_range(min, max), get_random_f64_range(min, max), get_random_f64_range(min, max))
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(get_random_f64_range(-1., 1.), get_random_f64_range(-1., 1.), 0.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }
}

impl Add<Point3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Point3) -> Vec3 {
        Self(
            self.0 + other.x(),
            self.1 + other.y(),
            self.2 + other.z(),
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Self(
            self.0 + other.x(),
            self.1 + other.y(),
            self.2 + other.z(),
        )
    }
}

impl Sub<Point3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Point3) -> Vec3 {
        Self(
            self.0 - other.x(),
            self.1 - other.y(),
            self.2 - other.z(),
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let self_f = self as f64;
        Vec3(
            other.0 * self_f,
            other.1 * self_f,
            other.2 * self_f,
        )
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: i32) -> Vec3 {
        let other = other as f64;
        Self(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(
            other.0 * self,
            other.1 * self,
            other.2 * self,
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Self(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        )
    }
}

impl Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: i32) -> Vec3 {
        let other = other as f64;
        Self(
            self.0 / other,
            self.1 / other,
            self.2 / other,
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Self(
            self.0 / other,
            self.1 / other,
            self.2 / other,
        )
    }
}

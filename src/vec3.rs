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
        if on_unit_sphere.dot(normal) > 0. {on_unit_sphere} else {on_unit_sphere*(-1.)}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let r1 = Vec3::new(1.0, 2.0, 3.0);
        let r2 = Vec3::new(10.0, 20.0, 30.0);
        let r3 = Vec3::new(11.0, 22.0, 33.0);
        assert_eq!(r1 + r2, r3);
    }

    #[test]
    fn test_subtraction() {
        let r1 = Vec3::new(5.0, 2.0, 90.0);
        let r2 = Vec3::new(3.0, 10.0, 30.0);
        let r3 = Vec3::new(2.0, -8.0, 60.0);
        assert_eq!(r1 - r2, r3);
    }

    #[test]
    fn test_multiplication() {
        let r1 = Vec3::new(1.0, 2.0, -3.0);
        let r2 = -5.0;
        let r3 = Vec3::new(-5.0, -10.0, 15.0);
        assert_eq!(r1 * r2, r3);
    }

    #[test]
    fn test_division() {
        let r1 = Vec3::new(15.0, -30.0, 3.0);
        let r2 = -5.0;
        let r3 = Vec3::new(-3.0, 6.0, -0.6);
        assert_eq!(r1 / r2, r3);
    }

    #[test]
    fn test_dot_product() {
        let r1 = Vec3::new(3.0, -4.0, 3.0);
        let r2 = Vec3::new(15.0, -30.0, 3.0);
        let r3 = 174.0;
        assert_eq!(r1.dot(r2), r3);
    }

    #[test]
    fn test_cross_product() {
        let r1 = Vec3::new(1.0, 2.0, 3.0);
        let r2 = Vec3::new(10.0, 20.0, 30.0);
        let r3 = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(r1.cross(r2), r3);
    }

    #[test]
    fn test_length_squared() {
        let r1 = Vec3::new(1.0, 2.0, 3.0);
        let r2 = 14.0;
        assert_eq!(r1.length_squared(), r2);
    }

    #[test]
    fn test_length() {
        let r1 = Vec3::new(1.0, 2.0, 3.0);
        let r2 = 374.0;
        assert_eq!((r1.len()*100.0).round(), r2);
    }

    #[test]
    fn test_unit_vector() {
        let r1 = Vec3::new(5.4, 56.54, 3.0);
        assert_eq!(r1.unit_vector().len(), 1.0);
    }
}

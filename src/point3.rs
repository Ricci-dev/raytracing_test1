use std::ops::{Add, Div, Mul, Sub};

use crate::vec3::{Vec3, Vec3Trait};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3(f64, f64, f64);

impl Vec3Trait for Point3 {
    fn x(&self) -> f64 {self.0}
    fn y(&self) -> f64 {self.1}
    fn z(&self) -> f64 {self.2}
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3(x, y, z)
    }

    pub fn cross(&self, other: impl Vec3Trait) -> Self {
        Self(
            self.y()*other.z() - self.z()*other.y(),
            self.z()*other.x() - self.x()*other.z(),
            self.x()*other.y() - self.y()*other.x(),
        )
    }

    pub fn unit_vector(&self) -> Point3 {
        *self / self.len()
    }

    pub fn point3(self) -> Point3 {
        self
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, other: Vec3) -> Point3 {
        Self(
            self.0 + other.x(),
            self.1 + other.y(),
            self.2 + other.z(),
        )
    }
}

impl Add for Point3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, other: Vec3) -> Point3 {
        Self(
            self.0 - other.x(),
            self.1 - other.y(),
            self.2 - other.z(),
        )
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, other: Point3) -> Vec3 {
        Vec3::new(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
    }
}

impl Mul<i32> for Point3 {
    type Output = Point3;

    fn mul(self, other: i32) -> Point3 {
        let other = other as f64;
        Self(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        )
    }
}

impl Mul<f64> for Point3 {
    type Output = Point3;

    fn mul(self, other: f64) -> Point3 {
        Self(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        )
    }
}

impl Div<i32> for Point3 {
    type Output = Point3;

    fn div(self, other: i32) -> Point3 {
        let other = other as f64;
        Self(
            self.0 / other,
            self.1 / other,
            self.2 / other,
        )
    }
}

impl Div<f64> for Point3 {
    type Output = Point3;

    fn div(self, other: f64) -> Point3 {
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
        let r1 = Point3::new(1.0, 2.0, 3.0);
        let r2 = Point3::new(10.0, 20.0, 30.0);
        let r3 = Point3::new(11.0, 22.0, 33.0);
        assert_eq!(r1 + r2, r3);
    }

    #[test]
    fn test_subtraction() {
        let r1 = Point3::new(5.0, 2.0, 90.0);
        let r2 = Point3::new(3.0, 10.0, 30.0);
        let r3 = Vec3::new(2.0, -8.0, 60.0);
        assert_eq!(r1 - r2, r3);
    }

    #[test]
    fn test_multiplication() {
        let r1 = Point3::new(1.0, 2.0, -3.0);
        let r2 = -5.0;
        let r3 = Point3::new(-5.0, -10.0, 15.0);
        assert_eq!(r1 * r2, r3);
    }

    #[test]
    fn test_division() {
        let r1 = Point3::new(15.0, -30.0, 3.0);
        let r2 = -5.0;
        let r3 = Point3::new(-3.0, 6.0, -0.6);
        assert_eq!(r1 / r2, r3);
    }

    #[test]
    fn test_dot_product() {
        let r1 = Point3::new(3.0, -4.0, 3.0);
        let r2 = Point3::new(15.0, -30.0, 3.0);
        let r3 = 174.0;
        assert_eq!(r1.dot(r2), r3);
    }

    #[test]
    fn test_cross_product() {
        let r1 = Point3::new(1.0, 2.0, 3.0);
        let r2 = Point3::new(10.0, 20.0, 30.0);
        let r3 = Point3::new(0.0, 0.0, 0.0);
        assert_eq!(r1.cross(r2), r3);
    }

    #[test]
    fn test_length_squared() {
        let r1 = Point3::new(1.0, 2.0, 3.0);
        let r2 = 14.0;
        assert_eq!(r1.length_squared(), r2);
    }

    #[test]
    fn test_length() {
        let r1 = Point3::new(1.0, 2.0, 3.0);
        let r2 = 374.0;
        assert_eq!((r1.len()*100.0).round(), r2);
    }

    #[test]
    fn test_unit_vector() {
        let r1 = Point3::new(5.4, 56.54, 3.0);
        assert_eq!(r1.unit_vector().len(), 1.0);
    }
}

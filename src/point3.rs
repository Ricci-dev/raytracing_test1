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

    pub fn origin() -> Point3 {
        Point3(0., 0., 0.)
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

impl Mul<Point3> for i32 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        let self_f = self as f64;
        Point3(
            other.0 * self_f,
            other.1 * self_f,
            other.2 * self_f,
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

impl Mul<Point3> for f64 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        Point3(
            other.0 * self,
            other.1 * self,
            other.2 * self,
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

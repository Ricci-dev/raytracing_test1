use std::ops::{Add, Div, Mul, Sub};

use crate::{interval::Interval, vec3::{Vec3, Vec3Trait}};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(f64, f64, f64);

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {return linear_component.sqrt();}
    0.
}

pub fn write_color(pixel_color: Color) {
    let mut r = pixel_color.r();
    let mut g = pixel_color.g();
    let mut b = pixel_color.b();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256. * intensity.clamp(r)) as i32;
    let gbyte = (256. * intensity.clamp(g)) as i32;
    let bbyte = (256. * intensity.clamp(b)) as i32;

    println!("{rbyte} {gbyte} {bbyte}");
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(r, g, b)
    }


    pub fn cross(&self, other: Color) -> Color {
        Self(
            self.1*other.2 - self.2*other.1,
            self.2*other.0 - self.0*other.2,
            self.0*other.1 - self.1*other.0,
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn len(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Color {
        *self / self.len()
    }

    pub fn dot(&self, other: Color) -> f64 {
        self.0*other.0 + self.1*other.1 + self.2*other.2
    }


    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }


    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
    }
}

impl Add<Vec3> for Color {
    type Output = Color;

    fn add(self, other: Vec3) -> Color {
        Self(
            self.0 + other.x(),
            self.1 + other.y(),
            self.2 + other.z(),
        )
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
    }
}

impl Sub<Vec3> for Color {
    type Output = Color;

    fn sub(self, other: Vec3) -> Color {
        Self(
            self.0 - other.x(),
            self.1 - other.y(),
            self.2 - other.z(),
        )
    }
}

impl Mul<i32> for Color {
    type Output = Color;

    fn mul(self, other: i32) -> Color {
        let other = other as f64;
        Self(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Self(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        )
    }
}

impl Div<i32> for Color {
    type Output = Color;

    fn div(self, other: i32) -> Color {
        let other = other as f64;
        Self(
            self.0 / other,
            self.1 / other,
            self.2 / other,
        )
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Color {
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
        let r1 = Color::new(1.0, 2.0, 3.0);
        let r2 = Color::new(10.0, 20.0, 30.0);
        let r3 = Color::new(11.0, 22.0, 33.0);
        assert_eq!(r1 + r2, r3);
    }

    #[test]
    fn test_subtraction() {
        let r1 = Color::new(5.0, 2.0, 90.0);
        let r2 = Color::new(3.0, 10.0, 30.0);
        let r3 = Color::new(2.0, -8.0, 60.0);
        assert_eq!(r1 - r2, r3);
    }

    #[test]
    fn test_multiplication() {
        let r1 = Color::new(1.0, 2.0, -3.0);
        let r2 = -5.0;
        let r3 = Color::new(-5.0, -10.0, 15.0);
        assert_eq!(r1 * r2, r3);
    }

    #[test]
    fn test_division() {
        let r1 = Color::new(15.0, -30.0, 3.0);
        let r2 = -5.0;
        let r3 = Color::new(-3.0, 6.0, -0.6);
        assert_eq!(r1 / r2, r3);
    }

    #[test]
    fn test_dot_product() {
        let r1 = Color::new(3.0, -4.0, 3.0);
        let r2 = Color::new(15.0, -30.0, 3.0);
        let r3 = 174.0;
        assert_eq!(r1.dot(r2), r3);
    }

    #[test]
    fn test_cross_product() {
        let r1 = Color::new(1.0, 2.0, 3.0);
        let r2 = Color::new(10.0, 20.0, 30.0);
        let r3 = Color::new(0.0, 0.0, 0.0);
        assert_eq!(r1.cross(r2), r3);
    }

    #[test]
    fn test_length_squared() {
        let r1 = Color::new(1.0, 2.0, 3.0);
        let r2 = 14.0;
        assert_eq!(r1.length_squared(), r2);
    }

    #[test]
    fn test_length() {
        let r1 = Color::new(1.0, 2.0, 3.0);
        let r2 = 374.0;
        assert_eq!((r1.len()*100.0).round(), r2);
    }

    #[test]
    fn test_unit_vector() {
        let r1 = Color::new(5.4, 56.54, 3.0);
        assert_eq!(r1.unit_vector().len(), 1.0);
    }
}

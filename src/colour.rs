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

    pub fn black() -> Color {
        Color(0., 0., 0.)
    }

    pub fn white() -> Color {
        Color(1., 1., 1.)
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

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Self(
            self.0 * other.r(),
            self.1 * other.g(),
            self.2 * other.b(),
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

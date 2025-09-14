use crate::{point3::Point3, vec3::Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn ray(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn placeholder() -> Ray {
        Ray {
            origin: Point3::origin(),
            direction: Vec3::nowhere(),
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + self.direction*t;
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

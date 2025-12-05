use crate::{point3::Point3, vec3::Vec3};

/// straigt line (Gerade) in R3
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    tm:f64,
}

impl Ray {
    /// Constructor
    pub fn ray(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            tm: 0.,
        }
    }

    pub fn rayt(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            tm: time,
        }
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    /// Placeholder ray (origin, no direction)
    pub fn placeholder() -> Ray {
        Ray {
            origin: Point3::origin(),
            direction: Vec3::nowhere(),
            tm: 0.,
        }
    }

    /// Point on the straight line
    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + self.direction*t;
    }

    /// GET the origin of the straigt line
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// GET the direction vector of the straight line
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

use crate::rtweekend::INFINITY;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    /// In empty passt nichts rein, weil das minimum unendlich groß ist und das maximum unendlich klein
    pub fn empty() -> Interval {
        Interval { min: INFINITY, max: -INFINITY }
    }

    /// In universe passt alles rein
    pub fn universe() -> Interval {
        Interval { min: -INFINITY, max: INFINITY }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
       self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {self.min}
        else if x > self.max {self.max}
        else {x}
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}


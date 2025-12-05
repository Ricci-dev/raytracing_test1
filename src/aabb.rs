use crate::{interval::{self, Interval}, point3::Point3, ray::Ray, vec3::Vec3Trait};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn empty() -> AABB {
        AABB { x: Interval::empty(), y: Interval::empty(), z: Interval::empty() }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> AABB {
        AABB { x, y, z }
    }

    pub fn newps(a: Point3, b: Point3) -> AABB {
        AABB {
            x: if a.x() <= b.x() {Interval::new(a.x(), b.x())} else {Interval::new(b.x(), a.x())},
            y: if a.y() <= b.y() {Interval::new(a.y(), b.y())} else {Interval::new(b.y(), a.y())},
            z: if a.z() <= b.z() {Interval::new(a.z(), b.z())} else {Interval::new(b.z(), a.z())},
        }
    }

    pub fn newbb(box0: AABB, box1: AABB) -> AABB {
        let x = Interval::newii(box0.x(), box1.x());
        let y = Interval::newii(box0.y(), box1.y());
        let z = Interval::newii(box0.z(), box1.z());
        AABB { x, y, z }
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        if n==1 {return self.y()}
        if n==2 {return self.z()}
        self.x()
    }

    pub fn hit(&self, r: Ray, ray_t: &mut Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..2 {
            let ax = self.axis_interval(axis);
            let adinv = 1./ray_dir.axis(axis);

            let t0 = (ax.min() - ray_orig.axis(axis)) * adinv;
            let t1 = (ax.max() - ray_orig.axis(axis)) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min() {ray_t.set_min(t0);}
                if t1 > ray_t.max() {ray_t.set_max(t1);}
            } else {
                if t1 > ray_t.min() {ray_t.set_min(t1);}
                if t0 > ray_t.max() {ray_t.set_max(t0);}
            }
            if ray_t.max() <= ray_t.min() {return false}
        }
        true
    }

    pub fn x(&self) -> Interval {self.x}
    pub fn y(&self) -> Interval {self.y}
    pub fn z(&self) -> Interval {self.z}
}
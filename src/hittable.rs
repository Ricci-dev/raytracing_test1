use crate::{interval::Interval, point3::Point3, ray::Ray, vec3::{Vec3, Vec3Trait}};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn placeholder() -> HitRecord {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3){
        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {outward_normal} else {outward_normal * (-1)};
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

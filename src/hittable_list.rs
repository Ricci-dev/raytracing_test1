use crate::{hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray};

pub struct HittableList<T: Hittable> {
    objects: Vec<Box<T>>,
}

impl<T> HittableList<T> where T: Hittable {
    pub fn new() -> HittableList<T> {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min(), closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}

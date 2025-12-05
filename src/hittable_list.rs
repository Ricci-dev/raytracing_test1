use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![], bbox: AABB::empty() }
    }

    // Static bad? - RAM filling when creating and removing objects?
    // Answer?: Fat pointer, static != static, static = ?, so its fine
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.bbox = AABB::newbb(self.bbox, object.bounding_box());
        self.objects.push(Box::new(object));
    }

    pub fn bounding_box(&self) -> AABB {
        self.bbox
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // TODO: calc only hitrecord of closest, w `get_hit_record` method?
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

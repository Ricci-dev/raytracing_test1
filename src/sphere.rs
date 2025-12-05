use std::rc::Rc;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, interval::Interval, material::Material, point3::Point3, ray::Ray, vec3::{Vec3, Vec3Trait}};

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {center: Ray::ray(static_center, Vec3::nowhere()), radius: f64::max(0., radius), mat, bbox: AABB::newps(static_center - rvec, static_center + rvec)}
    }

    pub fn newm(center1: Point3, center2: Point3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        let center = Ray::ray(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::newps(center.at(0.) - rvec, center.at(0.) + rvec);
        let box2 = AABB::newps(center.at(1.) - rvec, center.at(1.) + rvec);
        let bbox = AABB::newbb(box1, box2);
        Sphere {center, radius: f64::max(0., radius), mat, bbox}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        // calculate for what t does the ray hit the sphere
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius*self.radius;
        
        let discriminant = h*h - a*c;
        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // minus will produce smaller value than plus
        // => nearer hit point is checked first
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        // outsource to a `get_hit_rec` func?
        rec.t = root;
        // ray intersection point
        rec.p = r.at(rec.t);
        // this is a surface normal (perpendicular to the surface at the intersection point)
        // this vector is also a unit vector (len: 1)
        let outward_normal = (rec.p - current_center) / self.radius;
        // make sure surface normal points outwards of object
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
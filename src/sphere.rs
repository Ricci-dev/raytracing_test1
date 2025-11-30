use std::rc::Rc;

use crate::{hittable::{HitRecord, Hittable}, interval::Interval, material::Material, point3::Point3, ray::Ray, vec3::Vec3Trait};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        Sphere {center, radius: f64::max(0., radius), mat}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // calculate for what t does the ray hit the sphere
        let oc = self.center - r.origin();
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
        let outward_normal = (rec.p - self.center) / self.radius;
        // make sure surface normal points outwards of object
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}
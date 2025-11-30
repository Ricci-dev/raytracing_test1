use std::rc::Rc;

use crate::{interval::Interval, material::{Material, NoMat}, point3::Point3, ray::Ray, vec3::{Vec3, Vec3Trait}};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn placeholder() -> HitRecord {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            mat: Rc::new(NoMat{}),
            t: 0.,
            front_face: false,
        }
    }

    /// Accepts a `ray` and the `outward_normal` (unit vector)
    /// 
    /// It sets the `HitRecord.normal` to the `outward_normal`, but with the right direction
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3){
        /*
        check if vector points outwards
        w cos and dot product of ray direction and the perpendicular surface normal
        we can check, if the normal points outwards or inwards
        dotproduct with cos can be used to get the angle between two vectors
        between 0 and 90 degree its positive/0 and between 270 and 360 degree its also positive/0
        between 90+ and 270- degree its negative
        the normal is 90 degree to the surface
        ray comes from the outside and surface normal starts from the surface
        if both originate from the surface, then ray direction points inside
        and if normal faces inside, then ray direction is max 90 degree to the left or right to the normal
        this means the res is higher than 0 -> flip direction of surface normal (180 degree rot) by multiplying with -1

        if ray hits from inside, then its different (?)

        surface normal wird verwendet um herauszufinden, ob der strahl von innen/außen kommt

        does surface normal always point against direction of ray? and outward normal always out?

        above only math kinda correct
        right explenation: outward_normal always looks outwards
        depending on where the ray is coming from (outside/inside) the normal will be set oposite to ray direction
        => when ray comes from inside, normal points inside
        => when ray comes from outside, normal points outside
        */
        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {outward_normal} else {outward_normal * (-1)};
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

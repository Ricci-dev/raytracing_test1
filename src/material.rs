use std::rc::Rc;

use crate::{colour::Color, hittable::HitRecord, ray::Ray, vec3::{Vec3, Vec3Trait}};

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (bool, Color, Ray);
}

pub struct NoMat {}
impl Material for NoMat {fn scatter(&self, _r_in: Ray, _rec: &HitRecord) -> (bool, Color, Ray) {(false, Color::black(), Ray::placeholder())}}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Rc<Self> {
        Rc::new(Self {
            albedo
        })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {scatter_direction = rec.normal}

        (true, self.albedo, Ray::ray(rec.p, scatter_direction))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Rc<Self> {
        Rc::new(Self {
            albedo,
            fuzz: if fuzz < 1. {fuzz} else {1.},
        })
    }
}

impl Material for Metal {
    /// Returns (bool, attenuation, scattered)
    /// 
    ///  * `attenuation` - ?
    ///  * `scattered` - ?
    /// ```
    /// use raytracer::{material::{NoMat, Material}, ray::Ray, hittable::HitRecord};
    /// let mat = NoMat{};
    /// let (idk, attenuation, scattered) = mat.scatter(Ray::placeholder(), &HitRecord::placeholder());
    /// ```
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut reflected = Vec3::reflect(r_in.direction(), rec.normal);
        reflected = reflected.unit_vector() + (Vec3::random_unit_vector() * self.fuzz);
        let scattered = Ray::ray(rec.p, reflected);
        (scattered.direction().dot(rec.normal) > 0., self.albedo, scattered)
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Rc<Self> {
        Rc::new(Self { refraction_index })
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let ri = if rec.front_face {1. / self.refraction_index} else {self.refraction_index};

        let unit_direction = r_in.direction().unit_vector();
        let refracted = Vec3::refract(unit_direction, rec.normal, ri);
        (true, Color::white(), Ray::ray(rec.p, refracted))
    }
}

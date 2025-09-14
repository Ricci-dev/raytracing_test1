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
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo
        }
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
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. {fuzz} else {1.},
        }
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

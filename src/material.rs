use crate::{colour::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

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
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Metal {
    /// Returns (bool, attenuation, scattered)
    /// 
    /// Test
    /// 
    ///  * `attenuation` - ?
    ///  * `scattered` - ?
    /// ```
    /// let mat = NoMat{};
    /// let (idk, attenuation, scattered) = mat.scatter(Ray::placeholder(), HitRecord::placeholder());
    /// ```
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let reflected = Vec3::reflect(r_in.direction(), rec.normal);
        (true, self.albedo, Ray::ray(rec.p, reflected))
    }
}

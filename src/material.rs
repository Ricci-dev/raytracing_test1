use std::rc::Rc;

use crate::{colour::Color, hittable::HitRecord, ray::Ray, rtweekend::get_random_f64, vec3::{Vec3, Vec3Trait}};

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

        // scatter, attenuation, scatter_direction
        (true, self.albedo, Ray::rayt(rec.p, scatter_direction, _r_in.time()))
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
    ///  * `reflect` - If the light should be reflected (true) or absorbed (false)
    ///  * `attenuation` - ?
    ///  * `scattered` - the scattered ray (?)
    /// ```
    /// use raytracer::{material::{NoMat, Material}, ray::Ray, hittable::HitRecord};
    /// let mat = NoMat{};
    /// let (reflect, attenuation, scattered) = mat.scatter(Ray::placeholder(), &HitRecord::placeholder());
    /// ```
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut reflected = Vec3::reflect(r_in.direction(), rec.normal);
        reflected = reflected.unit_vector() + (Vec3::random_unit_vector() * self.fuzz);
        let scattered = Ray::rayt(rec.p, reflected, r_in.time());
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

    pub fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1. - refraction_index) / (1. + refraction_index);
        r0 = r0*r0;
        r0 + (1.-r0)*f64::powi(1. - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        // TODO: look at Snell's Law
        // Nur von Luft in anders, bzw. anderes in Luft akkurat (einfachheit?)
        // Solution: track current refraction index
        // `ri` = refraction index old/new (old = Air => 1/new;             new = Air => old/1 => old)
        let ri = if rec.front_face {1. / self.refraction_index} else {self.refraction_index};

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = (unit_direction*(-1.)).dot(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();

        let cannot_refract = ri*sin_theta > 1.;
        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > get_random_f64() {Vec3::reflect(unit_direction, rec.normal)}
                                else {Vec3::refract(unit_direction, rec.normal, ri)};

        (true, Color::white(), Ray::rayt(rec.p, direction, r_in.time()))
    }
}

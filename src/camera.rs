use std::io::{stderr, Write};

use crate::{colour::{write_color, Color}, hittable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, point3::Point3, ray::Ray, rtweekend::{get_random_f64, INFINITY}, vec3::{Vec3, Vec3Trait}};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_depth: i32) -> Camera {
        let origin = Point3::new(0., 0., 0.);
        Camera { aspect_ratio, image_width, samples_per_pixel, max_depth, image_height: 0, pixel_samples_scale: 0., center: origin.clone(), pixel00_loc: origin.clone(), pixel_delta_u:origin.vec3(), pixel_delta_v: origin.vec3() }
    }

    pub fn render<T: Hittable>(&mut self, world: &HittableList<T>) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..(self.image_height) {
            eprint!("\rScanlines remaining: {} ", self.image_height-j);
            let _ = stderr().flush();
            for i in 0..(self.image_width) {
                /*
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);
                // NOTE - AUTO VEC3 Conversion becuase point3 - point3 => vec3 now (if changed back, use point3.vec3() (TODO: Replace with as trait?))
                let ray_direction = pixel_center - self.center;
                let r = Ray::ray(self.center, ray_direction);

                let pixel_color = self.ray_color(r, &world);
                */
                let mut pixel_color = Color::new(0., 0., 0.);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(r, self.max_depth, &world);
                }
                write_color(pixel_color * self.pixel_samples_scale);
            }
        }
        eprintln!("\rDone                               ");
    }

    fn initialize(&mut self) {
        // width / height = ratio => width / ratio = height
        // let aspect_ratio = 16.0 / 9.0;
        // let image_width = 400;
        // let image_width = 3840;
        // let image_width = 4096;
        // let image_width = 1920;

        // calc height based on aspect ratio and image width
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        // Make sure height is at least one
        self.image_height = if self.image_height < 1 {1} else {self.image_height};

        self.pixel_samples_scale = 1. / self.samples_per_pixel as f64;

        // Distance viewport <=> camera center
        let focal_length = 1.0;
        let viewport_height = 2.0;
        // calc actual ratio: i_width / i_height
        // calc viewport_wdith: v_width / v_height = a_ratio => a_ratio * v_height = v_width
        let viewport_width = viewport_height * ((self.image_width as f64)/(self.image_height) as f64);
        self.center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u/2) - (viewport_v/2);
        self.pixel00_loc = viewport_upper_left + ((self.pixel_delta_u + self.pixel_delta_v)*0.5);
        eprintln!("Image, width: {}, height; {}", self.image_width, self.image_height);
        eprintln!("Viewport, width: {}, height; {}", viewport_width, viewport_height);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
                                    + (self.pixel_delta_u * (offset.x() + i as f64))
                                    + (self.pixel_delta_v * (offset.y() + j as f64));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::ray(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(get_random_f64() - 0.5, get_random_f64() - 0.5, 0.)
    }

    fn ray_color<T: Hittable>(&self, r: Ray, depth: i32, world: &HittableList<T>) -> Color {
        if depth <= 0 {return Color::new(0., 0., 0.)}

        let mut rec = HitRecord::placeholder();
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            // let direction = Vec3::random_on_hemisphere(rec.normal);
            let direction = rec.normal + Vec3::random_unit_vector();
            return (self.ray_color(Ray::ray(rec.p, direction), depth-1, world))*0.5;
            // return (Color::new(1., 1., 1.) + rec.normal)*0.5;
        }

        // y ist kleiner, gleich 1; größtmöglicher Wert: 1, damit unit_vector Länge = 1 ist
        let unit_direction = r.direction().unit_vector();
        // effekt bei kleinen zahlen verstärken? (bei y=1 wird es wieder y=1; ansonsten auch immer kleiner 1)
        let a = (unit_direction.y() + 1.0)*0.5;
        // Das linke wird immer schwächer umso größer a, das rechte immer stärker umso größer a
        // links weiß, rechts helles blau
        // a verkleinert sich, dadurch weiß immer stärker, blau immer helles schwächer
        // Werte nehmen gleichmäßig ab und zu, dadurch Farbzahlen nie größer als 1
        Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a
    }

}

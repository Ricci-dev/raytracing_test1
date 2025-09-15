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
        let origin = Point3::origin();
        Camera {
            aspect_ratio, image_width, samples_per_pixel, max_depth,
            image_height: 0,
            pixel_samples_scale: 0.,
            center: origin.clone(),
            pixel00_loc: origin.clone(),
            pixel_delta_u:origin.vec3(),
            pixel_delta_v: origin.vec3()
        }
    }

    pub fn render<T: Hittable>(&mut self, world: &HittableList<T>) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..(self.image_height) {
            eprint!("\rScanlines remaining: {} ", self.image_height-j);
            let _ = stderr().flush();
            for i in 0..(self.image_width) {
                let mut pixel_color = Color::black();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    // Get the Color for the current pixel and add it to the pixel current color
                    // This is done multiple times, because the current pixel is either the real current pixel or a random neighbor next to it
                    pixel_color = pixel_color + self.ray_color(r, self.max_depth, &world);
                }
                // After adding the many colors of the real current pixel and its neighbors,
                // calculate the average color by multiplying the resulting color by the `samples_per_pixel**(-1)`
                // This has the same effect as dividing by `samples_per_pixel`
                // By using this technique called anti aliasing we can get cleaner object borders (?)
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
        let focal_length = 1.;
        let viewport_height = 2.;
        // calc actual ratio: i_width / i_height
        // calc viewport_wdith: v_width / v_height = a_ratio => a_ratio * v_height = v_width
        let viewport_width = viewport_height * ((self.image_width as f64)/(self.image_height) as f64);
        
        // Camera center is set in construcor - Duplicate?
        //self.center = Point3::new(0., 0., 0.);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        let viewport_upper_left = self.center - Vec3::new(0., 0., focal_length) - (viewport_u/2) - (viewport_v/2);
        self.pixel00_loc = viewport_upper_left + ((self.pixel_delta_u + self.pixel_delta_v)*0.5);
        eprintln!("Image, width: {}, height; {}", self.image_width, self.image_height);
        eprintln!("Viewport, width: {}, height; {}", viewport_width, viewport_height);
    }

    /// Get the Ray for the current pixel in the image
    /// 
    /// This function is called multiple times (`samples_per_pixel`) to get multiple Rays,
    /// because the randomly generated offset randomly gives back a ray to the current pixel or a random neighbor pixel
    /// 
    /// This method is called anti aliasing and helps get cleaner object borders (?)
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Offset is used to randomly get the current image pixel or a neighbor
        let offset = self.sample_square();
        // Calculate the pixel position in the current viewport (?)
        let pixel_sample = self.pixel00_loc
                                    + (self.pixel_delta_u * (offset.x() + i as f64))
                                    + (self.pixel_delta_v * (offset.y() + j as f64));
        // Create the ray from camera to the chosen pixel
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::ray(ray_origin, ray_direction)
    }

    /// Generate a random Vector for a pixel that is next to the current pixel
    fn sample_square(&self) -> Vec3 {
        Vec3::new(get_random_f64() - 0.5, get_random_f64() - 0.5, 0.)
    }

    fn ray_color<T: Hittable>(&self, r: Ray, depth: i32, world: &HittableList<T>) -> Color {
        // Stop recursion after max `depth` recusions
        if depth <= 0 {return Color::new(0., 0., 0.)}

        // The `HitRecord` contains the Point that got hit and its Material
        // `rec.t` is used to determin the distance from the camera to the Point that got hit
        // The surface normal `rec.normal` is a unit vector, perpendicular to the surface at the Point that got hit
        // Surface normal purpose?
        let mut rec = HitRecord::placeholder();
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let (scatter_res, attenuation, scattered) = rec.mat.scatter(r, &rec);
            if scatter_res {
                return self.ray_color(scattered, depth-1, world) * attenuation;
            }
            return Color::black();
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

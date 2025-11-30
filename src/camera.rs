use std::io::{stderr, Write};

use crate::{colour::{Color, write_color}, hittable::HitRecord, hittable_list::HittableList, interval::Interval, point3::Point3, ray::Ray, rtweekend::{INFINITY, degrees_to_radians, get_random_f64}, vec3::{Vec3, Vec3Trait}};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    pub vfov: f64, // vertical view angle
    pub lookfrom: Point3, // Pos where cam is looking from
    pub lookat: Point3, // Pos where cam is looking at
    pub vup: Vec3, // Camera-relative up direction

    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // Camera frame basis vectors
    v: Vec3,
    u: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_depth: i32, vfov: f64, lookfrom: Point3, lookat: Point3, vup: Vec3) -> Camera {
        Camera {
            aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup,
            // everything zero, values are set later in init
            image_height: 0,
            pixel_samples_scale: 0.,
            center: lookfrom,
            pixel00_loc: Point3::origin(),
            pixel_delta_u: Vec3::nowhere(),
            pixel_delta_v: Vec3::nowhere(),
            v: Vec3::nowhere(),
            u: Vec3::nowhere(),
            w: Vec3::nowhere(),
        }
    }

    // TODO: replace `HittableList` with `Hittable` trait
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        // ppm format header (or smth)/image info
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
        // let aspect_ratio = 16.0 / 9.0;
        // let image_width = 400;
        // let image_width = 3840;
        // let image_width = 4096;
        // let image_width = 1920;

        // calc height based on aspect ratio and image width
        // ratio = width / height => height = width / ratio
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        // Make sure height is at least one
        self.image_height = if self.image_height < 1 {1} else {self.image_height};

        self.pixel_samples_scale = 1. / self.samples_per_pixel as f64;

        // Distance viewport <=> camera center
        let focal_length = (self.lookfrom - self.lookat).len();
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta/2.);
        // viewport is a virtual rectanglein R3, with the area, we are currently watching (?)
        // We shoot our rays towards this rectangle
        // let viewport_height = 2.;
        let viewport_height = 2. * h * focal_length;
        // calc actual ratio, to be more accurate (there are no 0.5 or 0.3532 pixel, that's why set ratio could be inacurate): i_width / i_height
        // calc viewport_wdith: v_width / v_height = a_ratio => a_ratio * v_height = v_width
        let viewport_width = viewport_height * ((self.image_width as f64)/(self.image_height) as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(self.w).unit_vector();
        self.v = self.w.cross(self.u);
        
        // Camera center is set in construcor - Duplicate?
        //self.center = Point3::new(0., 0., 0.);

        // Vector from left side of viewport to right side
        let viewport_u = viewport_width * self.u;
        // Vector from top of viewport to bottom
        let viewport_v = viewport_height * -1. * self.v;

        // distance between pixels in viewport side
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        // distance between pixels in viewport top/bottom
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // position of the upper left corner of the viewport (also determines general position of the viewport)
        // TODO: describe effect of focal len better
        // let viewport_upper_left = self.center - Vec3::new(0., 0., focal_length) - (viewport_u/2) - (viewport_v/2);
        let viewport_upper_left = self.center - focal_length*self.w - (viewport_u/2) - (viewport_v/2);
        // position of the pixel in the upper left corner of the viewport
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
        let mut offset = self.sample_square();
        // TODO: Is this a good idea???
        if self.samples_per_pixel == 1{
            offset = Vec3::nowhere();
        }
        // Calculate the pixel position in the current viewport (?)
        let pixel_sample = self.pixel00_loc
                                    + (self.pixel_delta_u * (offset.x() + i as f64))
                                    + (self.pixel_delta_v * (offset.y() + j as f64));
        // Create the ray from camera to the chosen pixel
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::ray(ray_origin, ray_direction)
    }

    /// Generate a random Vector for a pixels that is next to the current pixel
    fn sample_square(&self) -> Vec3 {
        Vec3::new(get_random_f64() - 0.5, get_random_f64() - 0.5, 0.)
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &HittableList) -> Color {
        // Stop recursion after max `depth` recusions (max ray bounces)
        if depth <= 0 {return Color::new(0., 0., 0.)}

        // The `HitRecord` contains the Point that got hit and its Material
        // `rec.t` is used to determin the distance from the camera to the Point that got hit
        // The surface normal `rec.normal` is a unit vector, perpendicular to the surface at the Point that got hit
        // The surface normal is used for comparison, to check if vectors starting at the hitpoint, point into the object or outwards (?)
        let mut rec = HitRecord::placeholder();
        // The Intervall is used to check, that objects are in front of the camera and to get the closest value (?)
        // Ray Casting is just shooting a ray out and see if it hits smth
        // lower Interval value similar to Blender `ClipStart` option ig (lowest val: 0.001m)
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let (scatter_res, attenuation, scattered) = rec.mat.scatter(r, &rec);
            // scattered or absorbed
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

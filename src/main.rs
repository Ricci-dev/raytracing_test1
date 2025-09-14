use raytracer::sphere::Sphere;
use raytracer::point3::{Point3};
use raytracer::hittable_list::HittableList;
use raytracer::camera::Camera;

fn main() {
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100.));

    let mut cam = Camera::new(
        16./9.,
        400,
        100,
        50
    );
    cam.render(&world);
}

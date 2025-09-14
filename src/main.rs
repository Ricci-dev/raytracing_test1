use std::rc::Rc;

use raytracer::colour::Color;
use raytracer::material::{Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::point3::{Point3};
use raytracer::hittable_list::HittableList;
use raytracer::camera::Camera;

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100., material_ground));
    world.add(Sphere::new(Point3::new(0., 0., -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.5, material_left));
    world.add(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right));

    let mut cam = Camera::new(
        16./9.,
        400,
        100,
        50
    );
    cam.render(&world);
}

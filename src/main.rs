use raytracer::colour::Color;
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::point3::{Point3};
use raytracer::hittable_list::HittableList;
use raytracer::camera::Camera;
use raytracer::vec3::Vec3;

fn main() {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1. / 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.);

    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100., material_ground));
    world.add(Sphere::new(Point3::new(0., 0., -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.5, material_left));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.4, material_bubble));
    world.add(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right));

    let mut cam = Camera::new(
        16./9.,
        400,
        100,
        50,
        90.,
        Point3::new(-2., 2., 1.),
        Point3::new(0., 0., -1.),
        Vec3::new(0., 1., 0.),
    );
    cam.render(&world);
}

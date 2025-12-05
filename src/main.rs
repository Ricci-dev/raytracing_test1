use std::rc::Rc;

use raytracer::colour::Color;
use raytracer::material::{Dielectric, Lambertian, Material, Metal, NoMat};
use raytracer::rtweekend::{get_random_f64, get_random_f64_range};
use raytracer::sphere::Sphere;
use raytracer::point3::{Point3};
use raytracer::hittable_list::HittableList;
use raytracer::camera::Camera;
use raytracer::vec3::{Vec3, Vec3Trait};

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

    let mut world2 = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world2.add(Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = get_random_f64();
            let center = Point3::new(a as f64 + 0.9*get_random_f64(), 0.2, b as f64 + 0.9*get_random_f64());
            if (center - Point3::new(4., 0.2, 0.)).len() > 0.9 {
                let mut sphere_material: Rc<dyn Material> = Rc::new(NoMat{});

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Lambertian::new(albedo);
                    let center2 = center + Vec3::new(0., get_random_f64_range(0., 0.5), 0.);
                    world2.add(Sphere::newm(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = get_random_f64_range(0., 0.5);
                    sphere_material = Metal::new(albedo, fuzz);
                    world2.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    sphere_material = Dielectric::new(1.5);
                    world2.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world2.add(Sphere::new(Point3::new(0., 1., 0.), 1., material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world2.add(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world2.add(Sphere::new(Point3::new(4., 1., 0.), 1., material3));

    let mut cam = Camera::new(
        16./9.,
        //400,
        800,
        200,
        50,
        20.,
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.), // Look up (or sideways if u change it, SHOULD NOT BE PARALLEL TO VIEW DIRECTION)
        0.6,
        10.
    );
    cam.render(&world2);
}

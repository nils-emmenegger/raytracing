mod camera;
mod hittable;
mod hittable_list;
mod material;
mod rtweekend;
mod sphere;

use camera::Camera;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use rtweekend::*;
use sphere::Sphere;

fn main() {
    let mut world: HittableList = Default::default();

    let ground_material = Rc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vector3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random().component_mul(&random());
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_in(0.5, 1.0);
                    let fuzz = random_double_in(0.0, 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };
                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam: Camera = Default::default();

    cam.image_width = 400;
    cam.image_height = 225;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Vector3::new(13.0, 2.0, 3.0);
    cam.lookat = Vector3::new(0.0, 0.0, 0.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

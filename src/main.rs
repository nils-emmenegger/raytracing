mod camera;
mod hittable;
mod hittable_list;
mod material;
mod rtweekend;
mod sphere;

use camera::CameraBuilder;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use rtweekend::*;
use sphere::Sphere;

fn main() {
    let mut world: HittableList = Default::default();

    let ground_material = Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
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
                let sphere_material: Arc<dyn Material + Send + Sync> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random().component_mul(&random());
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_in(0.5, 1.0);
                    let fuzz = random_double_in(0.0, 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let cam = CameraBuilder::default()
        .image_width(400)
        .image_height(225)
        .samples_per_pixel(10)
        .max_depth(50)
        .vfov(20.0)
        .lookfrom(Vector3::new(13.0, 2.0, 3.0))
        .lookat(Vector3::new(0.0, 0.0, 0.0))
        .vup(Vector3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();

    cam.multi_threaded_render(&world);
}

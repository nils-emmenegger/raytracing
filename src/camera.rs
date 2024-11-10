use crate::{hittable::Hittable, material::Scattering, rtweekend::*};

#[derive(Default)]
pub struct CameraBuilder {
    image_width: Option<i32>,
    image_height: Option<i32>,
    samples_per_pixel: Option<i32>,
    max_depth: Option<i32>,
    vfov: Option<f64>,
    lookfrom: Option<Vector3<f64>>,
    lookat: Option<Vector3<f64>>,
    vup: Option<Vector3<f64>>,
    defocus_angle: Option<f64>,
    focus_dist: Option<f64>,
}

macro_rules! impl_setter {
    ($struct_name:ident, $(($field:ident, $field_type:ty)),+) => {
        impl $struct_name {
            $(
                pub fn $field(&mut self, $field: $field_type) -> &mut Self {
                    self.$field = Some($field);
                    self
                }
            )+
        }
    };
}

impl_setter!(
    CameraBuilder,
    (image_width, i32),
    (image_height, i32),
    (samples_per_pixel, i32),
    (max_depth, i32),
    (vfov, f64),
    (lookfrom, Vector3<f64>),
    (lookat, Vector3<f64>),
    (vup, Vector3<f64>),
    (defocus_angle, f64),
    (focus_dist, f64)
);

impl CameraBuilder {
    pub fn build(&self) -> Camera {
        // Defaults
        let image_width = self.image_width.unwrap_or(100);
        let image_height = self.image_height.unwrap_or(100);
        let samples_per_pixel = self.samples_per_pixel.unwrap_or(10);
        let max_depth = self.max_depth.unwrap_or(10);
        let vfov = self.vfov.unwrap_or(90.0);
        let lookfrom = self.lookfrom.unwrap_or(Vector3::new(0.0, 0.0, 0.0));
        let lookat = self.lookat.unwrap_or(Vector3::new(0.0, 0.0, -1.0));
        let vup = self.vup.unwrap_or(Vector3::new(0.0, 1.0, 0.0));
        let defocus_angle = self.defocus_angle.unwrap_or(0.0);
        let focus_dist = self.focus_dist.unwrap_or(10.0);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = lookfrom;

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u).normalize();

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - focus_dist * w - 0.5 * (viewport_u + viewport_v);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            vfov,
            defocus_angle,
            focus_dist,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Camera {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    vfov: f64,
    defocus_angle: f64,
    focus_dist: f64,
    pixel_samples_scale: f64,
    center: Vector3<f64>,
    pixel00_loc: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    defocus_disk_u: Vector3<f64>,
    defocus_disk_v: Vector3<f64>,
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_colour = Vector3::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour += Camera::ray_colour(&r, self.max_depth, world);
                }
                write_colour(
                    std::io::stdout().lock(),
                    self.pixel_samples_scale * pixel_colour,
                );
            }
        }
        eprintln!("\rDone.                 ");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vector3<f64> {
        Vector3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vector3<f64> {
        let p = random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn ray_colour(r: &Ray, depth: i32, world: &dyn Hittable) -> Vector3<f64> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, 0.001..INFINITY) {
            if let Some(Scattering {
                scattered,
                attenuation,
            }) = rec.mat.scatter(r, &rec)
            {
                return attenuation.component_mul(&Self::ray_colour(&scattered, depth - 1, world));
            } else {
                return Vector3::new(0.0, 0.0, 0.0);
            }
        }

        let mut unit_direction = r.dir();
        unit_direction.normalize_mut();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
    }
}

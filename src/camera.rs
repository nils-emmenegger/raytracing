use crate::{
    hittable::{HitRecord, Hittable},
    rtweekend::*,
};

#[derive(Clone, Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Vector3<f64>,
    pub lookat: Vector3<f64>,
    pub vup: Vector3<f64>,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: i32,
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

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Vector3::new(0.0, 0.0, 0.0),
            lookat: Vector3::new(0.0, 0.0, -1.0),
            vup: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: Default::default(),
            pixel_samples_scale: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

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
                    &(self.pixel_samples_scale * &pixel_colour),
                );
            }
        }
        eprintln!("\rDone.                 ");
    }

    fn initialize(&mut self) {
        self.image_height =
            unsafe { (self.image_width as f64 / self.aspect_ratio).to_int_unchecked() };
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom.clone();

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = self.lookfrom - self.lookat;
        self.w.normalize_mut();
        self.u = self.vup.cross(&self.w);
        self.u.normalize_mut();
        self.v = self.w.cross(&self.u);
        self.v.normalize_mut();

        let viewport_u = viewport_width * &self.u;
        let viewport_v = viewport_height * &-&self.v;

        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        let viewport_upper_left =
            &(&self.center - &(self.focus_dist * &self.w)) - &(0.5 * &(&viewport_u + &viewport_v));
        self.pixel00_loc =
            &viewport_upper_left + &(0.5 * &(&self.pixel_delta_u + &self.pixel_delta_v));

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = &self.u * defocus_radius;
        self.defocus_disk_v = &self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = &pixel_sample - &ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square() -> Vector3<f64> {
        Vector3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vector3<f64> {
        let p = random_in_unit_disk();
        &(&self.center + &(p[0] * &self.defocus_disk_u)) + &(p[1] * &self.defocus_disk_v)
    }

    fn ray_colour(r: &Ray, depth: i32, world: &dyn Hittable) -> Vector3<f64> {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered: Ray = Default::default();
            let mut attenuation: Vector3<f64> = Default::default();
            if rec
                .mat
                .as_ref()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation.component_mul(&Self::ray_colour(&scattered, depth - 1, world));
            } else {
                return Vector3::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = r.dir().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        &((1.0 - a) * &Vector3::new(1.0, 1.0, 1.0)) + &(a * &Vector3::new(0.5, 0.7, 1.0))
    }
}

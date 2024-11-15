use crate::{hittable::HitRecord, rtweekend::*};

pub struct Scattering {
    pub scattered: Ray,
    pub attenuation: Vector3<f64>,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattering>;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vector3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scattering> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        Some(Scattering {
            scattered: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattering> {
        let reflected = reflect(r_in.dir(), rec.normal);
        let reflected = reflected.normalize() + self.fuzz * random_unit_vector();
        let scattered = Ray::new(rec.p, reflected);
        if scattered.dir().dot(&rec.normal) > 0.0 {
            Some(Scattering {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    /// Ratio of material's index over enclosing media (air)
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scattering> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.dir().normalize();
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_double() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, ri)
            };

        Some(Scattering {
            scattered: Ray::new(rec.p, direction),
            attenuation,
        })
    }
}

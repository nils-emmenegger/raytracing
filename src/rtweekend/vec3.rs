use super::{random_double, random_double_in};

pub use nalgebra::Vector3;

pub fn near_zero(v: Vector3<f64>) -> bool {
    const S: f64 = 1e-8;
    v.iter().all(|&x| x.abs() < S)
}

/// Returns a random vector with each component in [0, 1)
pub fn random() -> Vector3<f64> {
    Vector3::new(random_double(), random_double(), random_double())
}

/// Returns a random vector with each component in [min, max)
pub fn random_in(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        random_double_in(min, max),
        random_double_in(min, max),
        random_double_in(min, max),
    )
}

pub fn random_in_unit_disk() -> Vector3<f64> {
    loop {
        let p = Vector3::new(
            random_double_in(-1.0, 1.0),
            random_double_in(-1.0, 1.0),
            0.0,
        );
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vector3<f64> {
    loop {
        let p = random_in(-1.0, 1.0);
        let lensq = p.norm_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - (2.0 * v.dot(&n)) * n
}

pub fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let cos_theta = (-uv.dot(&n)).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

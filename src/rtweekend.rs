// Reference-Pounting Pointer

pub use std::rc::Rc;

// Constants

pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

// Utility functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    use rand::prelude::*;
    use rand_xoshiro::Xoshiro256Plus;
    use std::cell::RefCell;

    thread_local!(static RNG: RefCell<Xoshiro256Plus> = RefCell::new(Xoshiro256Plus::seed_from_u64(1)));

    // Generates a float in the range [0, 1)
    RNG.with_borrow_mut(|rng| rng.gen::<f64>())
}

pub fn random_double_in(min: f64, max: f64) -> f64 {
    // generates a random float in [min, max)
    min + (max - min) * random_double()
}

// Common Headers

mod colour;
mod interval;
mod ray;
mod vec3;

pub use colour::*;
pub use interval::*;
pub use ray::*;
pub use vec3::*;

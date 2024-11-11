// Atomic Reference-Counting Pointer
pub use std::sync::Arc;

// Intervals
pub type Interval = std::ops::Range<f64>;

// Constants

pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

// Utility functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Returns a random f64 in [0, 1)
pub fn random_double() -> f64 {
    use rand::prelude::*;
    use rand_xoshiro::Xoshiro256Plus;
    use std::cell::RefCell;

    thread_local!(static RNG: RefCell<Xoshiro256Plus> = RefCell::new(Xoshiro256Plus::from_entropy()));

    RNG.with_borrow_mut(|rng| rng.gen::<f64>())
}

/// Returns a random f64 in [min, max)
pub fn random_double_in(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

// Common Headers

mod colour;
mod ray;
mod vec3;

pub use colour::*;
pub use ray::*;
pub use vec3::*;

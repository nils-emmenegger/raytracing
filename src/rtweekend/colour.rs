use super::{Interval, Vec3};
use std::io::Write;

pub type Colour = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_colour<T: Write>(mut out: T, pixel_colour: &Colour) {
    // We assume that each of these is in [0, 1]
    let r = pixel_colour.x();
    let g = pixel_colour.y();
    let b = pixel_colour.z();
    debug_assert!((0.0..=1.0).contains(&r));
    debug_assert!((0.0..=1.0).contains(&g));
    debug_assert!((0.0..=1.0).contains(&b));
    // Apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // Translate the [0, 1] components to [0, 255]
    const INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };
    let rbyte: i32 = unsafe { (256.0 * INTENSITY.clamp(r)).to_int_unchecked() };
    let gbyte: i32 = unsafe { (256.0 * INTENSITY.clamp(g)).to_int_unchecked() };
    let bbyte: i32 = unsafe { (256.0 * INTENSITY.clamp(b)).to_int_unchecked() };

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}

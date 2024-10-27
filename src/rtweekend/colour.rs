use super::Vector3;
use std::io::Write;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

/// Clamp to [0, 0.999]
fn clamp_intensity(x: f64) -> f64 {
    x.clamp(0.0, 0.999)
}

pub fn write_colour<T: Write>(mut out: T, pixel_colour: Vector3<f64>) {
    // We assume that each of these is in [0, 1]
    let r = pixel_colour.x;
    let g = pixel_colour.y;
    let b = pixel_colour.z;
    debug_assert!((0.0..=1.0).contains(&r));
    debug_assert!((0.0..=1.0).contains(&g));
    debug_assert!((0.0..=1.0).contains(&b));
    // Apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // Translate the [0, 1] components to [0, 255]
    let rbyte: i32 = unsafe { (256.0 * clamp_intensity(r)).to_int_unchecked() };
    let gbyte: i32 = unsafe { (256.0 * clamp_intensity(g)).to_int_unchecked() };
    let bbyte: i32 = unsafe { (256.0 * clamp_intensity(b)).to_int_unchecked() };

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}

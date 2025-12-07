use core::f32;
use std::{
    io::{self, Write},
    ops::RangeInclusive,
};

use rand::{Rng, rng};

use crate::math::Vec3;

#[allow(unused)]
pub fn print_progress(j: i32, height: i32) {
    let progress = (100.0 * (j as f32 + 1.0) / height as f32) as i32;
    assert!(progress >= 0 && progress <= 100);
    let plus = "#".repeat(progress as usize / 4);
    let minus = "-".repeat(25 - (progress as usize / 4));
    print!("\rProgress: [{}{}] {:>3.0}%", plus, minus, progress);
    io::stdout().flush().unwrap();
}

#[allow(unused)]
pub fn random_float() -> f32 {
    let mut rng = rng();
    rng.random_range(0.0..=1.0)
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    let mut rng = rng();
    rng.random_range(min..=max)
}

#[allow(unused)]
pub fn sample_square() -> Vec3 {
    Vec3::new(random_float() - 0.5, random_float() - 0.5, 0.0)
}

#[allow(unused)]
pub fn linear_to_gamma(linear: f32) -> f32 {
    if linear > 0.0 {
        return f32::sqrt(linear);
    }
    0.0
}

#[allow(unused)]
pub fn surrounds(range: &RangeInclusive<f32>, a: &f32) -> bool {
    range.start() < a && range.end() > a
}

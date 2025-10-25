use core::f64;
use std::{
    fs::File,
    io::{self, Write},
    time::Instant,
};

use crate::{
    math::{
        Vec3,
        ray::{HittableList, Sphere},
    },
    util::Camera,
};

mod math;
mod util;

fn main() {
    println!("=================");
    println!("Starting ray tracing");
    println!("=================");

    let file = File::create("image.ppm").unwrap();

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::build(16.0 / 9.0, 400);
    let now = Instant::now();
    camera.render(&world, file);

    println!("\nDone, it took: {:?}", now.elapsed());
}

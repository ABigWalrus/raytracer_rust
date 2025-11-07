use std::{fs::File, time::Instant};

use winit::event_loop::{ControlFlow, EventLoop};

use crate::{
    app::RayTracer,
    math::Vec3,
    rt_core::{Camera, Color, Dielectric, HittableList, Lambertian, Metal, Sphere},
};

mod app;
mod math;
mod rt_core;
mod util;

fn main() {
    // println!("=================");
    // println!("Starting ray tracing");
    // println!("=================");

    // let file = File::create("image.ppm").unwrap();

    // let mut world = HittableList::new();

    // let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    // world.add(Sphere::new(
    //     Vec3::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     material_ground,
    // ));

    // for a in -11..11 {
    //     for b in -11..11 {
    //         let choose_material = random_float();
    //         let center = Vec3::new(
    //             a as f64 + 0.9 * random_float(),
    //             0.2,
    //             b as f64 + 0.9 * random_float(),
    //         );

    //         if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
    //             if choose_material < 0.8 {
    //                 let albedo = Color::random() * Color::random();
    //                 let sphere_material = Lambertian::new(albedo);
    //                 world.add(Sphere::new(center, 0.2, sphere_material));
    //             } else if choose_material < 0.95 {
    //                 let albedo = Color::random_range(0.5, 1.0);
    //                 let fuzz = random_float_range(0.0, 0.5);
    //                 let sphere_material = Metal::new(albedo, fuzz);
    //                 world.add(Sphere::new(center, 0.2, sphere_material));
    //             } else {
    //                 let sphere_material = Dielectric::new(1.5);
    //                 world.add(Sphere::new(center, 0.2, sphere_material));
    //             }
    //         }
    //     }
    // }

    // let sphere_material = Dielectric::new(1.5);
    // world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, sphere_material));

    // let sphere_material = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    // world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, sphere_material));

    // let sphere_material = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    // world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, sphere_material));

    // let camera = Camera::build(
    //     16.0 / 9.0,
    //     // 1200,
    //     300,
    //     20.0,
    //     Vec3::new(13.0, 2.0, 3.0),
    //     Vec3::new(0.0, 0.0, 0.0),
    //     10,
    //     50,
    //     0.6,
    //     10.0,
    // );
    // let now = Instant::now();
    // camera.render(&world, file);

    // println!("\nDone, it took: {:?}", now.elapsed());

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut ray_tracer = RayTracer::default();

    event_loop.run_app(&mut ray_tracer);
}

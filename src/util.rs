use core::f64;
use std::io::{self, Write};

use rand::{Rng, distr, rng};

use crate::math::{Ray, Vec3, ray::Hittable};

pub type Color = Vec3;

pub fn write_color(file: &mut impl Write, pixel: Color) {
    let inetsity = Interval::new(0.0, 0.999);
    let r = (256.0 * inetsity.clamp(pixel.x())) as i64;
    let g = (256.0 * inetsity.clamp(pixel.y())) as i64;
    let b = (256.0 * inetsity.clamp(pixel.z())) as i64;

    writeln!(file, "{} {} {}", r, g, b).unwrap();
}

fn print_progress(j: i32, height: i32) {
    let progress = (100.0 * (j as f64 + 1.0) / height as f64) as i32;
    assert!(progress >= 0 && progress <= 100);
    let plus = "#".repeat(progress as usize / 4);
    let minus = "-".repeat(25 - (progress as usize / 4));
    print!("\rProgress: [{}{}] {:>3.0}%", plus, minus, progress);
    io::stdout().flush().unwrap();
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if self.min > x {
            return self.min;
        }
        if self.max < x {
            return self.max;
        }
        x
    }
}

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: Vec3,
    image_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    pixels_sample_scale: f64,
}

const SAMPLES_PER_PIXEL: i32 = 100;

impl Camera {
    pub fn build(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let center = Vec3::zero();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u.div(image_width as f64);
        let pixel_delta_v = viewport_v.div(image_height as f64);

        let viewport_upper_left = center.clone()
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u.div(2.0)
            - viewport_v.div(2.0);
        let image_center =
            viewport_upper_left + (pixel_delta_u.clone() + pixel_delta_v.clone()).mul(0.5);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            image_center,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: SAMPLES_PER_PIXEL,
            pixels_sample_scale: 1.0 / SAMPLES_PER_PIXEL as f64,
        }
    }

    pub fn render(&self, world: &impl Hittable, mut target: impl Write) {
        writeln!(
            target,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )
        .unwrap();

        for j in 0..self.image_height {
            print_progress(j, self.image_height);
            for i in 0..self.image_width {
                let mut pixel = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel += ray_color(&ray, world);
                }
                // let pixel_center = self.image_center.clone()
                //     + self.pixel_delta_u.mul(f64::from(i))
                //     + self.pixel_delta_v.clone().mul(f64::from(j));
                // let ray_direction = pixel_center - self.center.clone();

                // let ray = Ray::new(self.center.clone(), ray_direction);
                // write_color(&mut target, ray_color(&ray, world));
                write_color(&mut target, pixel.mul(self.pixels_sample_scale));
            }
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.image_center.clone()
            + self.pixel_delta_u.mul(i as f64 + offset.x())
            + self.pixel_delta_v.mul(j as f64 + offset.y());
        let origin = self.center.clone();
        let dir = pixel_sample - origin.clone();

        Ray::new(origin, dir)
    }
}

fn ray_color(ray: &Ray, hittable: &impl Hittable) -> Color {
    if let Some(record) = hittable.hit(ray, Interval::new(0.0, f64::INFINITY)) {
        return (record.normal + Color::new(1.0, 1.0, 1.0)).mul(0.5);
    }

    let unit_dir = ray.dir().unit();
    let a = 0.5 * (unit_dir.y() + 1.0);
    Color::new(1.0, 1.0, 1.0).mul(1.0 - a) + Color::new(0.5, 0.7, 1.0).mul(a)
}

fn random_double() -> f64 {
    let mut rng = rng();
    rng.random_range(0.0..1.0)
}

fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rng();
    rng.random_range(min..max)
}

fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
}

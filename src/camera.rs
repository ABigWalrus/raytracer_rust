use std::f32::consts;

use crate::{
    math::Vec3,
    // rt_core::scene::{Hittable, Ray},
};

pub struct Camera {
    // aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    center: Vec3,
    image_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    pixels_sample_scale: f32,
    maximum_depth: i32,
    // vfov: f32,
    // u: Vec3,
    // v: Vec3,
    // w: Vec3,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

// const SAMPLES_PER_PIXEL: i32 = 10;
// const MAXIMUM_DEPTH: i32 = 50;

impl Camera {
    pub fn build(
        image_width: u32,
        image_height: u32,
        vfov: f32,
        look_from: Vec3,
        look_at: Vec3,
        samples_per_pixel: i32,
        maximum_depth: i32,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        // let image_height = (image_width as f32 / aspect_ratio) as i32;
        // let image_height = if image_height < 1 { 1 } else { image_height };

        let up_vector = Vec3::new(0.0, 1.0, 0.0);
        let center = look_from.clone();

        // let focal_length = (look_from.clone() - look_at.clone()).length();
        let theta = vfov / 180.0 * consts::PI;
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (look_from - look_at).unit();
        let u = up_vector.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = u.mul(viewport_width);
        let viewport_v = (-v.clone()).mul(viewport_height);

        let pixel_delta_u = viewport_u.div(image_width as f32);
        let pixel_delta_v = viewport_v.div(image_height as f32);

        let viewport_upper_left =
            center.clone() - w.mul(focus_dist) - viewport_u.div(2.0) - viewport_v.div(2.0);
        let image_center =
            viewport_upper_left + (pixel_delta_u.clone() + pixel_delta_v.clone()).mul(0.5);

        let defocus_radius = focus_dist * f32::tan(defocus_angle / 2.0 / 180.0 * consts::PI);
        let defocus_disk_u = u.mul(defocus_radius);
        let defocus_disk_v = v.mul(defocus_radius);

        Self {
            image_width,
            image_height,
            center,
            image_center,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixels_sample_scale: 1.0 / samples_per_pixel as f32,
            maximum_depth: maximum_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn to_bytes(&self) -> [u8; 128] {
        let mut bytes = [0u8; 128];

        bytes[0..4].copy_from_slice(&self.image_width.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.image_height.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.samples_per_pixel.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.pixels_sample_scale.to_le_bytes());

        bytes[16..32].copy_from_slice(&self.image_center.to_bytes());
        bytes[32..48].copy_from_slice(&self.pixel_delta_u.to_bytes());
        bytes[48..64].copy_from_slice(&self.pixel_delta_v.to_bytes());
        bytes[64..80].copy_from_slice(&self.center.to_bytes());
        bytes[80..96].copy_from_slice(&self.defocus_disk_u.to_bytes());
        bytes[96..112].copy_from_slice(&self.defocus_disk_v.to_bytes());

        bytes[112..116].copy_from_slice(&self.maximum_depth.to_le_bytes());
        bytes[116..120].copy_from_slice(&self.defocus_angle.to_le_bytes());
        // bytes[96..128].copy_from_slice(&[0u8; 32]);

        bytes

        // let mut bytes = [0u8; 128];
        // let f32_pad = [0u8; 4];

        // bytes[0..4].copy_from_slice(&self.image_width.to_le_bytes());
        // bytes[4..8].copy_from_slice(&self.image_height.to_le_bytes());

        // bytes[8..20].copy_from_slice(&self.center.to_bytes());
        // bytes[20..24].copy_from_slice(&f32_pad);
        // bytes[24..36].copy_from_slice(&self.image_center.to_bytes());
        // bytes[36..40].copy_from_slice(&f32_pad);
        // bytes[40..52].copy_from_slice(&self.pixel_delta_u.to_bytes());
        // bytes[52..56].copy_from_slice(&f32_pad);
        // bytes[56..68].copy_from_slice(&self.pixel_delta_v.to_bytes());
        // bytes[68..72].copy_from_slice(&f32_pad);

        // bytes[72..76].copy_from_slice(&self.samples_per_pixel.to_le_bytes());
        // bytes[76..80].copy_from_slice(&self.pixels_sample_scale.to_le_bytes());

        // bytes[80..84].copy_from_slice(&self.maximum_depth.to_le_bytes());
        // bytes[84..88].copy_from_slice(&self.defocus_angle.to_le_bytes());

        // bytes[88..100].copy_from_slice(&self.defocus_disk_u.to_bytes());
        // bytes[100..104].copy_from_slice(&f32_pad);
        // bytes[104..116].copy_from_slice(&self.defocus_disk_v.to_bytes());
        // bytes[116..120].copy_from_slice(&f32_pad);
        // bytes[120..128].copy_from_slice(&[0u8; 8]);
    }

    // pub fn render(&self, world: &impl Hittable, mut target: impl Write) {
    //     writeln!(
    //         target,
    //         "P3\n{} {}\n255",
    //         self.image_width, self.image_height
    //     )
    //     .unwrap();

    //     for j in 0..self.image_height {
    //         print_progress(j, self.image_height);
    //         for i in 0..self.image_width {
    //             let mut pixel = Color::new(0.0, 0.0, 0.0);
    //             for _sample in 0..self.samples_per_pixel {
    //                 let ray = self.get_ray(i, j);
    //                 pixel += self.ray_color(&ray, 0, world);
    //             }
    //             write_color(&mut target, pixel.mul(self.pixels_sample_scale));
    //         }
    //     }
    // }

    // fn get_ray(&self, i: i32, j: i32) -> Ray {
    //     let offset = sample_square();
    //     let pixel_sample = self.image_center.clone()
    //         + self.pixel_delta_u.mul(i as f32 + offset.x())
    //         + self.pixel_delta_v.mul(j as f32 + offset.y());
    //     let origin = if self.defocus_angle <= 0.0 {
    //         self.center.clone()
    //     } else {
    //         self.defocus_disk_sample()
    //     };
    //     let dir = pixel_sample - origin.clone();

    //     Ray::new(origin, dir)
    // }

    // fn ray_color(&self, ray: &Ray, depth: i32, hittable: &impl Hittable) -> Color {
    //     if depth >= self.maximum_depth {
    //         return Color::new(0.0, 0.0, 0.0);
    //     }

    //     if let Some(record) = hittable.hit(ray, 0.001..=f32::INFINITY) {
    //         let color = Vec3::zero();
    //         if let Some((scattered, attenuation)) = record.material.scatter(&ray, &record, &color) {
    //             return attenuation * self.ray_color(&scattered, depth + 1, hittable);
    //         } else {
    //             return Vec3::zero();
    //         }
    //     }

    //     let unit_dir = ray.dir().unit();
    //     let a = 0.5 * (unit_dir.y() + 1.0);
    //     Color::new(1.0, 1.0, 1.0).mul(1.0 - a) + Color::new(0.5, 0.7, 1.0).mul(a)
    // }

    // fn defocus_disk_sample(&self) -> Vec3 {
    //     let vec = Vec3::random_in_unit_disk();
    //     self.center.clone() + self.defocus_disk_u.mul(vec.x()) + self.defocus_disk_v.mul(vec.y())
    // }
}

// fn write_color(file: &mut impl Write, pixel: Color) {
//     let r = linear_to_gamma(pixel.x());
//     let g = linear_to_gamma(pixel.y());
//     let b = linear_to_gamma(pixel.z());

//     let intensity = 0.0..=0.999;
//     let r = (256.0 * r.clamp(*intensity.start(), *intensity.end())) as i64;
//     let g = (256.0 * g.clamp(*intensity.start(), *intensity.end())) as i64;
//     let b = (256.0 * b.clamp(*intensity.start(), *intensity.end())) as i64;

//     writeln!(file, "{} {} {}", r, g, b).unwrap();
// }

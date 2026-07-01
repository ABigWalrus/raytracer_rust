use std::f32::consts;

use crate::math::{
    mat::Mat4,
    vec::{Radians, Vec3, Vec4},
};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    position: Vec3,
    look_at: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    first_pixel_pos: Vec3,
    vfov: f32,
    needs_update: bool,
}

impl Camera {
    pub fn new(
        image_width: u32,
        image_height: u32,
        vfov: f32,
        position: Vec3,
        look_at: Vec3,
    ) -> Self {
        Self {
            image_width,
            image_height,
            position,
            look_at,
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            first_pixel_pos: Vec3::zero(),
            vfov,
            needs_update: true,
        }
    }

    pub fn update(&mut self) {
        if !self.needs_update {
            return;
        }
        self.needs_update = false;
        let up_vector = Vec3::new(0.0, 1.0, 0.0);

        let focal_length = (self.position - self.look_at).length();
        let theta = self.vfov / 180.0 * consts::PI;
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;

        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let w = (self.position - self.look_at).normalize();
        let u = up_vector.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = u.mul(viewport_width);
        let viewport_v = (-v).mul(viewport_height);

        self.pixel_delta_u = viewport_u.div(self.image_width as f32);
        self.pixel_delta_v = viewport_v.div(self.image_height as f32);

        let viewport_upper_left =
            self.position - w.mul(focal_length) - viewport_u.div(2.0) - viewport_v.div(2.0);
        self.first_pixel_pos =
            viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v).mul(0.5);
    }

    /// ## WGSL schema:
    /// struct Camera {
    ///     first_pixel_pos: vec3<f32>,
    ///     pixel_delta_u: vec3<f32>,
    ///     pixel_delta_v: vec3<f32>,
    ///     position: vec3<f32>,
    /// }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        bytes.extend_from_slice(&self.first_pixel_pos.to_bytes());
        bytes.extend_from_slice(&self.pixel_delta_u.to_bytes());
        bytes.extend_from_slice(&self.pixel_delta_v.to_bytes());
        bytes.extend_from_slice(&self.position.to_bytes());

        assert!(bytes.len() % 4 == 0);
        bytes
    }

    pub fn translate(&mut self, vec: Vec3) {
        let forward = (self.look_at - self.position).normalize();

        let world_up = Vec3::new(0.0, 1.0, 0.0);
        let right = forward.cross(&world_up).normalize();
        let up = right.cross(&forward);
        let offset = (forward.mul(vec.z())) + (right.mul(vec.x())) + (up.mul(vec.y()));

        self.position += offset;
        self.look_at += offset;
        self.needs_update = true;
    }

    pub fn rotate(&mut self, rotation: (f32, f32)) {
        let dir = Vec4::from_vec3(self.look_at - self.position, 1.0);

        let rotation_matrix = Mat4::rotation(Radians::new(rotation.0, rotation.1, 0.0));
        let new_dir = rotation_matrix * dir;

        self.look_at = new_dir.get_vec3() + self.position;
        self.needs_update = true;
    }
}

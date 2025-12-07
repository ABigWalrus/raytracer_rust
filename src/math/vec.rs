use std::ops;

use crate::util::random_float_range;

#[derive(Clone)]
pub struct Vec3 {
    value: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { value: [x, y, z] }
    }

    pub fn zero() -> Self {
        Self { value: [0.0; 3] }
    }

    pub const fn x(&self) -> f32 {
        self.value[0]
    }

    pub const fn y(&self) -> f32 {
        self.value[1]
    }

    pub const fn z(&self) -> f32 {
        self.value[2]
    }

    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn div(&self, scalar: f32) -> Self {
        Self::new(self.x() / scalar, self.y() / scalar, self.z() / scalar)
    }

    pub fn mul(&self, scalar: f32) -> Self {
        Self::new(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn unit(&self) -> Self {
        self.div(self.length())
    }

    // pub fn random() -> Self {
    //     Self::new(random_float(), random_float(), random_float())
    // }

    pub fn random_range(min: f32, max: f32) -> Self {
        Self::new(
            random_float_range(min, max),
            random_float_range(min, max),
            random_float_range(min, max),
        )
    }

    pub fn random_unit() -> Self {
        loop {
            let vec = Vec3::random_range(-1.0, 1.0);
            let lensq = vec.length_squared();
            if lensq >= 1e-160 && lensq <= 1.0 {
                return vec.div(f32::sqrt(lensq));
            }
        }
    }

    // pub fn random_on_hemisphere(normal: &Vec3) -> Self {
    //     let vec = Vec3::random_unit();
    //     if vec.dot(normal) >= 0.0 { vec } else { -vec }
    // }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        self.clone() - normal.mul(self.dot(&normal) * 2.0)
    }

    pub fn refract(&self, normal: &Vec3, factor: f32) -> Self {
        let cos_theta = f32::min((-self.clone()).dot(normal), 1.0);
        let out_perp = (self.clone() + normal.mul(cos_theta)).mul(factor);
        let out_parallel = normal.mul(-f32::sqrt((1.0 - out_perp.length_squared()).abs()));

        out_perp + out_parallel
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let vec = Vec3::new(
                random_float_range(-1.0, 1.0),
                random_float_range(-1.0, 1.0),
                0.0,
            );
            if vec.length_squared() <= 1.0 {
                return vec;
            }
        }
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0..4].copy_from_slice(&self.value[0].to_le_bytes());
        bytes[4..8].copy_from_slice(&self.value[1].to_le_bytes());
        bytes[8..12].copy_from_slice(&self.value[2].to_le_bytes());
        bytes
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.value[0] += rhs.x();
        self.value[1] += rhs.y();
        self.value[2] += rhs.z();
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value[0] -= rhs.x();
        self.value[1] -= rhs.y();
        self.value[2] -= rhs.z();
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.value[0] *= rhs.x();
        self.value[1] *= rhs.y();
        self.value[2] *= rhs.z();
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.value[0] /= rhs.x();
        self.value[1] /= rhs.y();
        self.value[2] /= rhs.z();
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

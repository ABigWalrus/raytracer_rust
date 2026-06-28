use std::ops;

use crate::math::vec::{Radians, Vec4};

#[inline]
fn mul_and_add_slices3(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn mul_and_add_slices4(a: [f32; 4], b: [f32; 4]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}

pub struct Mat3 {
    items: [f32; 9],
}

impl Mat3 {
    pub fn zero() -> Self {
        Self { items: [0.0; 9] }
    }

    fn from_slice(slice: [f32; 9]) -> Self {
        Self { items: slice }
    }

    // #[inline]
    // fn from_slices(slices: [[f32; 3]; 3]) -> Self {
    //     Self {
    //         items: bytemuck::cast(slices),
    //     }
    // }

    #[inline]
    fn rotation(angles: Radians) -> Self {
        let cos_alpha = angles.alpha().cos();
        let sin_alpha = angles.alpha().sin();

        let cos_beta = angles.beta().cos();
        let sin_beta = angles.beta().sin();

        let cos_gamma = angles.gamma().cos();
        let sin_gamma = angles.gamma().sin();

        Self::from_slice([
            (cos_beta * cos_gamma),
            (sin_alpha * sin_beta * cos_gamma - cos_alpha * sin_gamma),
            (cos_alpha * sin_beta * cos_gamma + sin_alpha * sin_gamma),
            (cos_beta * sin_gamma),
            (sin_alpha * sin_beta * sin_gamma + cos_alpha * cos_gamma),
            (cos_alpha * sin_beta * sin_gamma - sin_alpha * cos_gamma),
            (-sin_beta),
            (sin_alpha * cos_beta),
            (cos_alpha * cos_beta),
        ])
    }

    #[inline]
    fn get_row(&self, idx: usize) -> [f32; 3] {
        assert!(idx < 3);
        let row_idx = idx * 3;
        [
            self.items[row_idx + 0],
            self.items[row_idx + 1],
            self.items[row_idx + 2],
        ]
    }

    #[inline]
    fn get_column(&self, idx: usize) -> [f32; 3] {
        assert!(idx < 3);
        [
            self.items[idx + 0 * 3],
            self.items[idx + 1 * 3],
            self.items[idx + 2 * 3],
        ]
    }
}

impl ops::Mul for Mat3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut slice = [0.0; 9];

        for i in 0..3 {
            let row = self.get_row(i);
            for j in 0..3 {
                slice[i + j * 3] = mul_and_add_slices3(row, rhs.get_column(j));
            }
        }

        Self::from_slice(slice)
    }
}

impl ops::MulAssign for Mat3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let mut slice = [0.0; 9];

        for i in 0..3 {
            let row = self.get_row(i);
            for j in 0..3 {
                slice[i + j * 3] = mul_and_add_slices3(row, rhs.get_column(j));
            }
        }

        self.items = slice;
    }
}

pub struct Mat4 {
    items: [f32; 16],
}

impl Mat4 {
    pub fn zero() -> Self {
        Self { items: [0.0; 16] }
    }

    fn from_slice(slice: [f32; 16]) -> Self {
        Self { items: slice }
    }

    #[inline]
    pub fn rotation(angles: Radians) -> Self {
        let cos_alpha = angles.alpha().cos();
        let sin_alpha = angles.alpha().sin();

        let cos_beta = angles.beta().cos();
        let sin_beta = angles.beta().sin();

        let cos_gamma = angles.gamma().cos();
        let sin_gamma = angles.gamma().sin();

        Self::from_slice([
            (cos_beta * cos_gamma),
            (sin_alpha * sin_beta * cos_gamma - cos_alpha * sin_gamma),
            (cos_alpha * sin_beta * cos_gamma + sin_alpha * sin_gamma),
            0.0,
            (cos_beta * sin_gamma),
            (sin_alpha * sin_beta * sin_gamma + cos_alpha * cos_gamma),
            (cos_alpha * sin_beta * sin_gamma - sin_alpha * cos_gamma),
            0.0,
            (-sin_beta),
            (sin_alpha * cos_beta),
            (cos_alpha * cos_beta),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }

    #[inline]
    fn get_row(&self, idx: usize) -> [f32; 4] {
        assert!(idx < 4);
        let row_idx = idx * 4;
        [
            self.items[row_idx + 0],
            self.items[row_idx + 1],
            self.items[row_idx + 2],
            self.items[row_idx + 3],
        ]
    }

    #[inline]
    fn get_column(&self, idx: usize) -> [f32; 4] {
        assert!(idx < 4);
        [
            self.items[idx + 0 * 4],
            self.items[idx + 1 * 4],
            self.items[idx + 2 * 4],
            self.items[idx + 3 * 4],
        ]
    }
}

impl ops::Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut slice = [0.0; 16];

        for i in 0..4 {
            let row = self.get_row(i);
            for j in 0..4 {
                slice[i + j * 4] = mul_and_add_slices4(row, rhs.get_column(j));
            }
        }

        Self::from_slice(slice)
    }
}

impl ops::MulAssign for Mat4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let mut slice = [0.0; 16];

        for i in 0..4 {
            let row = self.get_row(i);
            for j in 0..4 {
                slice[i + j * 4] = mul_and_add_slices4(row, rhs.get_column(j));
            }
        }

        self.items = slice;
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            Vec4::from_slice(self.get_row(0)).dot(&rhs),
            Vec4::from_slice(self.get_row(1)).dot(&rhs),
            Vec4::from_slice(self.get_row(2)).dot(&rhs),
            Vec4::from_slice(self.get_row(3)).dot(&rhs),
        )
    }
}

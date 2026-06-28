use std::ops;

use crate::math::vec::Vec4;

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

#[inline]
fn mul_and_add_slices(a: [f32; 4], b: [f32; 4]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}

impl ops::Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut slice = [0.0; 16];

        for i in 0..4 {
            let row = self.get_row(i);
            for j in 0..4 {
                slice[i + j * 4] = mul_and_add_slices(row, rhs.get_column(j));
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
                slice[i + j * 4] = mul_and_add_slices(row, rhs.get_column(j));
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

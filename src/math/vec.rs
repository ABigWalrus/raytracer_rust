use std::ops;

#[derive(Clone)]
pub struct Vec3 {
    value: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { value: [x, y, z] }
    }

    pub fn zero() -> Self {
        Self { value: [0.0; 3] }
    }

    pub const fn x(&self) -> f64 {
        self.value[0]
    }

    pub const fn y(&self) -> f64 {
        self.value[1]
    }

    pub const fn z(&self) -> f64 {
        self.value[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn div(&self, scalar: f64) -> Self {
        Self::new(self.x() / scalar, self.y() / scalar, self.z() / scalar)
    }

    pub fn mul(&self, scalar: f64) -> Self {
        Self::new(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }

    pub fn dot(&self, other: &Self) -> f64 {
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

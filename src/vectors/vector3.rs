use fixed::types::I48F16;
use fixed_sqrt::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::vectors::SignedFractional;


#[derive(Eq, PartialEq, Debug, Default, Hash, Copy, Clone)]
pub struct Vec3 {
    pub x: SignedFractional,
    pub y: SignedFractional,
    pub z: SignedFractional,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: SignedFractional::ZERO,
        y: SignedFractional::ZERO,
        z: SignedFractional::ZERO,
    };

    pub const fn new(x: SignedFractional, y: SignedFractional, z: SignedFractional) -> Self {
        Self { x, y, z }
    }

    pub fn len_pow2(&self) -> SignedFractional {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> SignedFractional {
        self.len_pow2().sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.len();
    }

    pub fn get_normalized(&self) -> Self {
        let len = self.len();

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    #[inline]
    #[cold]
    fn considers_this_unlikely_to_happen() {}

    pub fn try_get_normalized(&self) -> Option<Self> {
        let len = self.len();

        if len == SignedFractional::ZERO {
            Self::considers_this_unlikely_to_happen();
            return None;
        }

        Some(Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        })
    }
}

impl From<(SignedFractional, SignedFractional, SignedFractional)> for Vec3 {
    fn from(n: (SignedFractional, SignedFractional, SignedFractional)) -> Self {
        Self { x: n.0, y: n.1, z: n.2 }
    }
}

impl From<Vec3> for (SignedFractional, SignedFractional, SignedFractional) {
    fn from(n: Vec3) -> Self {
        (n.x, n.y, n.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<(SignedFractional, SignedFractional, SignedFractional)> for Vec3 {
    type Output = Self;

    fn add(self, rhs: (SignedFractional, SignedFractional, SignedFractional)) -> Self::Output {
        self + Into::<Self>::into(rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<(SignedFractional, SignedFractional, SignedFractional)> for Vec3 {
    fn add_assign(&mut self, rhs: (SignedFractional, SignedFractional, SignedFractional)) {
        self.x += rhs.0;
        self.y += rhs.1;
        self.z += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Sub<(SignedFractional, SignedFractional, SignedFractional)> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: (SignedFractional, SignedFractional, SignedFractional)) -> Self::Output {
        self - Into::<Self>::into(rhs)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<(SignedFractional, SignedFractional, SignedFractional)> for Vec3 {
    fn sub_assign(&mut self, rhs: (SignedFractional, SignedFractional, SignedFractional)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
        self.z -= rhs.2;
    }
}

impl Mul<SignedFractional> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: SignedFractional) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl MulAssign<SignedFractional> for Vec3 {
    fn mul_assign(&mut self, rhs: SignedFractional) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<SignedFractional> for Vec3 {
    type Output = Self;

    fn div(self, rhs: SignedFractional) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl DivAssign<SignedFractional> for Vec3 {
    fn div_assign(&mut self, rhs: SignedFractional) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod test {
    use crate::vectors::{SignedFractional, Vec3};

    #[test]
    // Tests that derive(Eq) continues to be correct
    fn sanity_check() {
        let x = Vec3::new(2.into(), 3.into(), 6.into());
        let y = Vec3::new(5.into(), 7.into(), 9.into());

        assert_eq!(x, x);
        assert_ne!(x, y);
    }

    #[test]
    fn from_tuple() {
        let x: Vec3 = (5.into(), 7.into(), 9.into()).into();
        let y = Vec3::new(5.into(), 7.into(), 9.into());

        assert_eq!(x, y);
    }

    #[test]
    fn into_tuple() {
        let x: (SignedFractional, SignedFractional, SignedFractional) = Vec3::new(5.into(), 7.into(), 9.into()).into();
        let y: (SignedFractional, SignedFractional, SignedFractional) = (5.into(), 7.into(), 9.into());

        assert_eq!(x, y);
    }

    #[test]
    fn addition() {
        let x = Vec3::new(2.into(), 3.into(), 9.into());
        let y = Vec3::new(5.into(), 7.into(), 9.into());

        assert_eq!(x + y, Vec3::new(7.into(), 10.into(), 18.into()));
    }

    #[test]
    fn length() {
        let x = Vec3::new(3.into(), 4.into(), 12.into());

        assert_eq!(x.len_pow2(), 169);
        assert_eq!(x.len(), 13);
    }

    #[test]
    fn scalar_multiplication() {
        let x = Vec3::new(3.into(), 4.into(),5.into());
        let y = Vec3::new(6.into(), 8.into(), 10.into());

        assert_eq!(x * 2.into(), y);
    }

    #[test]
    fn scalar_division() {
        let x = Vec3::new(6.into(), 8.into(), 10.into());
        let y = Vec3::new(3.into(), 4.into(), 5.into());

        assert_eq!(x / 2.into(), y);
    }

    #[test]
    fn vector_normalization() {
        let x = Vec3::new(6.into(), 0.into(), 0.into());
        let y = Vec3::new(1.into(), 0.into(), 0.into());
        let wrong = Vec3::ZERO;

        assert_eq!(x.get_normalized(), y);
        assert_eq!(wrong.try_get_normalized(), None)
    }
}

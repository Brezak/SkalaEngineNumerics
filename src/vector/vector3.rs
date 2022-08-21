use crate::SignedFractional;
use fixed_sqrt::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 3d vector.
#[derive(Eq, PartialEq, Debug, Default, Hash, Copy, Clone)]
pub struct Vec3 {
    #[allow(missing_docs)]
    pub x: SignedFractional,
    #[allow(missing_docs)]
    pub y: SignedFractional,
    #[allow(missing_docs)]
    pub z: SignedFractional,
}

impl Vec3 {
    /// A vector of length zero
    pub const ZERO: Self = Self {
        x: SignedFractional::ZERO,
        y: SignedFractional::ZERO,
        z: SignedFractional::ZERO,
    };

    /// Creates a new [`Vec3`] from coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// # use skala_engine_numerics::vector::Vec3;
    /// let pos = Vec3::new(1, 1i32 , 5u8);
    /// ```
    pub fn new<A, B, C>(x: A, y: B, z: C) -> Self
    where
        A: Into<SignedFractional>,
        B: Into<SignedFractional>,
        C: Into<SignedFractional>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// Returns the magnitude of this [`Vec3`] raised to the power of two.
    ///
    /// # Examples
    /// ```
    /// # use skala_engine_numerics::vector::Vec3;
    /// let x = Vec3::new(1, 0, 0);
    ///
    /// // Proving we're working with a unit vector
    /// assert_eq!(x.magintude_pow2(), 1);
    /// ```
    pub fn magintude_pow2(&self) -> SignedFractional {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the magnitude of this [`Vec3`]
    /// # Examples
    /// ```
    /// # use skala_engine_numerics::vector::Vec3;
    /// let x = Vec3::new(2, 4, 4);
    ///
    /// assert_eq!(x.magnitude(), 6);
    /// ```
    pub fn magnitude(&self) -> SignedFractional {
        self.magintude_pow2().sqrt()
    }

    /// Sets the magnitude of this [`Vec3`] to one
    ///
    /// # Panics
    /// If vector is magnitude is zero
    ///
    /// # Examples
    /// ```
    /// # use skala_engine_numerics::vector::Vec3;
    /// let mut x = Vec3::new(20, 0, 0);
    ///
    /// // Before normalization
    /// assert_eq!(x.magnitude(), 20);
    ///
    /// x.normalize();
    /// // After normalization
    /// assert_eq!(x.magnitude(), 1);
    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }

    /// Creates a [`Vec3`] with magnitude equal to one and rotation equal to this [`Vec3`]
    ///
    /// # Panics
    /// If vector magnitude is 0
    ///
    /// # Examples
    /// ```
    /// # use skala_engine_numerics::vector::Vec3;
    /// let x = Vec3::new(10, 0, 0);
    ///
    /// assert_eq!(x.get_normalized(), Vec3::new(1, 0, 0));
    /// ```
    pub fn get_normalized(&self) -> Self {
        let len = self.magnitude();

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    #[inline]
    #[cold]
    /// stable equivalent of std::intrinsics::unlikely
    fn considers_this_unlikely_to_happen() {}

    /// Creates a [`Vec3`] with magnitude equal to one and rotation equal to this [`Vec3`]
    ///
    /// # Examples
    /// ```
    /// # use skala_engine_numerics::vector::Vec3;
    /// let x = Vec3::new(10, 0, 0);
    /// let zero = Vec3::new(0, 0, 0);
    ///
    /// assert_eq!(x.try_get_normalized(), Some(Vec3::new(1, 0, 0)));
    /// assert_eq!(zero.try_get_normalized(), None);
    /// ```
    pub fn try_get_normalized(&self) -> Option<Self> {
        let len = self.magnitude();

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
        Self {
            x: n.0,
            y: n.1,
            z: n.2,
        }
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
            z: self.z - rhs.z,
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
            z: self.z * rhs,
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
            z: self.z / rhs,
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
    use crate::vector::Vec3;
    use crate::SignedFractional;

    #[test]
    // Tests that derive(Eq) continues to be correct
    fn sanity_check() {
        let x = Vec3::new(2, 3, 6);
        let y = Vec3::new(5, 7, 9);

        assert_eq!(x, x);
        assert_ne!(x, y);
    }

    #[test]
    fn from_tuple() {
        let x: Vec3 = (5.into(), 7.into(), 9.into()).into();
        let y = Vec3::new(5, 7, 9);

        assert_eq!(x, y);
    }

    #[test]
    fn into_tuple() {
        let x: (SignedFractional, SignedFractional, SignedFractional) = Vec3::new(5, 7, 9).into();
        let y: (SignedFractional, SignedFractional, SignedFractional) =
            (5.into(), 7.into(), 9.into());

        assert_eq!(x, y);
    }

    #[test]
    fn addition() {
        let x = Vec3::new(2, 3, 9);
        let y = Vec3::new(5, 7, 9);

        assert_eq!(x + y, Vec3::new(7, 10, 18));
    }

    #[test]
    fn magnitude() {
        let x = Vec3::new(3, 4, 12);
        let y = Vec3::new(2, 4, 4);

        assert_eq!(x.magintude_pow2(), 169);
        assert_eq!(x.magnitude(), 13);
        assert_eq!(y.magnitude(), 6);
    }

    #[test]
    fn scalar_multiplication() {
        let x = Vec3::new(3, 4, 5);
        let y = Vec3::new(6, 8, 10);

        assert_eq!(x * 2.into(), y);
    }

    #[test]
    fn scalar_division() {
        let x = Vec3::new(6, 8, 10);
        let y = Vec3::new(3, 4, 5);

        assert_eq!(x / 2.into(), y);
    }

    #[test]
    fn vector_normalization() {
        let x = Vec3::new(4, 4, 4);
        let wrong = Vec3::ZERO;

        assert_eq!(x.get_normalized().magnitude(), 1);
        assert_eq!(wrong.try_get_normalized(), None)
    }
}

use crate::SignedFractional;
use fixed_sqrt::FixedSqrt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Eq, PartialEq, Debug, Default, Hash, Copy, Clone)]
/// A 2d vector
pub struct Vec2 {
    #[allow(missing_docs)]
    pub x: SignedFractional,
    #[allow(missing_docs)]
    pub y: SignedFractional,
}

impl Vec2 {
    /// A `vec2` with both it's coordinates set to zero
    pub const ZERO: Self = Self {
        x: SignedFractional::ZERO,
        y: SignedFractional::ZERO,
    };

    /// Creates a new vector from given coordinates
    /// # Example
    ///
    /// ```
    /// # use skala_engine_numerics::Vec2;
    /// let vector = Vec2::new(0.into(), 0.into());
    ///
    /// assert_eq!(vector, Vec2::ZERO);
    /// ```
    #[must_use = "Creating a vector without using it is just a waste of processing time"]
    pub const fn new(x: SignedFractional, y: SignedFractional) -> Self {
        Self { x, y }
    }

    /// Calculates the magnitude of a vector without squaring the result
    ///
    /// Useful when checking if vector is a [unit vector](https://en.wikipedia.org/wiki/Unit_vector) without wasting cpu cycles
    ///
    /// # Example
    ///
    /// ```
    /// # use skala_engine_numerics::{SignedFractional, Vec2};
    /// let vector = Vec2::new(1.into(), 0.into());
    /// let length: SignedFractional = 1.into();
    ///
    /// assert_eq!(vector.len_pow2(), length);
    /// ```
    #[must_use]
    pub fn len_pow2(&self) -> SignedFractional {
        self.x * self.x + self.y * self.y
    }

    /// Calculates the magnitude of a vector
    ///
    /// If checking if a vector is a [unit vector](https://en.wikipedia.org/wiki/Unit_vector) prefer using `len_pow2`
    ///
    /// # Example
    ///
    /// ```
    /// # use skala_engine_numerics::{SignedFractional, Vec2};
    /// let vector = Vec2::new(4.into(), 0.into());
    /// let length: SignedFractional = 16.into();
    ///
    ///
    /// assert_eq!(vector.len_pow2(), length);
    /// ```
    #[must_use]
    pub fn len(&self) -> SignedFractional {
        self.len_pow2().sqrt()
    }

    /// Modifies vector to have magnitude 1
    ///
    /// # Panics
    /// When vector is a zero vector
    ///
    /// # Example
    ///
    /// ```
    /// # use skala_engine_numerics::Vec2;
    /// let mut vector = Vec2::new(4.into(), 0.into());
    /// vector.normalize();
    ///
    /// let normalized = Vec2::new(1.into(), 0.into());
    ///
    /// assert_eq!(vector, normalized);
    /// ```
    pub fn normalize(&mut self) {
        *self /= self.len();
    }

    /// Creates a new `vec2` with same direction as `self` but magnitude 1
    ///
    /// # Panics
    /// When vector is a zero vector
    ///
    /// # Example
    ///
    /// ```
    /// # use skala_engine_numerics::Vec2;
    /// let mut vector = Vec2::new(4.into(), 0.into());
    /// let normalized = Vec2::new(1.into(), 0.into());
    ///
    /// assert_eq!(vector.get_normalized(), normalized);
    /// ```
    #[must_use]
    pub fn get_normalized(&self) -> Self {
        let len = self.len();

        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }

    #[inline]
    #[cold]
    fn considers_this_unlikely_to_happen() {}

    /// Creates a new `vec2` with same direction as `self` but magnitude 1
    /// If `self` is a zero vector returns None otherwise returns created vector
    ///
    /// # Example
    ///
    /// ```
    /// # use skala_engine_numerics::Vec2;
    /// let mut vector = Vec2::new(4.into(), 0.into());
    /// let normalized = Vec2::new(1.into(), 0.into());
    ///
    /// assert_eq!(vector.try_get_normalized(), Some(normalized));
    /// assert_eq!(Vec2::ZERO.try_get_normalized(), None);
    /// ```
    #[must_use]
    pub fn try_get_normalized(&self) -> Option<Self> {
        let len = self.len();

        if len == SignedFractional::ZERO {
            Self::considers_this_unlikely_to_happen();
            return None;
        }

        Some(Self {
            x: self.x / len,
            y: self.y / len,
        })
    }
}

impl From<(SignedFractional, SignedFractional)> for Vec2 {
    fn from(n: (SignedFractional, SignedFractional)) -> Self {
        Self { x: n.0, y: n.1 }
    }
}

impl From<Vec2> for (SignedFractional, SignedFractional) {
    fn from(n: Vec2) -> Self {
        (n.x, n.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(SignedFractional, SignedFractional)> for Vec2 {
    type Output = Self;

    fn add(self, rhs: (SignedFractional, SignedFractional)) -> Self::Output {
        self + Into::<Self>::into(rhs)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<(SignedFractional, SignedFractional)> for Vec2 {
    fn add_assign(&mut self, rhs: (SignedFractional, SignedFractional)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<(SignedFractional, SignedFractional)> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: (SignedFractional, SignedFractional)) -> Self::Output {
        self - Into::<Self>::into(rhs)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<(SignedFractional, SignedFractional)> for Vec2 {
    fn sub_assign(&mut self, rhs: (SignedFractional, SignedFractional)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}

impl Mul<SignedFractional> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: SignedFractional) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<SignedFractional> for Vec2 {
    fn mul_assign(&mut self, rhs: SignedFractional) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<SignedFractional> for Vec2 {
    type Output = Self;

    fn div(self, rhs: SignedFractional) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<SignedFractional> for Vec2 {
    fn div_assign(&mut self, rhs: SignedFractional) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vec2;
    use crate::SignedFractional;

    #[test]
    // Tests that derive(Eq) continues to be correct
    fn sanity_check() {
        let x = Vec2::new(2.into(), 3.into());
        let y = Vec2::new(5.into(), 7.into());

        assert_eq!(x, x);
        assert_ne!(x, y);
    }

    #[test]
    fn from_tuple() {
        let x: Vec2 = (5.into(), 7.into()).into();
        let y = Vec2::new(5.into(), 7.into());

        assert_eq!(x, y);
    }

    #[test]
    fn into_tuple() {
        let x: (SignedFractional, SignedFractional) = Vec2::new(5.into(), 7.into()).into();
        let y: (SignedFractional, SignedFractional) = (5.into(), 7.into());

        assert_eq!(x, y);
    }

    #[test]
    fn addition() {
        let x = Vec2::new(2.into(), 3.into());
        let y = Vec2::new(5.into(), 7.into());

        assert_eq!(x + y, Vec2::new(7.into(), 10.into()));
    }

    #[test]
    fn length() {
        let x = Vec2::new(3.into(), 4.into());

        assert_eq!(x.len_pow2(), 25);
        assert_eq!(x.len(), 5);
    }

    #[test]
    fn scalar_multiplication() {
        let x = Vec2::new(3.into(), 4.into());
        let y = Vec2::new(6.into(), 8.into());

        assert_eq!(x * 2.into(), y);
    }

    #[test]
    fn scalar_division() {
        let x = Vec2::new(6.into(), 8.into());
        let y = Vec2::new(3.into(), 4.into());

        assert_eq!(x / 2.into(), y);
    }

    #[test]
    fn vector_normalization() {
        let x = Vec2::new(6.into(), 0.into());
        let y = Vec2::new(1.into(), 0.into());
        let wrong = Vec2::ZERO;

        assert_eq!(x.get_normalized(), y);
        assert_eq!(wrong.try_get_normalized(), None)
    }
}

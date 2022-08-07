use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};
use fixed::types::I48F16;
use fixed_sqrt::*;

pub type SignedFractional = I48F16;


#[derive(Eq, PartialEq, Debug, Default, Hash)]
pub struct Vec2 {
    pub x: SignedFractional,
    pub y: SignedFractional,
}

impl Vec2 {
    pub fn new(x: SignedFractional, y: SignedFractional) -> Self {
        Self { x, y }
    }

    pub fn len_pow2(&self) -> SignedFractional {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> SignedFractional {
        self.len_pow2().sqrt()
    }
}

impl From<(SignedFractional, SignedFractional)> for Vec2 {
    fn from(n: (SignedFractional, SignedFractional)) -> Self {
        Self {x: n.0, y: n.1}
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
        Self { x: -self.x, y: -self.y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
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

impl Mul<SignedFractional> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: SignedFractional) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<SignedFractional> for Vec2 {
    fn mul_assign(&mut self, rhs: SignedFractional) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[cfg(test)]
mod test {
    use crate::vectors::{SignedFractional, Vec2};

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

        assert_eq!(x, y)
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
    fn scalar() {
        let x = Vec2::new(3.into(), 4.into());
        let y = Vec2::new(6.into(), 8.into());

        assert_eq!(x * 2.into(), y)
    }
}
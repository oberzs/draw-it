// Oliver Berzs
// https://github.com/oberzs/duku

use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use super::Vector2;
use crate::renderer::Color;

/// 3-component Vector.
///
/// Used for 3D sizing and positioning.
///
/// # Example
///
/// ```ignore
/// let point_1 = Vector3::new(-10.0, -10.0, -10.0);
/// let point_2 = Vector3::new(10.0, 10.0, 10.0);
///
/// target.draw_line_debug(point_1, point_2);
/// ```
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    /// the X component
    pub x: f32,
    /// the Y component
    pub y: f32,
    /// the Z component
    pub z: f32,
}

impl Vector3 {
    /// Create a new vector
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Create a new vector with all components
    /// with the same value
    pub const fn uniform(v: f32) -> Self {
        Self::new(v, v, v)
    }

    /// Calculate the dot-product of the vector
    pub fn dot(&self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    /// Calculate the cross-product of the vector
    pub fn cross(&self, other: impl Into<Self>) -> Self {
        let o = other.into();
        let x = self.y * o.z - self.z * o.y;
        let y = self.z * o.x - self.x * o.z;
        let z = self.x * o.y - self.y * o.x;
        Self::new(x, y, z)
    }

    /// Calculate the squared length of a vector
    ///
    /// Can sometimes use this instead of
    /// [length](crate::math::Vector3::length),
    /// because this is faster.
    pub fn sqr_length(&self) -> f32 {
        self.dot(*self)
    }

    /// Calculate the length of a vector
    pub fn length(&self) -> f32 {
        self.sqr_length().sqrt()
    }

    /// Calculate the unit vector
    ///
    /// The unit vector is of length 1 and can also be
    /// thought of as the direction of the vector.
    pub fn unit(&self) -> Self {
        let scale = 1.0 / self.length();
        *self * if scale.is_infinite() { 0.0 } else { scale }
    }

    /// Calculate the angle between 2 vectors
    ///
    /// Note: resulting angle is in degrees
    ///
    /// # Example
    ///
    /// ```ignore
    /// let up = Vector3::UP;
    /// let right = Vector3::RIGHT;
    /// let angle = up.angle_between(right);
    /// // angle is ~90 degrees
    /// ```
    pub fn angle_between(&self, other: impl Into<Self>) -> f32 {
        let o = other.into();
        let cos = self.dot(o) / (self.length() * o.length());
        cos.acos().to_degrees()
    }

    /// Calculate the projected vector onto some other vector
    pub fn project_onto(&self, other: impl Into<Self>) -> Self {
        let o = other.into();
        let projected_length = self.dot(o) / o.length();
        o.unit() * projected_length
    }

    /// Get the [Vector2](crate::math::Vector2)
    /// made from this vectors x and y
    pub const fn xy(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    /// Floor every component of the vector
    pub fn floor(&self) -> Self {
        Vector3::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    /// Ceil every component of the vector
    pub fn ceil(&self) -> Self {
        Vector3::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }

    /// Round every component of the vector
    pub fn round(&self) -> Self {
        Vector3::new(self.x.round(), self.y.round(), self.z.round())
    }

    /// Shorthand for `Vector3::new(0.0, 0.0, -1.0)`
    pub const BACK: Self = Self::new(0.0, 0.0, -1.0);
    /// Shorthand for `Vector3::new(0.0, 0.0, 1.0)`
    pub const FORWARD: Self = Self::new(0.0, 0.0, 1.0);
    /// Shorthand for `Vector3::new(0.0, 1.0, 0.0)`
    pub const UP: Self = Self::new(0.0, 1.0, 0.0);
    /// Shorthand for `Vector3::new(0.0, -1.0, 0.0)`
    pub const DOWN: Self = Self::new(0.0, -1.0, 0.0);
    /// Shorthand for `Vector3::new(-1.0, 0.0, 0.0)`
    pub const LEFT: Self = Self::new(-1.0, 0.0, 0.0);
    /// Shorthand for `Vector3::new(1.0, 0.0, 0.0)`
    pub const RIGHT: Self = Self::new(1.0, 0.0, 0.0);
}

impl<N: Into<f32> + Copy> From<[N; 3]> for Vector3 {
    fn from(a: [N; 3]) -> Self {
        Self::new(a[0].into(), a[1].into(), a[2].into())
    }
}

impl From<(Vector2, f32)> for Vector3 {
    fn from(v: (Vector2, f32)) -> Self {
        Self::new(v.0.x, v.0.y, v.1)
    }
}

impl From<Color> for Vector3 {
    fn from(c: Color) -> Self {
        Self::from(c.to_rgb_norm())
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range {}", index),
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add<Self> for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'vec> Sum<&'vec Vector3> for Vector3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'vec Self>,
    {
        iter.fold(Self::default(), |a, b| a + *b)
    }
}

impl Sub<Self> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl AddAssign<Self> for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign<Self> for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test {
    use super::Vector2;
    use super::Vector3;

    #[test]
    fn default() {
        let v = Vector3::default();
        assert_eq_delta!(v.x, 0.0);
        assert_eq_delta!(v.y, 0.0);
        assert_eq_delta!(v.z, 0.0);
    }

    #[test]
    fn new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq_delta!(v.x, 1.0);
        assert_eq_delta!(v.y, 2.0);
        assert_eq_delta!(v.z, 3.0);
    }

    #[test]
    fn dot() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(5.0, 6.0, 7.0);
        assert_eq_delta!(a.dot(b), 38.0);
    }

    #[test]
    fn cross() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = Vector3::new(5.0, 6.0, 7.0);
        let r = a.cross(b);
        assert_eq_delta!(r.x, -3.0);
        assert_eq_delta!(r.y, 6.0);
        assert_eq_delta!(r.z, -3.0);
    }

    #[test]
    fn length() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        assert_eq_delta!(v.length(), 5.0);
    }

    #[test]
    fn unit() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let u = v.unit();
        assert_eq_delta!(u.x, 0.6);
        assert_eq_delta!(u.y, 0.8);
        assert_eq_delta!(u.z, 0.0);
    }

    #[test]
    fn angle_between() {
        let a = Vector3::new(4.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 13.0, 0.0);
        assert_eq_delta!(a.angle_between(b), 90.0);
    }

    #[test]
    fn xy() {
        let v = Vector3::new(1.0, 3.0, 2.0);
        assert_eq!(v.xy(), Vector2::new(1.0, 3.0));
    }

    #[test]
    fn direction() {
        assert_eq!(Vector3::FORWARD, Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(Vector3::BACK, Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(Vector3::DOWN, Vector3::new(0.0, -1.0, 0.0));
        assert_eq!(Vector3::UP, Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(Vector3::RIGHT, Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(Vector3::LEFT, Vector3::new(-1.0, 0.0, 0.0));
    }

    #[test]
    fn operator() {
        let v1 = Vector3::new(2.0, 3.0, 4.0);
        let v2 = Vector3::new(2.0, 8.0, 4.0);
        assert_eq!(-v1, Vector3::new(-2.0, -3.0, -4.0));
        assert_eq!(v1 + v2, Vector3::new(4.0, 11.0, 8.0));
        assert_eq!(v1 - v2, Vector3::new(0.0, -5.0, 0.0));
        assert_eq!(v1 * 4.0, Vector3::new(8.0, 12.0, 16.0));
        assert_eq!(v2 / 2.0, Vector3::new(1.0, 4.0, 2.0));
    }
}

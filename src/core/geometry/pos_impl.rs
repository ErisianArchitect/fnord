use crate::core::math::{
    lerp,
};
use super::size_impl::*;
use super::dims_impl::*;
use super::rect_impl::*;
use std::ops::{
    Add, Sub,
    Mul, Div, Rem,
    Neg,
    Index, IndexMut,
    Deref, DerefMut,
};

use std::borrow::{Borrow, BorrowMut};

/// Represents a position in 2D space.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

#[inline]
pub const fn pos(x: f32, y: f32) -> Pos {
    Pos { x, y }
}

impl Pos {
    pub const ZERO:        Self = Self::new(0.0, 0.0);
    pub const NEG_HALF:    Self = Self::new(-0.5, -0.5);
    pub const HALF:        Self = Self::new(0.5, 0.5);
    pub const NEG_ONE:     Self = Self::new(-1.0, -1.0);
    pub const ONE:         Self = Self::new(1.0, 1.0);
    pub const NEG_X:       Self = Self::new(-1.0, 0.0);
    pub const X:           Self = Self::new(1.0, 0.0);
    pub const NEG_Y:       Self = Self::new(0.0, -1.0);
    pub const Y:           Self = Self::new(0.0, 1.0);
    pub const NEG_X_POS_Y: Self = Self::new(-1.0, 1.0);
    pub const POS_X_NEG_Y: Self = Self::new(1.0, -1.0);

    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(splat: f32) -> Self {
        Self { x: splat, y: splat }
    }

    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self { x: cos, y: sin }
    }

    #[inline]
    pub const fn rect(self, size: Size) -> Rect {
        Rect::from_min_size(self, size)
    }

    #[inline]
    pub const fn centered_rect(self, size: Size) -> Rect {
        Rect::centered(self, size)
    }

    #[inline]
    pub const fn square(self, size: f32) -> Rect {
        Rect::square_from_min_size(self, size)
    }

    #[inline]
    pub const fn centered_square(self, size: f32) -> Rect {
        Rect::centered_square(self, size)
    }

    #[inline]
    pub const fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub const fn distance_squared(self, other: Pos) -> f32 {
        other.sub_dims(self.x, self.y).length_squared()
    }

    #[inline]
    pub fn distance(self, other: Pos) -> f32 {
        self.distance_squared(other).sqrt()
    }

    #[inline]
    pub fn angle(self) -> f32 {
        <f32>::atan2(self.y, self.x)
    }

    #[inline]
    pub fn perp(self) -> Self {
        Self::new(-self.y, self.x)
    }

    #[inline]
    pub fn rotate(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.y * rhs.x + self.x * rhs.y,
        }
    }

    #[inline]
    pub const fn add_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x + x, self.y + y)
    }

    #[inline]
    pub const fn sub_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x - x, self.y - y)
    }

    #[inline]
    pub const fn mul_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x * x, self.y * y)
    }

    #[inline]
    pub const fn div_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x / x, self.y / y)
    }

    #[inline]
    pub const fn rem_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x % x, self.y % y)
    }

    #[inline]
    pub fn rem_euclid_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x.rem_euclid(x), self.y.rem_euclid(y))
    }

    #[inline]
    pub const fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    #[inline]
    pub const fn negated(self) -> Self {
        Self::new(-self.x, -self.y)
    }

    #[inline]
    pub const fn to_tuple(self) -> (f32, f32) {
        (self.x, self.y)
    }

    #[inline]
    pub const fn from_tuple((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }

    #[inline]
    pub const fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub const fn from_array([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
    }

    #[inline]
    pub const fn as_slice<'a>(&'a self) -> &'a [f32] {
        unsafe {
            std::slice::from_raw_parts(self as *const Self as *const f32, 2)
        }
    }

    #[inline]
    pub const fn as_mut_slice<'a>(&'a mut self) -> &'a mut [f32] {
        unsafe {
            std::slice::from_raw_parts_mut(self as *mut Self as *mut f32, 2)
        }
    }

    /// Returns the min of both components.
    #[inline]
    pub const fn min(self, rhs: Self) -> Self {
        Self::new(
            self.x.min(rhs.x),
            self.y.min(rhs.y),
        )
    }

    /// Returns the max of both components.
    #[inline]
    pub const fn max(self, rhs: Self) -> Self {
        Self::new(
            self.x.max(rhs.x),
            self.y.max(rhs.y),
        )
    }

    #[inline]
    pub const fn min_max(self, rhs: Self) -> (Self, Self) {
        (
            self.min(rhs),
            rhs.max(self),
        )
    }

    #[inline]
    pub const fn lerp(self, other: Self, t: f32) -> Self {
        Self::new(
            lerp(self.x, other.x, t),
            lerp(self.y, other.y, t),
        )
    }

    #[inline]
    pub const fn mid_point(self, other: Self) -> Self {
        Self::new(
            self.x.midpoint(other.x),
            self.y.midpoint(other.y),
        )
    }

    #[inline]
    pub const fn clamped_lerp(self, other: Self, t: f32) -> Self {
        self.lerp(other, t.clamp(0.0, 1.0))
    }

    #[inline]
    pub const fn clamp(self, min: Pos, max: Pos) -> Self {
        Self::new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
        )
    }

    #[inline]
    pub const fn clamp_both(self, min: f32, max: f32) -> Self {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
        )
    }

    #[inline]
    pub const fn clamp_uv(self) -> Self {
        Self::clamp_both(self, 0.0, 1.0)
    }

    #[inline]
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        let length = self.length();
        if length >= min && length <= max {
            return self;
        }
        let clamped_length = length.clamp(min, max);
        let mult = clamped_length / length;
        Self::new(self.x * mult, self.y * mult)
    }

    #[inline]
    pub fn clamp_length_min(self, min: f32) -> Self {
        let length = self.length();
        if length >= min {
            return self;
        }
        let clamped_length = length.max(min);
        let mult = clamped_length / length;
        Self::new(self.x * mult, self.y * mult)
    }

    #[inline]
    pub fn clamp_length_max(self, max: f32) -> Self {
        let length = self.length();
        if length <= max {
            return self;
        }
        let clamped_length = length.min(max);
        let mult = clamped_length / length;
        Self::new(self.x * mult, self.y * mult)
    }

    #[inline]
    pub const fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }

    #[inline]
    pub const fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let length = self.length();
        Self::new(self.x / length, self.y / length)
    }

    #[inline]
    pub const fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    #[inline]
    pub fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }

    #[inline]
    pub fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    #[inline]
    pub fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    #[inline]
    pub fn fract(self) -> Self {
        Self::new(self.x.fract(), self.y.fract())
    }

    #[inline]
    pub fn map<R, F: FnOnce(f32, f32) -> R>(self, map: F) -> R {
        map(self.x, self.y)
    }

    // Comparisons

    /// Checks that `self.x < other.x` and `self.y < other.y`.
    #[inline]
    pub const fn lt(self, other: Pos) -> bool {
        self.x < other.x && self.y < other.y
    }

    /// Checks that `self.x <= other.x` and `self.y <= other.y`.
    #[inline]
    pub const fn le(self, other: Pos) -> bool {
        self.x <= other.x && self.y <= other.y
    }

    /// Checks that `self.x == other.x` and `self.y == other.y`.
    #[inline]
    pub const fn eq(self, other: Pos) -> bool {
        self.x == other.x && self.y == other.y
    }

    /// Checks that `self.x >= other.x` and `self.y >= other.y`.
    #[inline]
    pub const fn ge(self, other: Pos) -> bool {
        self.x >= other.x && self.y >= other.y
    }

    /// Checks that `self.x > other.x` and `self.y > other.y`.
    #[inline]
    pub const fn gt(self, other: Pos) -> bool {
        self.x > other.x && self.y > other.y
    }

    #[inline]
    pub const fn dims(self) -> Dims {
        unsafe {
            std::mem::transmute(self)
        }
    }

    #[inline]
    pub const fn from_dims(dims: Dims) -> Self {
        unsafe {
            std::mem::transmute(dims)
        }
    }
}

impl std::cmp::PartialOrd<Pos> for Pos {
    fn ge(&self, other: &Pos) -> bool {
        Pos::ge(*self, *other)
    }

    fn gt(&self, other: &Pos) -> bool {
        Pos::gt(*self, *other)
    }

    fn le(&self, other: &Pos) -> bool {
        Pos::le(*self, *other)
    }

    fn lt(&self, other: &Pos) -> bool {
        Pos::lt(*self, *other)
    }

    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        let lt = Pos::lt(*self, *other);
        let gt = Pos::gt(*self, *other);
        let eq = Pos::eq(*self, *other);
        Some(match (lt, eq, gt) {
            (true, true, true) => unreachable!(),
            (true, true, false) => unreachable!(),
            (true, false, true) => unreachable!(),
            (true, false, false) => std::cmp::Ordering::Less,
            (false, true, true) => unreachable!(),
            (false, true, false) => std::cmp::Ordering::Equal,
            (false, false, true) => std::cmp::Ordering::Greater,
            (false, false, false) => return None,
        })
    }
}

impl Deref for Pos {
    type Target = Dims;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self as *const Self as *const Dims)
        }
    }
}

impl DerefMut for Pos {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *(self as *mut Self as *mut Dims)
        }
    }
}

impl AsRef<Dims> for Pos {
    #[inline]
    fn as_ref(&self) -> &Dims {
        &*self
    }
}

impl AsMut<Dims> for Pos {
    #[inline]
    fn as_mut(&mut self) -> &mut Dims {
        &mut *self
    }
}

impl Borrow<Dims> for Pos {
    #[inline]
    fn borrow(&self) -> &Dims {
        &*self
    }
}

impl BorrowMut<Dims> for Pos {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Dims {
        &mut *self
    }
}

impl Index<usize> for Pos {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds."),
        }
    }
}

impl IndexMut<usize> for Pos {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds."),
        }
    }
}

impl Neg for Pos {
    type Output = Pos;
    #[inline]
    fn neg(self) -> Self::Output {
        self.negated()
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;
    #[inline]
    fn add(self, rhs: Pos) -> Self::Output {
        self.add_dims(rhs.x, rhs.y)
    }
}

impl Add<Size> for Pos {
    type Output = Pos;
    #[inline]
    fn add(self, rhs: Size) -> Self::Output {
        self.add_dims(rhs.width, rhs.height)
    }
}

impl Add<(f32, f32)> for Pos {
    type Output = Pos;
    #[inline]
    fn add(self, rhs: (f32, f32)) -> Self::Output {
        self.add_dims(rhs.0, rhs.1)
    }
}

impl Add<[f32; 2]> for Pos {
    type Output = Pos;
    #[inline]
    fn add(self, rhs: [f32; 2]) -> Self::Output {
        self.add_dims(rhs[0], rhs[1])
    }
}

impl Add<f32> for Pos {
    type Output = Pos;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        self.add_dims(rhs, rhs)
    }
}

impl Sub<Pos> for Pos {
    type Output = Pos;
    #[inline]
    fn sub(self, rhs: Pos) -> Self::Output {
        self.sub_dims(rhs.x, rhs.y)
    }
}

impl Sub<Size> for Pos {
    type Output = Pos;
    #[inline]
    fn sub(self, rhs: Size) -> Self::Output {
        self.sub_dims(rhs.width, rhs.height)
    }
}

impl Sub<(f32, f32)> for Pos {
    type Output = Pos;
    #[inline]
    fn sub(self, (x, y): (f32, f32)) -> Self::Output {
        self.sub_dims(x, y)
    }
}

impl Sub<[f32; 2]> for Pos {
    type Output = Pos;
    #[inline]
    fn sub(self, [x, y]: [f32; 2]) -> Self::Output {
        self.sub_dims(x, y)
    }
}

impl Sub<f32> for Pos {
    type Output = Pos;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        self.sub_dims(rhs, rhs)
    }
}

impl Mul<Pos> for Pos {
    type Output = Pos;
    #[inline]
    fn mul(self, rhs: Pos) -> Self::Output {
        self.mul_dims(rhs.x, rhs.y)
    }
}

impl Mul<Size> for Pos {
    type Output = Pos;
    #[inline]
    fn mul(self, rhs: Size) -> Self::Output {
        self.mul_dims(rhs.width, rhs.height)
    }
}

impl Mul<(f32, f32)> for Pos {
    type Output = Pos;
    #[inline]
    fn mul(self, (x, y): (f32, f32)) -> Self::Output {
        self.mul_dims(x, y)
    }
}

impl Mul<[f32; 2]> for Pos {
    type Output = Pos;
    #[inline]
    fn mul(self, [x, y]: [f32; 2]) -> Self::Output {
        self.mul_dims(x, y)
    }
}

impl Mul<f32> for Pos {
    type Output = Pos;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.mul_dims(rhs, rhs)
    }
}

impl Div<Pos> for Pos {
    type Output = Pos;
    #[inline]
    fn div(self, rhs: Pos) -> Self::Output {
        self.div_dims(rhs.x, rhs.y)
    }
}

impl Div<Size> for Pos {
    type Output = Pos;
    #[inline]
    fn div(self, rhs: Size) -> Self::Output {
        self.div_dims(rhs.width, rhs.height)
    }
}

impl Div<(f32, f32)> for Pos {
    type Output = Pos;
    #[inline]
    fn div(self, (x, y): (f32, f32)) -> Self::Output {
        self.div_dims(x, y)
    }
}

impl Div<[f32; 2]> for Pos {
    type Output = Pos;
    #[inline]
    fn div(self, [x, y]: [f32; 2]) -> Self::Output {
        self.div_dims(x, y)
    }
}

impl Div<f32> for Pos {
    type Output = Pos;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.div_dims(rhs, rhs)
    }
}

impl Rem<Pos> for Pos {
    type Output = Pos;
    #[inline]
    fn rem(self, rhs: Pos) -> Self::Output {
        self.rem_dims(rhs.x, rhs.y)
    }
}

impl Rem<Size> for Pos {
    type Output = Pos;
    #[inline]
    fn rem(self, rhs: Size) -> Self::Output {
        self.rem_dims(rhs.width, rhs.height)
    }
}

impl Rem<(f32, f32)> for Pos {
    type Output = Pos;
    #[inline]
    fn rem(self, (x, y): (f32, f32)) -> Self::Output {
        self.rem_dims(x, y)
    }
}

impl Rem<[f32; 2]> for Pos {
    type Output = Pos;
    #[inline]
    fn rem(self, [x, y]: [f32; 2]) -> Self::Output {
        self.rem_dims(x, y)
    }
}

impl Rem<f32> for Pos {
    type Output = Pos;
    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        self.rem_dims(rhs, rhs)
    }
}
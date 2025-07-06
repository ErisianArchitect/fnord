use crate::core::geometry::{normalize_angle, Axial, Cardinal};
use crate::core::math::{
    lerp,
};
use super::size_impl::*;
use super::dims_impl::*;
use super::rect_impl::*;
use std::f32::consts::*;
use std::num::FpCategory;
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

    #[must_use]
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    #[inline]
    pub const fn splat(splat: f32) -> Self {
        Self { x: splat, y: splat }
    }

    #[must_use]
    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self { x: cos, y: sin }
    }

    #[must_use]
    #[inline]
    pub const fn rect(self, size: Size) -> Rect {
        Rect::from_min_size(self, size)
    }

    #[must_use]
    #[inline]
    pub const fn centered_rect(self, size: Size) -> Rect {
        Rect::centered(self, size)
    }

    #[must_use]
    #[inline]
    pub const fn square(self, size: f32) -> Rect {
        Rect::square_from_min_size(self, size)
    }

    #[must_use]
    #[inline]
    pub const fn centered_square(self, size: f32) -> Rect {
        Rect::centered_square(self, size)
    }

    #[must_use]
    #[inline]
    pub const fn x(self) -> f32 {
        self.x
    }

    #[inline]
    pub const fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    #[must_use]
    #[inline]
    pub const fn with_x(mut self, x: f32) -> Self {
        self.set_x(x);
        self
    }

    #[must_use]
    #[inline]
    pub const fn y(self) -> f32 {
        self.y
    }

    #[inline]
    pub const fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    #[must_use]
    #[inline]
    pub const fn with_y(mut self, y: f32) -> Self {
        self.set_y(y);
        self
    }

    /// Returns a self with the x and y swapped.
    #[must_use]
    #[inline]
    pub const fn yx(self) -> Self {
        Pos::new(self.y, self.x)
    }

    /// Sets `(self.x, self.y)` to `(yx.y, yx.x)`.
    #[must_use]
    #[inline]
    pub const fn set_yx(&mut self, yx: Pos) {
        self.x = yx.y;
        self.y = yx.x;
    }

    #[must_use]
    #[inline]
    pub const fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[must_use]
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

    /// Returns the angle in radians. (`x` right, `y` down)
    #[inline]
    pub fn angle(self) -> f32 {
        <f32>::atan2(-self.y, self.x)
    }

    #[inline]
    pub fn normalized_angle(self) -> f32 {
        normalize_angle(self.angle())
    }

    /// Rotates the [Pos] 90 degrees to the right.
    #[inline]
    pub fn perp_cw(self) -> Self {
        // (0.5, 0.25) becomes (-0.25, 0.5)
        Self::new(-self.y, self.x)
    }

    /// Rotates the [Pos] 90 degrees to the left.
    #[inline]
    pub fn perp_ccw(self) -> Self {
        Self::new(self.y, self.x)
    }

    /// Assumes that `self` and `normal` are normalized.
    #[must_use]
    #[inline]
    pub const fn reflect(self, normal: Self) -> Self {
        self.sub(self.mul(normal).mul_dims(2.0, 2.0).mul(normal))
    }

    /// Assumes that both `self` and `rhs` are unit vectors, and rotates `self` by the rotation of `rhs`.
    #[inline]
    pub fn rotate_by(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.y * rhs.x + self.x * rhs.y,
        }
    }

    #[must_use]
    #[inline]
    pub const fn add_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x + x, self.y + y)
    }

    #[must_use]
    #[inline]
    pub const fn add(self, rhs: Self) -> Self {
        self.add_dims(rhs.x, rhs.y)
    }

    #[must_use]
    #[inline]
    pub const fn sub_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x - x, self.y - y)
    }

    #[must_use]
    #[inline]
    pub const fn sub(self, rhs: Self) -> Self {
        self.sub_dims(rhs.x, rhs.y)
    }

    #[must_use]
    #[inline]
    pub const fn mul_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x * x, self.y * y)
    }

    #[must_use]
    #[inline]
    pub const fn mul(self, rhs: Self) -> Self {
        self.mul_dims(rhs.x, rhs.y)
    }

    #[must_use]
    #[inline]
    pub fn mul_add_dims(self, mul_x: f32, mul_y: f32, add_x: f32, add_y: f32) -> Self {
        Self::new(
            self.x.mul_add(mul_x, add_x),
            self.y.mul_add(mul_y, add_y)
        )
    }

    #[must_use]
    #[inline]
    pub fn mul_add(self, mul: Self, add: Self) -> Self {
        self.mul_add_dims(mul.x, mul.y, add.x, add.y)
    }

    #[must_use]
    #[inline]
    pub const fn div_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x / x, self.y / y)
    }

    #[must_use]
    #[inline]
    pub const fn div(self, rhs: Self) -> Self {
        self.div_dims(rhs.x, rhs.y)
    }

    #[must_use]
    #[inline]
    pub const fn rem_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x % x, self.y % y)
    }

    #[must_use]
    #[inline]
    pub const fn rem(self, rhs: Self) -> Self {
        self.rem_dims(rhs.x, rhs.y)
    }

    #[must_use]
    #[inline]
    pub fn rem_euclid_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x.rem_euclid(x), self.y.rem_euclid(y))
    }

    #[must_use]
    #[inline]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        self.rem_euclid_dims(rhs.x, rhs.y)
    }

    #[must_use]
    #[inline]
    pub fn div_euclid_dims(self, x: f32, y: f32) -> Self {
        Self::new(self.x.div_euclid(x), self.y.div_euclid(y))
    }

    #[must_use]
    #[inline]
    pub fn div_euclid(self, rhs: Self) -> Self {
        self.div_euclid_dims(rhs.x, rhs.y)
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
    pub fn floor(self) -> Self {
        Self::new(
            self.x.floor(),
            self.y.floor()
        )
    }

    #[inline]
    pub fn ceil(self) -> Self {
        Self::new(
            self.x.ceil(),
            self.y.ceil()
        )
    }

    #[inline]
    pub fn round(self) -> Self {
        Self::new(
            self.x.round(),
            self.y.round()
        )
    }

    #[inline]
    pub fn rount_ties_even(self) -> Self {
        Self::new(
            self.x.round_ties_even(),
            self.y.round_ties_even()
        )
    }

    #[must_use]
    #[inline]
    pub const fn to_degrees(self) -> Self {
        Self::new(
            self.x.to_degrees(),
            self.y.to_degrees()
        )
    }

    #[must_use]
    #[inline]
    pub const fn to_radians(self) -> Self {
        Self::new(
            self.x.to_radians(),
            self.y.to_radians()
        )
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self::new(
            self.x.abs(),
            self.y.abs()
        )
    }

    #[inline]
    pub fn cbrt(self) -> Self {
        Self::new(
            self.x.cbrt(),
            self.y.cbrt()
        )
    }

    #[must_use]
    #[inline]
    pub const fn copysign(self, sign: Pos) -> Self {
        Self::new(
            self.x.copysign(sign.x),
            self.y.copysign(sign.y)
        )
    }

    #[must_use]
    #[inline]
    pub const fn copysign2(self, sign: f32) -> Self {
        Self::new(
            self.x.copysign(sign),
            self.y.copysign(sign)
        )
    }

    #[must_use]
    #[inline]
    pub fn copy_x_sign(self) -> Self {
        Self::new(
            self.x,
            self.y.copysign(self.x)
        )
    }

    #[must_use]
    pub fn copy_y_sign(self) -> Self {
        Self::new(
            self.x.copysign(self.y),
            self.y
        )
    }

    #[must_use]
    #[inline]
    pub fn exp(self) -> Self {
        Self::new(
            self.x.exp(),
            self.y.exp()
        )
    }

    #[must_use]
    #[inline]
    pub fn exp2(self) -> Self {
        Self::new(
            self.x.exp2(),
            self.y.exp2()
        )
    }

    #[must_use]
    #[inline]
    pub fn atan2_xy(self) -> f32 {
        self.x.atan2(self.y)
    }

    #[must_use]
    #[inline]
    pub fn atan2_yx(self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Snaps to the nearest point on a [Rect].
    #[must_use]
    #[inline]
    pub fn snap_to_rect(self, rect: Rect) -> Pos {
        rect.closest_point(self)
    }

    #[must_use]
    #[inline]
    pub fn next_up(self) -> Self {
        Self::new(
            self.x.next_up(),
            self.y.next_up()
        )
    }

    #[must_use]
    #[inline]
    pub fn signum(self) -> Self {
        Self::new(
            self.x.signum(),
            self.y.signum()
        )
    }

    #[must_use]
    #[inline]
    pub fn recip(self) -> Self {
        Self::new(
            self.x.recip(),
            self.y.recip()
        )
    }

    #[must_use]
    #[inline]
    pub const fn classify(self) -> [FpCategory; 2] {
        [
            self.x.classify(),
            self.y.classify(),
        ]
    }

    // also, XOR, and versions returning [bool; 2]

    #[must_use]
    #[inline]
    pub const fn is_finite(self) -> [bool; 2] {
        [
            self.x.is_finite(),
            self.y.is_finite(),
        ]
    }

    #[must_use]
    #[inline]
    pub const fn is_finite_or(self) -> bool {
        self.x.is_finite() || self.y.is_finite()
    }

    #[must_use]
    #[inline]
    pub const fn is_finite_and(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[must_use]
    #[inline]
    pub const fn is_finite_xor(self) -> bool {
        self.x.is_finite() ^ self.y.is_finite()
    }

    #[must_use]
    #[inline]
    pub const fn is_finite_nor(self) -> bool {
        !self.is_finite_or()
    }

    #[must_use]
    #[inline]
    pub const fn is_finite_nand(self) -> bool {
        !self.is_finite_and()
    }

    #[must_use]
    #[inline]
    pub const fn is_finite_xnor(self) -> bool {
        self.x.is_finite() == self.y.is_finite()
    }

    #[must_use]
    #[inline]
    pub const fn is_infinite(self) -> [bool; 2] {
        [
            self.x.is_infinite(),
            self.y.is_infinite(),
        ]
    }

    #[must_use]
    #[inline]
    pub const fn is_infinite_or(self) -> bool {
        self.x.is_infinite() || self.y.is_infinite()
    }

    #[must_use]
    #[inline]
    pub const fn is_infinite_and(self) -> bool {
        self.x.is_infinite() && self.y.is_infinite()
    }

    #[must_use]
    #[inline]
    pub const fn is_infinite_xor(self) -> bool {
        self.x.is_infinite() ^ self.y.is_infinite()
    }

    #[must_use]
    #[inline]
    pub const fn is_infinite_nor(self) -> bool {
        !self.is_infinite_or()
    }

    #[must_use]
    #[inline]
    pub const fn is_infinite_nand(self) -> bool {
        !self.is_infinite_and()
    }

    #[must_use]
    #[inline]
    pub const fn is_infinite_xnor(self) -> bool {
        self.x.is_infinite() == self.y.is_infinite()
    }

    #[must_use]
    #[inline]
    pub const fn is_nan(self) -> [bool; 2] {
        [
            self.x.is_nan(),
            self.y.is_nan(),
        ]
    }

    #[must_use]
    #[inline]
    pub const fn is_nan_or(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    #[must_use]
    #[inline]
    pub const fn is_nan_and(self) -> bool {
        self.x.is_nan() && self.y.is_nan()
    }

    #[must_use]
    #[inline]
    pub const fn is_nan_xor(self) -> bool {
        self.x.is_nan() ^ self.y.is_nan()
    }

    #[must_use]
    #[inline]
    pub const fn is_nan_nor(self) -> bool {
        !self.is_nan_or()
    }

    #[must_use]
    #[inline]
    pub const fn is_nan_nand(self) -> bool {
        !self.is_nan_and()
    }

    #[must_use]
    #[inline]
    pub const fn is_nan_xnor(self) -> bool {
        self.x.is_nan() == self.y.is_nan()
    }

    #[must_use]
    #[inline]
    pub const fn is_normal(self) -> [bool; 2] {
        [
            self.x.is_normal(),
            self.y.is_normal(),
        ]
    }

    #[must_use]
    #[inline]
    pub const fn is_normal_or(self) -> bool {
        self.x.is_normal() || self.y.is_normal()
    }

    #[must_use]
    #[inline]
    pub const fn is_normal_and(self) -> bool {
        self.x.is_normal() && self.y.is_normal()
    }

    #[must_use]
    pub const fn is_normal_xor(self) -> bool {
        self.x.is_normal() ^ self.y.is_normal()
    }

    #[must_use]
    #[inline]
    pub const fn is_normal_nor(self) -> bool {
        !self.is_normal_or()
    }

    #[must_use]
    #[inline]
    pub const fn is_normal_nand(self) -> bool {
        !self.is_normal_and()
    }

    #[must_use]
    #[inline]
    pub const fn is_normal_xnor(self) -> bool {
        self.x.is_normal() == self.y.is_normal()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_negative(self) -> [bool; 2] {
        [
            self.x.is_sign_negative(),
            self.y.is_sign_negative(),
        ]
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_negative_or(self) -> bool {
        self.x.is_sign_negative() || self.y.is_sign_negative()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_negative_and(self) -> bool {
        self.x.is_sign_negative() && self.y.is_sign_negative()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_negative_xor(self) -> bool {
        self.x.is_sign_negative() ^ self.y.is_sign_negative()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_negative_nor(self) -> bool {
        !self.is_sign_negative_or()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_negative_nand(self) -> bool {
        !self.is_sign_negative_and()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_negative_xnor(self) -> bool {
        self.x.is_sign_negative() == self.y.is_sign_negative()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_positive(self) -> [bool; 2] {
        [
            self.x.is_sign_positive(),
            self.y.is_sign_positive(),
        ]
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_positive_or(self) -> bool {
        self.x.is_sign_positive() || self.y.is_sign_positive()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_positive_and(self) -> bool {
        self.x.is_sign_positive() && self.y.is_sign_positive()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_positive_xor(self) -> bool {
        self.x.is_sign_positive() ^ self.y.is_sign_positive()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_positive_nor(self) -> bool {
        !self.is_sign_positive_or()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_positive_nand(self) -> bool {
        !self.is_sign_positive_and()
    }

    #[must_use]
    #[inline]
    pub const fn is_sign_positive_xnor(self) -> bool {
        self.x.is_sign_positive() == self.y.is_sign_positive()
    }

    #[must_use]
    #[inline]
    pub const fn is_subnormal(self) -> [bool; 2] {
        [
            self.x.is_subnormal(),
            self.y.is_subnormal(),
        ]
    }

    #[must_use]
    #[inline]
    pub const fn is_subnormal_or(self) -> bool {
        self.x.is_subnormal() || self.y.is_subnormal()
    }

    #[must_use]
    #[inline]
    pub const fn is_subnormal_and(self) -> bool {
        self.x.is_subnormal() && self.y.is_subnormal()
    }

    #[must_use]
    #[inline]
    pub const fn is_subnormal_xor(self) -> bool {
        self.x.is_subnormal() ^ self.y.is_subnormal()
    }

    #[must_use]
    #[inline]
    pub const fn is_subnormal_nor(self) -> bool {
        !self.is_subnormal_or()
    }

    #[must_use]
    #[inline]
    pub const fn is_subnormal_nand(self) -> bool {
        !self.is_subnormal_and()
    }

    #[must_use]
    #[inline]
    pub const fn is_subnormal_xnor(self) -> bool {
        self.x.is_subnormal() == self.y.is_subnormal()
    }

    #[must_use]
    #[inline]
    pub fn trunc(self) -> Self {
        Self::new(
            self.x.trunc(),
            self.y.trunc()
        )
    }

    #[must_use]
    #[inline]
    pub fn next_down(self) -> Self {
        Self::new(
            self.x.next_down(),
            self.y.next_down()
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
        debug_assert!(min.le(max));
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
    pub fn normalized(self) -> Self {
        let length = self.length();
        Self::new(self.x / length, self.y / length)
    }

    #[inline]
    pub fn fract(self) -> Self {
        Self::new(self.x.fract(), self.y.fract())
    }

    #[must_use]
    #[inline]
    pub fn map<R, F: FnOnce(f32, f32) -> R>(self, map: F) -> R {
        map(self.x, self.y)
    }

    #[must_use]
    #[inline]
    pub fn map_each<X: FnOnce(f32) -> f32, Y: FnOnce(f32) -> f32>(self, x: X, y: Y) -> Self {
        Self::new(x(self.x), y(self.y))
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

    #[inline]
    pub fn test<P: FnOnce(f32, f32) -> bool>(self, pred: P) -> bool {
        pred(self.x, self.y)
    }

    #[must_use]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 8] {
        unsafe {
            std::mem::transmute([
                self.x.to_be_bytes(),
                self.y.to_be_bytes(),
            ])
        }
    }

    #[must_use]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; 8] {
        unsafe {
            std::mem::transmute([
                self.x.to_le_bytes(),
                self.y.to_le_bytes(),
            ])
        }
    }

    #[must_use]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; 8] {
        unsafe {
            std::mem::transmute([
                self.x.to_ne_bytes(),
                self.y.to_ne_bytes(),
            ])
        }
    }

    #[must_use]
    #[inline]
    pub const fn to_bits(self) -> [u32; 2] {
        [
            self.x.to_bits(),
            self.y.to_bits(),
        ]
    }

    #[must_use]
    #[inline]
    pub fn cardinal(self) -> Cardinal {
        let theta = self.normalized_angle();
        let octant = ((normalize_angle(theta + FRAC_PI_8) / FRAC_PI_4).floor() as u8) & 0b111;
        unsafe {
            std::mem::transmute(octant)
        }
    }

    #[must_use]
    #[inline]
    pub fn axial(self) -> Axial {
        let theta = self.angle();
        let quadrant = (((theta + FRAC_PI_4) / FRAC_PI_2).floor() as u8) & 0b11;
        unsafe {
            std::mem::transmute(quadrant)
        }
    }
}

impl std::cmp::PartialOrd<Pos> for Pos {
    #[inline]
    fn ge(&self, other: &Pos) -> bool {
        Pos::ge(*self, *other)
    }

    #[inline]
    fn gt(&self, other: &Pos) -> bool {
        Pos::gt(*self, *other)
    }

    #[inline]
    fn le(&self, other: &Pos) -> bool {
        Pos::le(*self, *other)
    }

    #[inline]
    fn lt(&self, other: &Pos) -> bool {
        Pos::lt(*self, *other)
    }

    #[inline]
    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        let lt = Pos::lt(*self, *other);
        let gt = Pos::gt(*self, *other);
        let eq = Pos::eq(*self, *other);
        Some(match (lt, eq, gt) {
            // Only one condition can be true.
            (true, true, true) => unreachable!(),
            // Only one condition can be true.
            (true, true, false) => unreachable!(),
            // Only one condition can be true.
            (true, false, true) => unreachable!(),
            (true, false, false) => std::cmp::Ordering::Less,
            // Only one condition can be true.
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
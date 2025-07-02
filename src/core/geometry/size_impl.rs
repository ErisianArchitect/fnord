use std::{borrow::{Borrow, BorrowMut}, ops::{
    Add, Deref, DerefMut, Div, Index, IndexMut, Mul, Neg, Rem, Sub
}};
use crate::core::geometry::dims_impl::Dims;

use super::util_impl::*;

/// Represents width and height dimensions.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// Creates a new [Size] from the given `width` and `height`.
#[inline]
pub const fn size(width: f32, height: f32) -> Size {
    Size { width, height }
}

impl Size {
    /// (0.0, 0.0)
    pub const ZERO: Self = Self::new(0.0, 0.0);
    /// (1.0, 1.0)
    pub const ONE:  Self = Self::new(1.0, 1.0);
    /// (1.0, 0.0)
    pub const W:    Self = Self::new(1.0, 0.0);
    /// (0.0, 1.0)
    pub const H:    Self = Self::new(0.0, 1.0);
    // Resolutions
    pub const VGA: Self = Self::new(640.0, 360.0);
    pub const SD_NTSC: Self = Self::new(720.0, 480.0);
    pub const SD_PAL: Self = Self::new(720.0, 576.0);
    pub const HD: Self = Self::new(1280.0, 720.0);
    pub const WXGA: Self = Self::new(1280.0, 800.0);
    pub const FHD: Self = Self::new(1920.0, 1080.0);
    pub const QHD: Self = Self::new(2560.0, 1440.0);
    pub const DCI_2K: Self = Self::new(2048.0, 1080.0);
    pub const UHD_4K: Self = Self::new(3840.0, 2160.0);
    pub const DCI_4K: Self = Self::new(4096.0, 2160.0);
    pub const UHD_8K: Self = Self::new(7680.0, 4320.0);
    pub const DCI_8K: Self = Self::new(8192.0, 4320.0);

    /// Create a new [Size] from the given `width` and `height`.
    #[inline]
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Create a new [Size] with equal `width` and `height`.
    #[inline]
    pub const fn square(side_length: f32) -> Self {
        Self { width: side_length, height: side_length }
    }

    // This method might fail in some way.
    /// Gets the area of the [Size]. This is the width multiplied by the height.
    #[inline]
    pub const fn area(self) -> f32 {
        self.width * self.height
    }

    #[inline]
    pub const fn half(self) -> Self {
        Self::new(half(self.width), half(self.height))
    }

    #[inline]
    pub const fn half_width(self) -> f32 {
        half(self.width)
    }

    #[inline]
    pub const fn half_height(self) -> f32 {
        half(self.height)
    }

    /// Converts the [Size] into a tuple.
    #[inline]
    pub const fn to_tuple(self) -> (f32, f32) {
        (self.width, self.height)
    }

    /// Creates a [Size] from a tuple.
    #[inline]
    pub const fn from_tuple((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }

    /// Converts the [Size] into an array.
    #[inline]
    pub const fn to_array(self) -> [f32; 2] {
        [self.width, self.height]
    }

    /// Creates a [Size] from an array.
    #[inline]
    pub const fn from_array([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
    }

    /// Returns the size as a slice of [f32] where `slice[0]` is width and `slice[1]` is height.
    #[inline]
    pub const fn as_slice<'a>(&'a self) -> &'a [f32] {
        unsafe {
            std::slice::from_raw_parts(self as *const Size as *const f32, 2)
        }
    }

    /// Returns the size as a mutable slice of [f32] where `slice[0]` is width and `slice[1]` is height.
    #[inline]
    pub const fn as_mut_slice<'a>(&'a mut self) -> &'a mut [f32] {
        unsafe {
            std::slice::from_raw_parts_mut(self as *mut Size as *mut f32, 2)
        }
    }

    /// Tests if the [Size] is square (both sides are equal).
    /// 
    /// This method may be unreliable due to floating point arithemetic imprecision. Try `is_square_fuzzy` if you're dealing with precision issues.
    #[inline]
    pub const fn is_square(self) -> bool {
        self.width == self.height
    }

    /// Determines if a size is as close to being square as `error`. This method is not reliable due to floating point weirdness.
    /// I advise that you use an `error` value that is greater than the value you expect. For example, if you want within `0.1`, use `0.11` or `0.109` etc..
    #[inline]
    pub const fn is_square_fuzzy(self, error: f32) -> bool {
        (self.width.max(self.height) - self.height.min(self.width)) <= error
    }

    /// Determines if the width is greater than the height.
    #[inline]
    pub const fn is_horizontal(self) -> bool {
        self.width > self.height
    }

    /// Determines if the height is greater than the width.
    #[inline]
    pub const fn is_vertical(self) -> bool {
        self.height > self.width
    }

    #[inline]
    pub const fn aspect_ratio(self) -> f32 {
        self.width / self.height
    }

    #[inline]
    pub const fn is_positive(self) -> bool {
        self.width >= 0.0 && self.height >= 0.0
    }

    #[inline]
    pub const fn negate(self) -> Self {
        Self::new(-self.width, -self.height)
    }

    #[inline]
    pub const fn scale(self, scalar: f32) -> Self {
        Self::new(self.width * scalar, self.height * scalar)
    }

    #[inline]
    pub const fn min_dim(self) -> f32 {
        self.width.min(self.height)
    }

    #[inline]
    pub const fn max_dim(self) -> f32 {
        self.width.max(self.height)
    }

    #[inline]
    pub const fn inner_square(self) -> Self {
        let side_length = self.min_dim();
        Self::new(side_length, side_length)
    }

    /// Swaps the width and height.
    #[inline]
    pub const fn swap_dims(self) -> Size {
        Self::new(self.height, self.height)
    }

    /// Add `width` to `self.width` and `height` to `self.height`.
    #[inline]
    pub const fn add_dims(self, width: f32, height: f32) -> Self {
        Self::new(self.width + width, self.height + height)
    }

    /// Subtract `width` from `self.width` and `height` from `self.height`.
    #[inline]
    pub const fn sub_dims(self, width: f32, height: f32) -> Self {
        Self::new(self.width - width, self.height - height)
    }

    /// Multiply `width` with `self.width` and `height` with `self.height`.
    #[inline]
    pub const fn mul_dims(self, width: f32, height: f32) -> Self {
        Self::new(self.width * width, self.height * height)
    }

    /// Divide `self.width` by `width` and `self.height` by `height`.
    #[inline]
    pub const fn div_dims(self, width: f32, height: f32) -> Self {
        Self::new(self.width / width, self.height / height)
    }

    /// Division remainder of `self.width` by `width` and `self.height` by `height`.
    #[inline]
    pub const fn rem_dims(self, width: f32, height: f32) -> Self {
        Self::new(self.width % width, self.height % height)
    }
}

impl Deref for Size {
    type Target = Dims;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self as *const Self as *const Dims)
        }
    }
}

impl DerefMut for Size {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *(self as *mut Self as *mut Dims)
        }
    }
}

impl AsRef<Dims> for Size {
    #[inline]
    fn as_ref(&self) -> &Dims {
        &*self
    }
}

impl AsRef<[f32]> for Size {
    #[inline]
    fn as_ref(&self) -> &[f32] {
        self.as_slice()
    }
}

impl AsMut<Dims> for Size {
    #[inline]
    fn as_mut(&mut self) -> &mut Dims {
        &mut *self
    }
}

impl AsMut<[f32]> for Size {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32] {
        self.as_mut_slice()
    }
}

impl Borrow<Dims> for Size {
    #[inline]
    fn borrow(&self) -> &Dims {
        &*self
    }
}

impl BorrowMut<Dims> for Size {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Dims {
        &mut *self
    }
}

impl From<(f32, f32)> for Size {
    #[inline]
    fn from(value: (f32, f32)) -> Self {
        Self::from_tuple(value)
    }
}

impl Into<(f32, f32)> for Size {
    #[inline]
    fn into(self) -> (f32, f32) {
        self.to_tuple()
    }
}

impl From<[f32; 2]> for Size {
    #[inline]
    fn from(value: [f32; 2]) -> Self {
        Self::from_array(value)
    }
}

impl Into<[f32; 2]> for Size {
    #[inline]
    fn into(self) -> [f32; 2] {
        self.to_array()
    }
}

impl Index<usize> for Size {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.width,
            1 => &self.height,
            _ => panic!("Index out of bounds."),
        }
    }
}

impl IndexMut<usize> for Size {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.width,
            1 => &mut self.height,
            _ => panic!("Index out of bounds."),
        }
    }
}

impl From<f32> for Size {
    /// Converts the [f32] value into `Size::square(value)`.
    #[inline]
    fn from(value: f32) -> Self {
        Self::square(value)
    }
}

impl Neg for Size {
    type Output = Size;
    #[inline]
    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Add<Size> for Size {
    type Output = Size;
    #[inline]
    fn add(self, rhs: Size) -> Self::Output {
        self.add_dims(rhs.width, rhs.height)
    }
}

impl Add<(f32, f32)> for Size {
    type Output = Size;
    #[inline]
    fn add(self, rhs: (f32, f32)) -> Self::Output {
        self.add_dims(rhs.0, rhs.1)
    }
}

impl Add<[f32; 2]> for Size {
    type Output = Size;
    #[inline]
    fn add(self, rhs: [f32; 2]) -> Self::Output {
        self.add_dims(rhs[0], rhs[1])
    }
}

impl Add<f32> for Size {
    type Output = Size;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        self.add_dims(rhs, rhs)
    }
}

impl Sub<Size> for Size {
    type Output = Size;
    #[inline]
    fn sub(self, rhs: Size) -> Self::Output {
        self.sub_dims(rhs.width, rhs.height)
    }
}

impl Sub<(f32, f32)> for Size {
    type Output = Size;
    #[inline]
    fn sub(self, rhs: (f32, f32)) -> Self::Output {
        self.sub_dims(rhs.0, rhs.1)
    }
}

impl Sub<[f32; 2]> for Size {
    type Output = Size;
    #[inline]
    fn sub(self, rhs: [f32; 2]) -> Self::Output {
        self.sub_dims(rhs[0], rhs[1])
    }
}

impl Sub<f32> for Size {
    type Output = Size;
    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        self.sub_dims(rhs, rhs)
    }
}

impl Mul<Size> for Size {
    type Output = Size;
    #[inline]
    fn mul(self, rhs: Size) -> Self::Output {
        self.mul_dims(rhs.width, rhs.height)
    }
}

impl Mul<(f32, f32)> for Size {
    type Output = Size;
    #[inline]
    fn mul(self, rhs: (f32, f32)) -> Self::Output {
        self.mul_dims(rhs.0, rhs.1)
    }
}

impl Mul<[f32; 2]> for Size {
    type Output = Size;
    #[inline]
    fn mul(self, rhs: [f32; 2]) -> Self::Output {
        self.mul_dims(rhs[0], rhs[1])
    }
}

impl Mul<f32> for Size {
    type Output = Size;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.mul_dims(rhs, rhs)
    }
}

impl Div<Size> for Size {
    type Output = Size;
    #[inline]
    fn div(self, rhs: Size) -> Self::Output {
        self.div_dims(rhs.width, rhs.height)
    }
}

impl Div<(f32, f32)> for Size {
    type Output = Size;
    #[inline]
    fn div(self, rhs: (f32, f32)) -> Self::Output {
        self.div_dims(rhs.0, rhs.1)
    }
}

impl Div<[f32; 2]> for Size {
    type Output = Size;
    #[inline]
    fn div(self, rhs: [f32; 2]) -> Self::Output {
        self.div_dims(rhs[0], rhs[1])
    }
}

impl Div<f32> for Size {
    type Output = Size;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.div_dims(rhs, rhs)
    }
}

impl Rem<Size> for Size {
    type Output = Size;
    #[inline]
    fn rem(self, rhs: Size) -> Self::Output {
        self.rem_dims(rhs.width, rhs.height)
    }
}

impl Rem<(f32, f32)> for Size {
    type Output = Size;
    #[inline]
    fn rem(self, (x, y): (f32, f32)) -> Self::Output {
        self.rem_dims(x, y)
    }
}

impl Rem<[f32; 2]> for Size {
    type Output = Size;
    #[inline]
    fn rem(self, [x, y]: [f32; 2]) -> Self::Output {
        self.rem_dims(x, y)
    }
}

impl Rem<f32> for Size {
    type Output = Size;
    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        self.rem_dims(rhs, rhs)
    }
}
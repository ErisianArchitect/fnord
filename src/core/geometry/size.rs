
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

    /// Returns the size as a slice of [f32].
    #[inline]
    pub const fn as_array<'a>(&'a self) -> &'a [f32] {
        unsafe {
            std::slice::from_raw_parts(self as *const Size as *const f32, 2)
        }
    }

    #[inline]
    pub const fn as_mut_array<'a>(&'a mut self) -> &'a mut [f32] {
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
}

impl From<(f32, f32)> for Size {
    fn from(value: (f32, f32)) -> Self {
        Self::from_tuple(value)
    }
}

impl From<[f32; 2]> for Size {
    fn from(value: [f32; 2]) -> Self {
        Self::from_array(value)
    }
}

impl From<f32> for Size {
    /// Converts the [f32] value into `Size::square(value)`.
    fn from(value: f32) -> Self {
        Self::square(value)
    }
}

impl std::ops::Add<Size> for Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Self::Output {
        self.add_dims(rhs.width, rhs.height)
    }
}

impl std::ops::Add<(f32, f32)> for Size {
    type Output = Size;
    fn add(self, rhs: (f32, f32)) -> Self::Output {
        self.add_dims(rhs.0, rhs.1)
    }
}

impl std::ops::Add<[f32; 2]> for Size {
    type Output = Size;
    fn add(self, rhs: [f32; 2]) -> Self::Output {
        self.add_dims(rhs[0], rhs[1])
    }
}

impl std::ops::Add<f32> for Size {
    type Output = Size;
    fn add(self, rhs: f32) -> Self::Output {
        self.add_dims(rhs, rhs)
    }
}

impl std::ops::Sub<Size> for Size {
    type Output = Size;
    fn sub(self, rhs: Size) -> Self::Output {
        self.sub_dims(rhs.width, rhs.height)
    }
}

impl std::ops::Sub<(f32, f32)> for Size {
    type Output = Size;
    fn sub(self, rhs: (f32, f32)) -> Self::Output {
        self.sub_dims(rhs.0, rhs.1)
    }
}

impl std::ops::Sub<[f32; 2]> for Size {
    type Output = Size;
    fn sub(self, rhs: [f32; 2]) -> Self::Output {
        self.sub_dims(rhs[0], rhs[1])
    }
}

impl std::ops::Sub<f32> for Size {
    type Output = Size;
    fn sub(self, rhs: f32) -> Self::Output {
        self.sub_dims(rhs, rhs)
    }
}

impl std::ops::Mul<Size> for Size {
    type Output = Size;
    fn mul(self, rhs: Size) -> Self::Output {
        self.mul_dims(rhs.width, rhs.height)
    }
}

impl std::ops::Mul<(f32, f32)> for Size {
    type Output = Size;
    fn mul(self, rhs: (f32, f32)) -> Self::Output {
        self.mul_dims(rhs.0, rhs.1)
    }
}

impl std::ops::Mul<[f32; 2]> for Size {
    type Output = Size;
    fn mul(self, rhs: [f32; 2]) -> Self::Output {
        self.mul_dims(rhs[0], rhs[1])
    }
}

impl std::ops::Mul<f32> for Size {
    type Output = Size;
    fn mul(self, rhs: f32) -> Self::Output {
        self.mul_dims(rhs, rhs)
    }
}

impl std::ops::Div<Size> for Size {
    type Output = Size;
    fn div(self, rhs: Size) -> Self::Output {
        self.div_dims(rhs.width, rhs.height)
    }
}

impl std::ops::Div<(f32, f32)> for Size {
    type Output = Size;
    fn div(self, rhs: (f32, f32)) -> Self::Output {
        self.div_dims(rhs.0, rhs.1)
    }
}

impl std::ops::Div<[f32; 2]> for Size {
    type Output = Size;
    fn div(self, rhs: [f32; 2]) -> Self::Output {
        self.div_dims(rhs[0], rhs[1])
    }
}

impl std::ops::Div<f32> for Size {
    type Output = Size;
    fn div(self, rhs: f32) -> Self::Output {
        self.div_dims(rhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn size_math_test() {
        let base = Size::new(10.0, 10.0);
        let mult = <Size as std::ops::Add<_>>::add(base, Size::new(1.0, 2.0));
        println!("{mult:?}");
    }
}
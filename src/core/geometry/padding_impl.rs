use super::Margin;
use super::Size;
use crate::core::math::lerp;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Padding {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Padding {
    pub const ZERO: Self = Padding::same(0.0);
    pub const NEG_ONE: Self = Padding::same(-1.0);
    pub const S1: Self = Padding::same(1.0);
    pub const S2: Self = Padding::same(2.0);
    pub const S3: Self = Padding::same(3.0);
    pub const S4: Self = Padding::same(4.0);
    pub const S5: Self = Padding::same(5.0);
    pub const S6: Self = Padding::same(6.0);
    pub const S8: Self = Padding::same(8.0);
    pub const S10: Self = Padding::same(10.0);
    pub const S15: Self = Padding::same(15.0);
    pub const S16: Self = Padding::same(16.0);
    pub const S18: Self = Padding::same(18.0);
    pub const S20: Self = Padding::same(20.0);
    pub const S22: Self = Padding::same(22.0);
    pub const S24: Self = Padding::same(24.0);
    pub const S25: Self = Padding::same(25.0);
    pub const S28: Self = Padding::same(28.0);
    pub const S32: Self = Padding::same(32.0);
    pub const S40: Self = Padding::same(40.0);
    pub const S50: Self = Padding::same(50.0);
    pub const S75: Self = Padding::same(75.0);
    pub const S100: Self = Padding::same(100.0);

    #[inline]
    #[must_use]
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left, top,
            right, bottom,
        }
    }

    #[inline]
    #[must_use]
    pub const fn same(all: f32) -> Self {
        Self {
            left: all,
            top: all,
            right: all,
            bottom: all,
        }
    }

    #[inline]
    #[must_use]
    pub const fn symmetric(x: f32, y: f32) -> Self {
        Self {
            left: x,
            top: y,
            right: x,
            bottom: y,
        }
    }

    #[inline]
    #[must_use]
    pub const fn x(self) -> f32 {
        self.left + self.right
    }

    #[inline]
    #[must_use]
    pub const fn y(self) -> f32 {
        self.top + self.bottom
    }

    #[inline]
    #[must_use]
    pub const fn add_padding(self, rhs: Padding) -> Self {
        Self {
            left: self.left + rhs.left,
            top: self.top + rhs.top,
            right: self.right + rhs.right,
            bottom: self.bottom + rhs.bottom,
        }
    }

    #[inline]
    #[must_use]
    pub const fn sub_padding(self, rhs: Padding) -> Self {
        Self {
            left: self.left - rhs.left,
            top: self.top - rhs.top,
            right: self.right - rhs.right,
            bottom: self.bottom - rhs.bottom,
        }
    }

    #[inline]
    #[must_use]
    pub const fn to_margin(self) -> Margin {
        unsafe {
            std::mem::transmute(self)
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_margin(margin: Margin) -> Self {
        unsafe {
            std::mem::transmute(margin)
        }
    }

    #[inline]
    #[must_use]
    pub const fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            left: lerp(self.left, other.left, t),
            top: lerp(self.top, other.top, t),
            right: lerp(self.right, other.right, t),
            bottom: lerp(self.bottom, other.bottom, t),
        }
    }

    #[inline]
    #[must_use]
    pub const fn clamped_lerp(self, other: Self, t: f32) -> Self {
        self.lerp(other, t.clamp(0.0, 1.0))
    }

    #[inline]
    #[must_use]
    pub const fn total_size(self) -> Size {
        Size::new(self.x(), self.y())
    }
}

impl From<Margin> for Padding {
    #[inline]
    fn from(value: Margin) -> Self {
        Self::from_margin(value)
    }
}

impl std::ops::Add<Padding> for Padding {
    type Output = Padding;

    #[inline]
    fn add(self, rhs: Padding) -> Self::Output {
        self.add_padding(rhs)
    }
}

impl std::ops::Sub<Padding> for Padding {
    type Output = Padding;

    #[inline]
    fn sub(self, rhs: Padding) -> Self::Output {
        self.sub_padding(rhs)
    }
}
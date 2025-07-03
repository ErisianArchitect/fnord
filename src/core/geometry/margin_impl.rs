use super::Padding;
use super::Size;
use crate::core::math::lerp;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Margin {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Margin {
    pub const ZERO: Self = Margin::same(0.0);
    pub const NEG_ONE: Self = Margin::same(-1.0);
    pub const S1: Self = Margin::same(1.0);
    pub const S2: Self = Margin::same(2.0);
    pub const S3: Self = Margin::same(3.0);
    pub const S4: Self = Margin::same(4.0);
    pub const S5: Self = Margin::same(5.0);
    pub const S6: Self = Margin::same(6.0);
    pub const S8: Self = Margin::same(8.0);
    pub const S10: Self = Margin::same(10.0);
    pub const S15: Self = Margin::same(15.0);
    pub const S16: Self = Margin::same(16.0);
    pub const S18: Self = Margin::same(18.0);
    pub const S20: Self = Margin::same(20.0);
    pub const S22: Self = Margin::same(22.0);
    pub const S24: Self = Margin::same(24.0);
    pub const S25: Self = Margin::same(25.0);
    pub const S28: Self = Margin::same(28.0);
    pub const S32: Self = Margin::same(32.0);
    pub const S40: Self = Margin::same(40.0);
    pub const S50: Self = Margin::same(50.0);
    pub const S75: Self = Margin::same(75.0);
    pub const S100: Self = Margin::same(100.0);

    #[inline]
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left, top,
            right, bottom,
        }
    }

    #[inline]
    pub const fn same(all: f32) -> Self {
        Self {
            left: all,
            top: all,
            right: all,
            bottom: all,
        }
    }

    #[inline]
    pub const fn symmetric(x: f32, y: f32) -> Self {
        Self {
            left: x,
            top: y,
            right: x,
            bottom: y,
        }
    }

    #[inline]
    pub const fn x(self) -> f32 {
        self.left + self.right
    }

    #[inline]
    pub const fn y(self) -> f32 {
        self.top + self.bottom
    }

    #[inline]
    pub const fn add_margin(self, rhs: Margin) -> Self {
        Self {
            left: self.left + rhs.left,
            top: self.top + rhs.top,
            right: self.right + rhs.right,
            bottom: self.bottom + rhs.bottom,
        }
    }

    #[inline]
    pub const fn sub_margin(self, rhs: Margin) -> Self {
        Self {
            left: self.left - rhs.left,
            top: self.top - rhs.top,
            right: self.right - rhs.right,
            bottom: self.bottom - rhs.bottom,
        }
    }

    #[inline]
    pub const fn to_padding(self) -> Padding {
        unsafe {
            std::mem::transmute(self)
        }
    }

    #[inline]
    pub const fn from_padding(padding: Padding) -> Self {
        unsafe {
            std::mem::transmute(padding)
        }
    }

    #[inline]
    pub const fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            left: lerp(self.left, other.left, t),
            top: lerp(self.top, other.top, t),
            right: lerp(self.right, other.right, t),
            bottom: lerp(self.bottom, other.bottom, t),
        }
    }

    #[inline]
    pub const fn clamped_lerp(self, other: Self, t: f32) -> Self {
        self.lerp(other, t.clamp(0.0, 1.0))
    }

    #[inline]
    pub const fn total_size(self) -> Size {
        Size::new(self.x(), self.y())
    }
}

impl From<Padding> for Margin {
    #[inline]
    fn from(value: Padding) -> Self {
        Self::from_padding(value)
    }
}

impl std::ops::Add<Margin> for Margin {
    type Output = Margin;

    #[inline]
    fn add(self, rhs: Margin) -> Self::Output {
        self.add_margin(rhs)
    }
}

impl std::ops::Sub<Margin> for Margin {
    type Output = Margin;

    #[inline]
    fn sub(self, rhs: Margin) -> Self::Output {
        self.sub_margin(rhs)
    }
}
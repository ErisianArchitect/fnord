use super::Margin;
use crate::core::math::lerp;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Padding {
    pub left: i8,
    pub top: i8,
    pub right: i8,
    pub bottom: i8,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Paddingf {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Padding {
    pub const ZERO: Self = Padding::same(0);
    pub const NEG_ONE: Self = Padding::same(-1);
    pub const S1: Self = Padding::same(1);
    pub const S2: Self = Padding::same(2);
    pub const S3: Self = Padding::same(3);
    pub const S4: Self = Padding::same(4);
    pub const S5: Self = Padding::same(5);
    pub const S6: Self = Padding::same(6);
    pub const S8: Self = Padding::same(8);
    pub const S10: Self = Padding::same(10);
    pub const S15: Self = Padding::same(15);
    pub const S16: Self = Padding::same(16);
    pub const S18: Self = Padding::same(18);
    pub const S20: Self = Padding::same(20);
    pub const S22: Self = Padding::same(22);
    pub const S24: Self = Padding::same(24);
    pub const S25: Self = Padding::same(25);
    pub const S28: Self = Padding::same(28);
    pub const S32: Self = Padding::same(32);
    pub const S40: Self = Padding::same(40);
    pub const S50: Self = Padding::same(50);
    pub const S75: Self = Padding::same(75);
    pub const S100: Self = Padding::same(100);
    pub const MIN: Self = Padding::same(i8::MIN);
    pub const MAX: Self = Padding::same(i8::MAX);

    #[inline]
    pub const fn new(left: i8, top: i8, right: i8, bottom: i8) -> Self {
        Self {
            left, top,
            right, bottom,
        }
    }

    #[inline]
    pub const fn same(all: i8) -> Self {
        Self {
            left: all,
            top: all,
            right: all,
            bottom: all,
        }
    }

    #[inline]
    pub const fn symmetric(x: i8, y: i8) -> Self {
        Self {
            left: x,
            top: y,
            right: x,
            bottom: y,
        }
    }

    #[inline]
    pub const fn x(self) -> i16 {
        self.left as i16 + self.right as i16
    }

    #[inline]
    pub const fn y(self) -> i16 {
        self.top as i16 + self.bottom as i16
    }

    #[inline]
    pub const fn to_paddingf(self) -> Paddingf {
        Paddingf {
            left: self.left as f32,
            top: self.top as f32,
            right: self.right as f32,
            bottom: self.bottom as f32,
        }
    }

    #[inline]
    pub const fn add_padding(self, rhs: Padding) -> Self {
        Self {
            left: self.left + rhs.left,
            top: self.top + rhs.top,
            right: self.right + rhs.right,
            bottom: self.bottom + rhs.bottom,
        }
    }

    #[inline]
    pub const fn sub_padding(self, rhs: Padding) -> Self {
        Self {
            left: self.left - rhs.left,
            top: self.top - rhs.top,
            right: self.right - rhs.right,
            bottom: self.bottom - rhs.bottom,
        }
    }

    #[inline]
    pub const fn to_margin(self) -> Margin {
        unsafe {
            std::mem::transmute(self)
        }
    }

    #[inline]
    pub const fn from_margin(margin: Margin) -> Self {
        unsafe {
            std::mem::transmute(margin)
        }
    }

    #[inline]
    pub const fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            left: lerp(self.left as f32, other.left as f32, t) as i8,
            top: lerp(self.top as f32, other.top as f32, t) as i8,
            right: lerp(self.right as f32, other.right as f32, t) as i8,
            bottom: lerp(self.bottom as f32, other.bottom as f32, t) as i8,
        }
    }

    #[inline]
    pub const fn clamped_lerp(self, other: Self, t: f32) -> Self {
        self.lerp(other, t.clamp(0.0, 1.0))
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
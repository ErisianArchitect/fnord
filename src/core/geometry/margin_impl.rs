use super::Padding;
use crate::core::math::lerp;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Margin {
    pub left: i8,
    pub top: i8,
    pub right: i8,
    pub bottom: i8,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Marginf {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Margin {
    pub const ZERO: Self = Margin::same(0);
    pub const NEG_ONE: Self = Margin::same(-1);
    pub const S1: Self = Margin::same(1);
    pub const S2: Self = Margin::same(2);
    pub const S3: Self = Margin::same(3);
    pub const S4: Self = Margin::same(4);
    pub const S5: Self = Margin::same(5);
    pub const S6: Self = Margin::same(6);
    pub const S8: Self = Margin::same(8);
    pub const S10: Self = Margin::same(10);
    pub const S15: Self = Margin::same(15);
    pub const S16: Self = Margin::same(16);
    pub const S18: Self = Margin::same(18);
    pub const S20: Self = Margin::same(20);
    pub const S22: Self = Margin::same(22);
    pub const S24: Self = Margin::same(24);
    pub const S25: Self = Margin::same(25);
    pub const S28: Self = Margin::same(28);
    pub const S32: Self = Margin::same(32);
    pub const S40: Self = Margin::same(40);
    pub const S50: Self = Margin::same(50);
    pub const S75: Self = Margin::same(75);
    pub const S100: Self = Margin::same(100);
    pub const MIN: Self = Margin::same(i8::MIN);
    pub const MAX: Self = Margin::same(i8::MAX);

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
    pub const fn to_marginf(self) -> Marginf {
        Marginf {
            left: self.left as f32,
            top: self.top as f32,
            right: self.right as f32,
            bottom: self.bottom as f32,
        }
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

    #[inline]
    pub const fn leftf(self) -> f32 {
        self.left as f32
    }

    #[inline]
    pub const fn topf(self) -> f32 {
        self.top as f32
    }

    #[inline]
    pub const fn rightf(self) -> f32 {
        self.right as f32
    }

    #[inline]
    pub const fn bottomf(self) -> f32 {
        self.bottom as f32
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
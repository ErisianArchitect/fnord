
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Align {
    #[default]
    Min = 0,
    Center = 1,
    Max = 2,
}

impl Align {
    pub const LEFT: Self = Self::Min;
    pub const RIGHT: Self = Self::Max;
    pub const TOP: Self = Self::Min;
    pub const BOTTOM: Self = Self::Max;

    #[inline]
    pub const fn from_i8(align: i8) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    #[inline]
    pub const fn from_i16(align: i16) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    #[inline]
    pub const fn from_i32(align: i32) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    #[inline]
    pub const fn from_i64(align: i64) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    #[inline]
    pub const fn from_i128(align: i128) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    #[inline]
    pub const fn from_f32(align: f32) -> Self {
        if align < 0.0 {
            Self::Min
        } else if align > 0.0 {
            Self::Max
        } else {
            Self::Center
        }
    }

    #[inline]
    pub const fn from_f64(align: f64) -> Self {
        if align < 0.0 {
            Self::Min
        } else if align > 0.0 {
            Self::Max
        } else {
            Self::Center
        }
    }

    #[inline]
    pub const fn align(self, min: f32, max: f32) -> f32 {
        match self {
            Align::Min => min,
            Align::Center => min + (max - min) * 0.5,
            Align::Max => max,
        }
    }
}
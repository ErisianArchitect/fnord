
/// Represents alignment along an axis within bounds.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Align {
    /// Alignment at the minimum bound (left/top).
    #[default]
    Min = 0,
    /// Alignment in the center.
    Center = 1,
    /// Alignment at the maximum bound (right/bottom).
    Max = 2,
}

impl Align {
    /// Alignment on the left.
    pub const LEFT: Self = Self::Min;
    /// Alignment on the right.
    pub const RIGHT: Self = Self::Max;
    /// Alignment at the top.
    pub const TOP: Self = Self::Min;
    /// Alignment at the bottom.
    pub const BOTTOM: Self = Self::Max;

    /// Converts an integer value into an [Align].
    /// `n < 0`  = [Align::Min]
    /// `n == 0` = [Align::Center]
    /// `n > 0`  = [Align::Max]
    #[inline]
    pub const fn from_i8(align: i8) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    /// Converts an integer value into an [Align].
    /// `n < 0`  = [Align::Min]
    /// `n == 0` = [Align::Center]
    /// `n > 0`  = [Align::Max]
    #[inline]
    pub const fn from_i16(align: i16) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    /// Converts an integer value into an [Align].
    /// `n < 0`  = [Align::Min]
    /// `n == 0` = [Align::Center]
    /// `n > 0`  = [Align::Max]
    #[inline]
    pub const fn from_i32(align: i32) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    /// Converts an integer value into an [Align].
    /// `n < 0`  = [Align::Min]
    /// `n == 0` = [Align::Center]
    /// `n > 0`  = [Align::Max]
    #[inline]
    pub const fn from_i64(align: i64) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    /// Converts an integer value into an [Align].
    /// `n < 0`  = [Align::Min]
    /// `n == 0` = [Align::Center]
    /// `n > 0`  = [Align::Max]
    #[inline]
    pub const fn from_i128(align: i128) -> Self {
        match align {
            ..0 => Self::Min,
            0 => Self::Center,
            1.. => Self::Max,
        }
    }

    /// Converts an integer value into an [Align].
    /// `n < 0`  = [Align::Min]
    /// `n == 0` = [Align::Center]
    /// `n > 0`  = [Align::Max]
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

    /// Converts an integer value into an [Align].
    /// `n < 0`  = [Align::Min]
    /// `n == 0` = [Align::Center]
    /// `n > 0`  = [Align::Max]
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

    /// Align within `min` and `max`.
    #[inline]
    pub const fn align(self, min: f32, max: f32) -> f32 {
        match self {
            Align::Min => min,
            Align::Center => min + (max - min) * 0.5,
            Align::Max => max,
        }
    }
}
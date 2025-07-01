use super::size::*;
use super::pos::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Grid {
    pub offset: Pos,
    pub cell_size: Size,
}

#[inline]
pub const fn grid(offset: Pos, cell_size: Size) -> Grid {
    Grid { offset, cell_size }
}

impl Grid {
    pub const UNIT: Self = Self::square(Pos::ZERO, 1.0);
    /// A unit grid (`width` and `height` are `1.0`) where the offset if half the size less than `(0.0, 0.0)`.
    pub const UNIT_CENTERED: Self = Self::centered_square(Pos::ZERO, 1.0);
    #[inline]
    pub const fn new(offset: Pos, cell_size: Size) -> Self {
        Self { offset, cell_size }
    }

    #[inline]
    pub const fn centered(center: Pos, cell_size: Size) -> Self {
        let half_size = cell_size.mul_dims(0.5, 0.5);
        Self::new(center.sub_dims(half_size.width, half_size.height), cell_size)
    }

    #[inline]
    pub const fn square(offset: Pos, size: f32) -> Self {
        Self::new(offset, Size::square(size))
    }

    #[inline]
    pub const fn centered_square(offset: Pos, size: f32) -> Self {
        let half = size * 0.5;
        Self::new(offset.sub_dims(half, half), Size::square(size))
    }
}
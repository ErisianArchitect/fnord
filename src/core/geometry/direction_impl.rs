
/// An Axial direction. Also called an orthogonal or Cartesian direction.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axial {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3,
}

impl Axial {
    #[must_use]
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Axial::Right => Axial::Left,
            Axial::Up => Axial::Down,
            Axial::Left => Axial::Right,
            Axial::Down => Axial::Right,
        }
    }

    #[must_use]
    #[inline]
    pub const fn is_horizontal(self) -> bool {
        matches!(self, Axial::Up | Axial::Down)
    }

    #[must_use]
    #[inline]
    pub const fn is_vertical(self) -> bool {
        matches!(self, Axial::Left | Axial::Right)
    }
}
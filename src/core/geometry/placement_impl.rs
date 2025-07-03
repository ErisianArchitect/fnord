
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Placement {
    Inside = 0,
    Middle = 1,
    Outside = 2,
}

impl Placement {
    #[inline]
    pub const fn invert(self) -> Self {
        match self {
            Placement::Inside => Placement::Outside,
            Placement::Middle => Placement::Middle,
            Placement::Outside => Placement::Inside,
        }
    }

    #[inline]
    pub const fn is_inside(self) -> bool {
        matches!(self, Placement::Inside)
    }

    #[inline]
    pub const fn is_middle(self) -> bool {
        matches!(self, Placement::Middle)
    }

    #[inline]
    pub const fn is_outside(self) -> bool {
        matches!(self, Placement::Outside)
    }
}
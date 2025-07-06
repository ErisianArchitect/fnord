
/// Represents the eight directions on a compass.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cardinal {
    /// East
    E = 0,
    /// North-East
    Ne = 1,
    /// North
    N = 2,
    /// North-West
    Nw = 3,
    /// West
    W = 4,
    /// South-West
    Sw = 5,
    /// South
    S = 6,
    /// South-East
    Se = 7,
}

impl Cardinal {
    /// Each [Cardinal] direction in clockwise order starting with East.
    pub const CW_FROM_NW: [Self; 8] = [
        Self::E,
        Self::Se,
        Self::S,
        Self::Sw,
        Self::W,
        Self::Nw,
        Self::N,
        Self::Ne,
    ];

    /// Each [Cardinal] direction in counter-clockwise order starting with East.
    pub const CCW_FROM_NW: [Self; 8] = [
        Self::E,
        Self::Ne,
        Self::N,
        Self::Nw,
        Self::W,
        Self::Sw,
        Self::S,
        Self::Se,
    ];

    /// The opposite direction.
    #[must_use]
    #[inline]
    pub const fn antipode(self) -> Self {
        match self {
            Cardinal::E => Cardinal::W,
            Cardinal::Ne => Cardinal::Sw,
            Cardinal::N => Cardinal::S,
            Cardinal::Nw => Cardinal::Se,
            Cardinal::W => Cardinal::E,
            Cardinal::Sw => Cardinal::Ne,
            Cardinal::S => Cardinal::N,
            Cardinal::Se => Cardinal::Nw,
        }
    }

    #[must_use]
    #[inline]
    pub const fn text(self) -> &'static str {
        match self {
            Cardinal::E => "East",
            Cardinal::Ne => "Northeast",
            Cardinal::N => "North",
            Cardinal::Nw => "Northwest",
            Cardinal::W => "West",
            Cardinal::Sw => "Southwest",
            Cardinal::S => "South",
            Cardinal::Se => "Southeast",
        }
    }

    #[must_use]
    #[inline]
    pub const fn is_northward(self) -> bool {
        matches!(self, Cardinal::Nw | Cardinal::N | Cardinal::Ne)
    }

    #[must_use]
    #[inline]
    pub const fn is_eastward(self) -> bool {
        matches!(self, Cardinal::Ne | Cardinal::E | Cardinal::Se)
    }

    #[must_use]
    #[inline]
    pub const fn is_southward(self) -> bool {
        matches!(self, Cardinal::Se | Cardinal::S | Cardinal::Sw)
    }

    #[must_use]
    #[inline]
    pub const fn is_westward(self) -> bool {
        matches!(self, Cardinal::Sw | Cardinal::W | Cardinal::Nw)
    }

    #[must_use]
    #[inline]
    pub const fn is_northwestward(self) -> bool {
        matches!(self, Cardinal::Nw | Cardinal::N | Cardinal::W)
    }

    #[must_use]
    #[inline]
    pub const fn is_northeastward(self) -> bool {
        matches!(self, Cardinal::N | Cardinal::Ne | Cardinal::E)
    }

    #[must_use]
    #[inline]
    pub const fn is_southeastward(self) -> bool {
        matches!(self, Cardinal::E | Cardinal::Se | Cardinal::S)
    }

    #[must_use]
    #[inline]
    pub const fn is_southwestward(self) -> bool {
        matches!(self, Cardinal::S | Cardinal::Sw | Cardinal::W)
    }

    #[must_use]
    #[inline]
    pub const fn is_primary(self) -> bool {
        matches!(self, Cardinal::N | Cardinal::E | Cardinal::S | Cardinal::W)
    }

    #[must_use]
    #[inline]
    pub const fn is_secondary(self) -> bool {
        matches!(self, Cardinal::Nw | Cardinal::Ne | Cardinal::Se | Cardinal::Sw)
    }
}

impl std::fmt::Display for Cardinal {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cardinal::E => write!(f, "East"),
            Cardinal::Ne => write!(f, "Northeast"),
            Cardinal::N => write!(f, "North"),
            Cardinal::Nw => write!(f, "Northwest"),
            Cardinal::W => write!(f, "West"),
            Cardinal::Sw => write!(f, "Southwest"),
            Cardinal::S => write!(f, "South"),
            Cardinal::Se => write!(f, "Southeast"),
        }
    }
}

impl std::fmt::Debug for Cardinal {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cardinal::E => write!(f, "Cardinal::E"),
            Cardinal::Ne => write!(f, "Cardinal::Ne"),
            Cardinal::N => write!(f, "Cardinal::N"),
            Cardinal::Nw => write!(f, "Cardinal::Nw"),
            Cardinal::W => write!(f, "Cardinal::W"),
            Cardinal::Sw => write!(f, "Cardinal::Sw"),
            Cardinal::S => write!(f, "Cardinal::S"),
            Cardinal::Se => write!(f, "Cardinal::Se"),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimaryCardinal {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intercardinal {
    Ne = 0,
    Nw = 1,
    Sw = 2,
    Se = 3,
}

impl PrimaryCardinal {
    pub const CW_FROM_EAST: [Self; 4] = [
        Self::East,
        Self::South,
        Self::West,
        Self::North,
    ];

    pub const CCW_FROM_EAST: [Self; 4] = [
        Self::East,
        Self::North,
        Self::West,
        Self::South,
    ];

    /// Returns the opposite direction.
    #[must_use]
    #[inline]
    pub const fn antipode(self) -> Self {
        match self {
            PrimaryCardinal::East => Self::West,
            PrimaryCardinal::North => Self::South,
            PrimaryCardinal::West => Self::East,
            PrimaryCardinal::South => Self::North,
        }
    }

    /// Determines if the direction is either North or South.
    #[must_use]
    #[inline]
    pub const fn is_longitudinal(self) -> bool {
        matches!(self, Self::North | Self::West)
    }

    /// Determines if the direction is either East or West.
    #[must_use]
    #[inline]
    pub const fn is_lateral(self) -> bool {
        matches!(self, Self::East | Self::West)
    }

    #[must_use]
    #[inline]
    pub const fn is_north_or_west(self) -> bool {
        matches!(self, Self::North | Self::West)
    }

    #[must_use]
    #[inline]
    pub const fn is_north_or_east(self) -> bool {
        matches!(self, Self::North | Self::East)
    }

    #[must_use]
    #[inline]
    pub const fn is_south_or_east(self) -> bool {
        matches!(self, Self::East | Self::South)
    }

    #[must_use]
    #[inline]
    pub const fn is_south_or_west(self) -> bool {
        matches!(self, Self::South | Self::West)
    }
}

impl Intercardinal {
    #[must_use]
    #[inline]
    pub const fn antipode(self) -> Self {
        match self {
            Intercardinal::Ne => Self::Sw,
            Intercardinal::Nw => Self::Se,
            Intercardinal::Sw => Self::Ne,
            Intercardinal::Se => Self::Nw,
        }
    }

    #[must_use]
    #[inline]
    pub const fn is_northward(self) -> bool {
        matches!(self, Self::Nw | Self::Ne)
    }

    #[must_use]
    #[inline]
    pub const fn is_eastward(self) -> bool {
        matches!(self, Self::Ne | Self::Se)
    }

    #[must_use]
    #[inline]
    pub const fn is_southward(self) -> bool {
        matches!(self, Self::Se | Self::Sw)
    }

    #[must_use]
    #[inline]
    pub const fn is_westward(self) -> bool {
        matches!(self, Self::Nw | Self::Sw)
    }
}
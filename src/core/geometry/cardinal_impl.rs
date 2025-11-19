use crate::core::geometry::Anchor;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Primary,
    Secondary,
}

impl Rank {
    #[inline(always)]
    #[must_use]
    pub const fn is_primary(self) -> bool {
        matches!(self, Self::Primary)
    }
    
    #[inline(always)]
    #[must_use]
    pub const fn is_secondary(self) -> bool {
        matches!(self, Self::Secondary)
    }
}

/// Represents the eight directions on a compass.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cardinal {
    // HEY! The order is important Please do not reorder.
    /// North-West
    Nw = 0,
    /// West
    W = 1,
    /// South-West
    Sw = 2,
    /// South
    S = 3,
    /// South-East
    Se = 4,
    /// East
    E = 5,
    /// North-East
    Ne = 6,
    /// North
    N = 7,
}

impl Cardinal {
    /// Each [Cardinal] direction in clockwise order starting with [Cardinal::Nw] (Norhtwest).
    pub const CW_FROM_NW: [Self; 8] = [
        Self::Nw,
        Self::N,
        Self::Ne,
        Self::E,
        Self::Se,
        Self::S,
        Self::Sw,
        Self::W,
    ];

    /// Each [Cardinal] direction in counter-clockwise order starting with [Cardinal::Nw] (Northwest).
    pub const CCW_FROM_NW: [Self; 8] = [
        Self::Nw,
        Self::W,
        Self::Sw,
        Self::S,
        Self::Se,
        Self::E,
        Self::Ne,
        Self::N,
    ];

    /// The opposite direction.
    #[inline]
    #[must_use]
    pub const fn antipode(self) -> Self {
        match self {
            Cardinal::Nw => Cardinal::Se,
            Cardinal::W => Cardinal::E,
            Cardinal::Sw => Cardinal::Ne,
            Cardinal::S => Cardinal::N,
            Cardinal::Se => Cardinal::Nw,
            Cardinal::E => Cardinal::W,
            Cardinal::Ne => Cardinal::Sw,
            Cardinal::N => Cardinal::S,
        }
    }

    #[inline]
    #[must_use]
    pub const fn anchor(self) -> Anchor {
        match self {
            Cardinal::Nw => Anchor::LeftTop,
            Cardinal::W => Anchor::LeftCenter,
            Cardinal::Sw => Anchor::LeftBottom,
            Cardinal::S => Anchor::BottomCenter,
            Cardinal::Se => Anchor::RightBottom,
            Cardinal::E => Anchor::RightCenter,
            Cardinal::Ne => Anchor::RightTop,
            Cardinal::N => Anchor::TopCenter,
        }
    }

    #[inline]
    #[must_use]
    pub const fn text(self) -> &'static str {
        match self {
            Cardinal::Nw => "Northwest",
            Cardinal::W => "West",
            Cardinal::Sw => "Southwest",
            Cardinal::S => "South",
            Cardinal::Se => "Southeast",
            Cardinal::E => "East",
            Cardinal::Ne => "Northeast",
            Cardinal::N => "North",
        }
    }
    
    #[inline]
    #[must_use]
    pub const fn rank(self) -> Rank {
        const RANKS: [Rank; 2] = [
            Rank::Secondary,
            Rank::Primary
        ];
        let bits = self as u8;
        // All of the primary directions have the 1 bit (2^0) set.
        // None of the secondary directions have the 1 bit (2^0) set.
        RANKS[(bits & 1) as usize]
    }

    #[inline]
    #[must_use]
    pub const fn is_primary(self) -> bool {
        self.rank().is_primary()
    }

    #[inline]
    #[must_use]
    pub const fn is_secondary(self) -> bool {
        self.rank().is_secondary()
    }

    #[inline]
    #[must_use]
    pub const fn is_northward(self) -> bool {
        matches!(self, Cardinal::Nw | Cardinal::N | Cardinal::Ne)
    }

    #[inline]
    #[must_use]
    pub const fn is_eastward(self) -> bool {
        matches!(self, Cardinal::Ne | Cardinal::E | Cardinal::Se)
    }

    #[inline]
    #[must_use]
    pub const fn is_southward(self) -> bool {
        matches!(self, Cardinal::Se | Cardinal::S | Cardinal::Sw)
    }

    #[inline]
    #[must_use]
    pub const fn is_westward(self) -> bool {
        matches!(self, Cardinal::Sw | Cardinal::W | Cardinal::Nw)
    }

    #[inline]
    #[must_use]
    pub const fn is_northwestward(self) -> bool {
        matches!(self, Cardinal::Nw | Cardinal::N | Cardinal::W)
    }

    #[inline]
    #[must_use]
    pub const fn is_northeastward(self) -> bool {
        matches!(self, Cardinal::N | Cardinal::Ne | Cardinal::E)
    }

    #[inline]
    #[must_use]
    pub const fn is_southeastward(self) -> bool {
        matches!(self, Cardinal::E | Cardinal::Se | Cardinal::S)
    }

    #[inline]
    #[must_use]
    pub const fn is_southwestward(self) -> bool {
        matches!(self, Cardinal::S | Cardinal::Sw | Cardinal::W)
    }
    
    #[inline]
    #[must_use]
    pub const fn is_northwest(self) -> bool {
        matches!(self, Cardinal::Nw)
    }
    
    #[inline]
    #[must_use]
    pub const fn is_west(self) -> bool {
        matches!(self, Cardinal::W)
    }
    
    #[inline]
    #[must_use]
    pub const fn is_southwest(self) -> bool {
        matches!(self, Cardinal::Sw)
    }
    
    #[inline]
    #[must_use]
    pub const fn is_south(self) -> bool {
        matches!(self, Cardinal::S)
    }
    
    #[inline]
    #[must_use]
    pub const fn is_southeast(self) -> bool {
        matches!(self, Cardinal::Se)
    }
    
    #[inline]
    #[must_use]
    pub const fn is_east(self) -> bool {
        matches!(self, Cardinal::E)
    }
    
    #[inline]
    #[must_use]
    pub const fn is_northeast(self) -> bool {
        matches!(self, Cardinal::Ne)
    }
    
    #[inline]
    #[must_use]
    pub const fn is_north(self) -> bool {
        matches!(self, Cardinal::N)
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
    #[inline]
    #[must_use]
    pub const fn antipode(self) -> Self {
        match self {
            PrimaryCardinal::East => Self::West,
            PrimaryCardinal::North => Self::South,
            PrimaryCardinal::West => Self::East,
            PrimaryCardinal::South => Self::North,
        }
    }

    /// Determines if the direction is either North or South.
    #[inline]
    #[must_use]
    pub const fn is_longitudinal(self) -> bool {
        matches!(self, Self::North | Self::South)
    }

    /// Determines if the direction is either East or West.
    #[inline]
    #[must_use]
    pub const fn is_lateral(self) -> bool {
        matches!(self, Self::East | Self::West)
    }

    #[inline]
    #[must_use]
    pub const fn is_north_or_west(self) -> bool {
        matches!(self, Self::North | Self::West)
    }

    #[inline]
    #[must_use]
    pub const fn is_north_or_east(self) -> bool {
        matches!(self, Self::North | Self::East)
    }

    #[inline]
    #[must_use]
    pub const fn is_south_or_east(self) -> bool {
        matches!(self, Self::East | Self::South)
    }

    #[inline]
    #[must_use]
    pub const fn is_south_or_west(self) -> bool {
        matches!(self, Self::South | Self::West)
    }
}

impl Intercardinal {
    #[inline]
    #[must_use]
    pub const fn antipode(self) -> Self {
        match self {
            Intercardinal::Ne => Self::Sw,
            Intercardinal::Nw => Self::Se,
            Intercardinal::Sw => Self::Ne,
            Intercardinal::Se => Self::Nw,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_northward(self) -> bool {
        matches!(self, Self::Nw | Self::Ne)
    }

    #[inline]
    #[must_use]
    pub const fn is_eastward(self) -> bool {
        matches!(self, Self::Ne | Self::Se)
    }

    #[inline]
    #[must_use]
    pub const fn is_southward(self) -> bool {
        matches!(self, Self::Se | Self::Sw)
    }

    #[inline]
    #[must_use]
    pub const fn is_westward(self) -> bool {
        matches!(self, Self::Nw | Self::Sw)
    }
}
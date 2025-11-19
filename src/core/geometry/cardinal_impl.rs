use crate::core::geometry::Anchor;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Primary,
    Secondary,
}

impl Rank {
    #[must_use]
    #[inline(always)]
    pub const fn is_primary(self) -> bool {
        matches!(self, Self::Primary)
    }
    
    #[must_use]
    #[inline(always)]
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
    
    #[must_use]
    #[inline(always)]
    pub const fn rank(self) -> Rank {
        const RANKS: [Rank; 2] = [
            Rank::Secondary,
            Rank::Primary
        ];
        let bits = self as usize;
        // All of the primary directions have the 1 bit (2^0) set.
        // None of the secondary directions have the 1 bit (2^0) set.
        RANKS[bits & 1]
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_primary(self) -> bool {
        self.rank().is_primary()
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_secondary(self) -> bool {
        self.rank().is_secondary()
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_northwestward(self) -> bool {
        matches!(self, Cardinal::Nw | Cardinal::N | Cardinal::W)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_westward(self) -> bool {
        matches!(self, Cardinal::Sw | Cardinal::W | Cardinal::Nw)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_southwestward(self) -> bool {
        matches!(self, Cardinal::S | Cardinal::Sw | Cardinal::W)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_southward(self) -> bool {
        matches!(self, Cardinal::Se | Cardinal::S | Cardinal::Sw)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_southeastward(self) -> bool {
        matches!(self, Cardinal::E | Cardinal::Se | Cardinal::S)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_eastward(self) -> bool {
        matches!(self, Cardinal::Ne | Cardinal::E | Cardinal::Se)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_northeastward(self) -> bool {
        matches!(self, Cardinal::N | Cardinal::Ne | Cardinal::E)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_northward(self) -> bool {
        matches!(self, Cardinal::Nw | Cardinal::N | Cardinal::Ne)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_northwest(self) -> bool {
        matches!(self, Cardinal::Nw)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_west(self) -> bool {
        matches!(self, Cardinal::W)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_southwest(self) -> bool {
        matches!(self, Cardinal::Sw)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_south(self) -> bool {
        matches!(self, Cardinal::S)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_southeast(self) -> bool {
        matches!(self, Cardinal::Se)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_east(self) -> bool {
        matches!(self, Cardinal::E)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_northeast(self) -> bool {
        matches!(self, Cardinal::Ne)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_north(self) -> bool {
        matches!(self, Cardinal::N)
    }
}

impl std::fmt::Display for Cardinal {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl std::fmt::Debug for Cardinal {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cardinal::Nw => write!(f, "Cardinal::Nw"),
            Cardinal::W => write!(f, "Cardinal::W"),
            Cardinal::Sw => write!(f, "Cardinal::Sw"),
            Cardinal::S => write!(f, "Cardinal::S"),
            Cardinal::Se => write!(f, "Cardinal::Se"),
            Cardinal::E => write!(f, "Cardinal::E"),
            Cardinal::Ne => write!(f, "Cardinal::Ne"),
            Cardinal::N => write!(f, "Cardinal::N"),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimaryCardinal {
    North = 0,
    West = 1,
    South = 2,
    East = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intercardinal {
    Nw = 0,
    Sw = 1,
    Se = 2,
    Ne = 3,
}

impl PrimaryCardinal {
    pub const CW_FROM_NORTH: [Self; 4] = [
        Self::North,
        Self::East,
        Self::South,
        Self::West,
    ];

    pub const CCW_FROM_NORTH: [Self; 4] = [
        Self::North,
        Self::West,
        Self::South,
        Self::East,
    ];

    /// Returns the opposite direction.
    #[must_use]
    #[inline(always)]
    pub const fn antipode(self) -> Self {
        match self {
            PrimaryCardinal::North => Self::South,
            PrimaryCardinal::West => Self::East,
            PrimaryCardinal::South => Self::North,
            PrimaryCardinal::East => Self::West,
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_north_or_west(self) -> bool {
        matches!(self, Self::North | Self::West)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_north_or_east(self) -> bool {
        matches!(self, Self::North | Self::East)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_south_or_east(self) -> bool {
        matches!(self, Self::East | Self::South)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_south_or_west(self) -> bool {
        matches!(self, Self::South | Self::West)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_north_or_south(self) -> bool {
        matches!(self, Self::North | Self::South)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_west_or_east(self) -> bool {
        matches!(self, Self::West | Self::East)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_north(self) -> bool {
        matches!(self, Self::North)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_west(self) -> bool {
        matches!(self, Self::West)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_south(self) -> bool {
        matches!(self, Self::South)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_east(self) -> bool {
        matches!(self, Self::East)
    }

    /// Determines if the direction is either North or South.
    #[must_use]
    #[inline(always)]
    pub const fn is_longitudinal(self) -> bool {
        self.is_north_or_south()
    }

    /// Determines if the direction is either West or East.
    #[must_use]
    #[inline(always)]
    pub const fn is_lateral(self) -> bool {
        self.is_west_or_east()
    }
}

impl Intercardinal {
    #[must_use]
    #[inline(always)]
    pub const fn antipode(self) -> Self {
        match self {
            Intercardinal::Nw => Self::Se,
            Intercardinal::Sw => Self::Ne,
            Intercardinal::Se => Self::Nw,
            Intercardinal::Ne => Self::Sw,
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_northward(self) -> bool {
        matches!(self, Self::Nw | Self::Ne)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_westward(self) -> bool {
        matches!(self, Self::Nw | Self::Sw)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_southward(self) -> bool {
        matches!(self, Self::Se | Self::Sw)
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_eastward(self) -> bool {
        matches!(self, Self::Ne | Self::Se)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn has_cardinality(self, cardinal: PrimaryCardinal) -> bool {
        match cardinal {
            PrimaryCardinal::North => self.is_northward(),
            PrimaryCardinal::West => self.is_westward(),
            PrimaryCardinal::South => self.is_southward(),
            PrimaryCardinal::East => self.is_eastward(),
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_nw(self) -> bool {
        matches!(self, Self::Nw)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_sw(self) -> bool {
        matches!(self, Self::Sw)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_se(self) -> bool {
        matches!(self, Self::Se)
    }
    
    #[must_use]
    #[inline(always)]
    pub const fn is_ne(self) -> bool {
        matches!(self, Self::Ne)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Anchor {
    LeftTop = 0,
    LeftCenter = 1,
    LeftBottom = 2,
    BottomCenter = 3,
    RightBottom = 4,
    RightCenter = 5,
    RightTop = 6,
    TopCenter = 7,
    Center = 8,
}

impl Anchor {
    pub const PERIMETER: [Anchor; 8] = [
        Anchor::LeftTop,
        Anchor::LeftCenter,
        Anchor::LeftBottom,
        Anchor::BottomCenter,
        Anchor::RightBottom,
        Anchor::RightCenter,
        Anchor::RightTop,
        Anchor::TopCenter,
    ];

    /// If the Anchor is not the Center, then this will rotate the anchor
    /// counter-clockwise by rotation.
    pub const fn rotate(self, rotation: i32) -> Self {
        let start_index = match self {
            Anchor::LeftTop => 0,
            Anchor::LeftCenter => 1,
            Anchor::LeftBottom => 2,
            Anchor::BottomCenter => 3,
            Anchor::RightBottom => 4,
            Anchor::RightCenter => 5,
            Anchor::RightTop => 6,
            Anchor::TopCenter => 7,
            Anchor::Center => return self,
        };
        let rot = (start_index as i64) + (rotation as i64);
        let selection = rot.rem_euclid(8) as usize;
        Self::PERIMETER[selection]
    }

    // left-top, left-center, left-bottom,
    // bottom-center, right-bottom, right-center
    // right-top, top-center
    #[inline]
    pub const fn invert(self) -> Self {
        match self {
            Anchor::LeftTop => Anchor::RightBottom,
            Anchor::LeftCenter => Anchor::RightCenter,
            Anchor::LeftBottom => Anchor::RightTop,
            Anchor::BottomCenter => Anchor::TopCenter,
            Anchor::RightBottom => Anchor::LeftTop,
            Anchor::RightCenter => Anchor::LeftCenter,
            Anchor::RightTop => Anchor::LeftBottom,
            Anchor::TopCenter => Anchor::BottomCenter,
            Anchor::Center => Anchor::Center,
        }
    }

    #[inline]
    pub const fn invert_horizontal(self) -> Self {
        match self {
            Anchor::LeftTop => Anchor::RightTop,
            Anchor::LeftCenter => Anchor::RightCenter,
            Anchor::LeftBottom => Anchor::RightBottom,
            Anchor::BottomCenter => Anchor::BottomCenter,
            Anchor::RightBottom => Anchor::LeftBottom,
            Anchor::RightCenter => Anchor::LeftCenter,
            Anchor::RightTop => Anchor::LeftTop,
            Anchor::TopCenter => Anchor::TopCenter,
            Anchor::Center => Anchor::Center,
        }
    }

    #[inline]
    pub const fn invert_vertical(self) -> Self {
        match self {
            Anchor::LeftTop => Anchor::LeftBottom,
            Anchor::LeftCenter => Anchor::LeftCenter,
            Anchor::LeftBottom => Anchor::LeftTop,
            Anchor::BottomCenter => Anchor::TopCenter,
            Anchor::RightBottom => Anchor::RightTop,
            Anchor::RightCenter => Anchor::RightCenter,
            Anchor::RightTop => Anchor::RightBottom,
            Anchor::TopCenter => Anchor::BottomCenter,
            Anchor::Center => Anchor::Center,
        }
    }
}
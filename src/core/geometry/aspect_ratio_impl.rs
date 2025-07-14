
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AspectRatio {
    pub ratio: f32
}

impl AspectRatio {
    #[must_use]
    #[inline]
    pub const fn new(ratio: f32) -> Self {
        Self { ratio }
    }

    #[must_use]
    #[inline]
    pub const fn from_dims(width: f32, height: f32) -> Self {
        Self::new(width / height)
    }

    // multiply height by aspect ratio to get width
    pub const fn width_from_height(self, height: f32) -> f32 {
        height * self.ratio
    }
    
    // divide by width by aspect ratio to get height
    pub const fn height_from_width(self, width: f32) -> f32 {
        width / self.ratio
    }
}
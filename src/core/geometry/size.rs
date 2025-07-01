
#[repr(C)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[inline]
pub const fn size(width: f32, height: f32) -> Size {
    Size { width, height }
}

impl Size {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE:  Self = Self::new(1.0, 1.0);
    pub const W:    Self = Self::new(1.0, 0.0);
    pub const H:    Self = Self::new(0.0, 1.0);
    
    #[inline]
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
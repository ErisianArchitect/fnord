
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Anchor {
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
    LeftCenter,
    TopCenter,
    RightCenter,
    BottomCenter,
    Center,
}
use super::pos::*;
use super::size::*;
use super::grid::*;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    min: Pos,
    max: Pos,
}

#[inline]
pub const fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect {
        min: Pos::new(x, y),
        max: Pos::new(x + width, y + height),
    }
}


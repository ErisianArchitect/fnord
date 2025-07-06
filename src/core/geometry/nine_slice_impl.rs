use super::Rect;
use super::Placement;
use super::Anchor;

#[derive(Debug, Clone, PartialEq)]
pub struct NineSlice {
    pub left_top: Rect,
    pub center_top: Rect,
    pub right_top: Rect,
    pub left_center: Rect,
    pub center: Rect,
    pub right_center: Rect,
    pub left_bottom: Rect,
    pub center_bottom: Rect,
    pub right_bottom: Rect,
}

impl NineSlice {
    pub const fn from_rect(rect: Rect, placement: Placement, size: f32) -> Self {
        Self {
            left_top: rect.handle_rect(Anchor::LeftTop, placement, size),
            center_top: rect.handle_rect(Anchor::TopCenter, placement, size),
            right_top: rect.handle_rect(Anchor::RightTop, placement, size),
            left_center: rect.handle_rect(Anchor::LeftCenter, placement, size),
            center: rect.handle_rect(Anchor::Center, placement, size),
            right_center: rect.handle_rect(Anchor::RightCenter, placement, size),
            left_bottom: rect.handle_rect(Anchor::LeftBottom, placement, size),
            center_bottom: rect.handle_rect(Anchor::BottomCenter, placement, size),
            right_bottom: rect.handle_rect(Anchor::RightBottom, placement, size),
        }
    }
}
use crate::core::geometry::util_impl::half;

use super::pos_impl::*;
use super::size_impl::*;
use super::margin_impl::*;
use super::anchor_impl::Anchor;
use super::placement_impl::Placement;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Pos,
    pub max: Pos,
}

#[inline]
pub const fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    debug_assert!(width >= 0.0 && height >= 0.0);
    Rect {
        min: Pos::new(x, y),
        max: Pos::new(x + width, y + height),
    }
}

impl Rect {
    #[inline]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        debug_assert!(width >= 0.0 && height >= 0.0);
        Self {
            min: Pos::new(x, y),
            max: Pos::new(x + width, y + height),
        }
    }

    #[inline]
    pub const fn from_min_max(min: Pos, max: Pos) -> Self {
        debug_assert!(min.x <= max.x && min.y <= max.y);
        Self {
            min,
            max,
        }
    }

    #[inline]
    pub const fn from_min_size(min: Pos, size: Size) -> Self {
        debug_assert!(size.is_positive());
        Self {
            min,
            max: min.add_dims(size.width, size.height),
        }
    }

    #[inline]
    pub const fn square_from_min_size(min: Pos, size: f32) -> Self {
        Self {
            min,
            max: Pos::new(min.x + size, min.y + size),
        }
    }

    #[inline]
    pub const fn centered(center: Pos, size: Size) -> Self {
        let half_size = size.mul_dims(0.5, 0.5);
        Self {
            min: center.sub_dims(half_size.width, half_size.height),
            max: center.add_dims(half_size.width, half_size.height),
        }
    }

    #[inline]
    pub const fn centered_square(center: Pos, size: f32) -> Self {
        let half_size = size * 0.5;
        Self {
            min: center.sub_dims(half_size, half_size),
            max: center.add_dims(half_size, half_size),
        }
    }

    pub const fn from_anchored_pivot(anchor: Anchor, pivot: Pos, size: Size) -> Rect {
        match anchor {
            Anchor::LeftTop => Self::from_min_size(pivot, size),
            Anchor::RightTop => Self::from_min_size(
                Pos::new(pivot.x - size.width, pivot.y),
                size,
            ),
            Anchor::LeftBottom => Self::from_min_size(
                Pos::new(pivot.x, pivot.y - size.height),
                size,
            ),
            Anchor::RightBottom => Self::from_min_size(
                Pos::new(pivot.x - size.width, pivot.y - size.height),
                size,
            ),
            Anchor::LeftCenter => {
                let half_height = size.half_height();
                Self::from_min_size(
                    Pos::new(pivot.x, pivot.y - half_height),
                    size,
                )
            },
            Anchor::TopCenter => {
                let half_width = size.half_width();
                Self::from_min_size(
                    Pos::new(pivot.x - half_width, pivot.y),
                    size,
                )
            },
            Anchor::RightCenter => {
                let half_height = size.half_height();
                Self::from_min_size(
                    Pos::new(pivot.x - size.width, pivot.y - half_height),
                    size,
                )
            },
            Anchor::BottomCenter => {
                let half_width = size.half_width();
                Self::from_min_size(
                    Pos::new(pivot.x - half_width, pivot.y - size.height),
                    size,
                )
            },
            Anchor::Center => {
                let half_size = size.half();
                Self::from_min_size(
                    Pos::new(pivot.x - half_size.width, pivot.y - half_size.height),
                    size,
                )
            },
        }
    }

    #[inline]
    pub const fn from_points(points: [Pos; 2]) -> Self {
        Self {
            min: Pos::new(
                points[0].x.min(points[1].x),
                points[0].y.min(points[1].y)
            ),
            max: Pos::new(
                points[0].x.max(points[1].x),
                points[0].y.max(points[1].y)
            ),
        }
    }

    #[inline]
    pub const fn size(self) -> Size {
        Size::new(
            self.width(),
            self.height(),
        )
    }

    // Dimensions
    #[inline]
    pub const fn width(self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub const fn set_width(&mut self, width: f32) {
        self.max.x = self.min.x + width;
    }

    #[inline]
    pub const fn height(self) -> f32 {
        self.max.y - self.min.y
    }

    #[inline]
    pub const fn set_height(&mut self, height: f32) {
        self.max.y = self.min.y + height;
    }

    #[inline]
    pub const fn left(self) -> f32 {
        self.min.x
    }

    #[inline]
    pub const fn set_left(&mut self, left: f32) {
        let width = self.width();
        self.min.x = left;
        self.max.x = left + width;
    }

    #[inline]
    pub const fn right(self) -> f32 {
        self.max.x
    }

    #[inline]
    pub const fn set_right(&mut self, right: f32) {
        let width = self.width();
        self.min.x = right - width;
        self.max.x = right;
    }

    #[inline]
    pub const fn top(self) -> f32 {
        self.min.y
    }

    #[inline]
    pub const fn set_top(&mut self, top: f32) {
        let height = self.height();
        self.min.y = top;
        self.max.y = top + height; 
    }

    #[inline]
    pub const fn bottom(self) -> f32 {
        self.max.y
    }

    #[inline]
    pub const fn set_bottom(&mut self, bottom: f32) {
        let height = self.height();
        self.min.y = bottom - height;
        self.max.y = bottom;
    }

    #[inline]
    pub const fn left_top(self) -> Pos {
        self.min
    }

    #[inline]
    pub const fn set_left_top(&mut self, left_top: Pos) {
        let size = self.size();
        self.min = left_top;
        self.max = left_top.add_dims(size.width, size.height);
    }

    #[inline]
    pub const fn right_top(self) -> Pos {
        Pos::new(self.max.x, self.min.y)
    }

    #[inline]
    pub const fn set_right_top(&mut self, right_top: Pos) {
        let size = self.size();
        self.min = pos(right_top.x - size.width, right_top.y);
        self.max = pos(right_top.x, right_top.y + size.height);
    }

    #[inline]
    pub const fn left_bottom(self) -> Pos {
        Pos::new(self.min.x, self.max.y)
    }

    #[inline]
    pub const fn set_left_bottom(&mut self, left_bottom: Pos) {
        let size = self.size();
        self.min = pos(left_bottom.x, left_bottom.y - size.height);
        self.max = pos(left_bottom.x + size.width, left_bottom.y);
    }

    #[inline]
    pub const fn right_bottom(self) -> Pos {
        self.max
    }

    #[inline]
    pub const fn set_right_bottom(&mut self, right_bottom: Pos) {
        let size = self.size();
        self.min = pos(right_bottom.x - size.width, right_bottom.y - size.height);
        self.max = right_bottom;
    }

    #[inline]
    pub const fn left_center(self) -> Pos {
        // We assume that self.max.y >= self.min.y
        let center_y = self.min.y + (self.max.y - self.min.y) * 0.5;
        Pos::new(self.min.x, center_y)
    }

    #[inline]
    pub const fn set_left_center(&mut self, left_center: Pos) {
        let size = self.size();
        let half_height = size.half_height();
        self.min = pos(left_center.x, left_center.y - half_height);
        self.max = pos(left_center.x + size.width, left_center.y + half_height);
    }

    #[inline]
    pub const fn top_center(self) -> Pos {
        let center_x = self.min.x + (self.max.x - self.max.y) * 0.5;
        Pos::new(center_x, self.min.y)
    }

    #[inline]
    pub const fn set_top_center(&mut self, top_center: Pos) {
        let size = self.size();
        let half_width = size.half_width();
        self.min = pos(top_center.x - half_width, top_center.y);
        self.max = pos(top_center.x + half_width, top_center.y + size.height);
    }

    #[inline]
    pub const fn right_center(self) -> Pos {
        let center_y = self.min.y + (self.max.y - self.min.y) * 0.5;
        Pos::new(self.max.x, center_y)
    }

    #[inline]
    pub const fn set_right_center(&mut self, right_center: Pos) {
        let size = self.size();
        let half_height = size.half_height();
        self.min = pos(right_center.x - size.width, right_center.y - half_height);
        self.max = pos(right_center.x, right_center.y + half_height);
    }

    #[inline]
    pub const fn bottom_center(self) -> Pos {
        let center_x = self.min.x + (self.max.x - self.min.x) * 0.5;
        Pos::new(center_x, self.max.y)
    }

    #[inline]
    pub const fn set_bottom_center(&mut self, bottom_center: Pos) {
        let size = self.size();
        let half_width = size.half_width();
        self.min = pos(bottom_center.x - half_width, bottom_center.y - size.height);
        self.max = pos(bottom_center.x + half_width, bottom_center.y);
    }

    #[inline]
    pub const fn center(self) -> Pos {
        let center_x = self.min.x + (self.max.x - self.min.x) * 0.5;
        let center_y = self.min.y + (self.max.y - self.min.y) * 0.5;
        Pos::new(center_x, center_y)
    }

    #[inline]
    pub const fn set_center(&mut self, center: Pos) {
        let half_size = self.size().half();
        self.min = pos(center.x - half_size.width, center.y - half_size.height);
        self.max = pos(center.x + half_size.width, center.y + half_size.height);
    }

    /// Takes a uv coordinate (0.0 to 1.0 for x and y) and returns the position at that UV coordinate on the [Rect].
    #[inline]
    pub const fn uv_pos(self, uv: Pos) -> Pos {
        let x = self.min.x + (self.max.x - self.min.x) * uv.x;
        let y = self.min.y + (self.max.y - self.min.y) * uv.y;
        Pos::new(x, y)
    }

    #[inline]
    pub const fn contains(self, pos: Pos) -> bool {
        self.min.x <= pos.x && self.min.y <= pos.y
        && self.max.x > pos.x && self.max.y > pos.y
    }

    #[inline]
    pub const fn intersects(self, rect: &Rect) -> bool {
        rect.min.x < self.max.x && rect.min.y < self.max.y
        && rect.max.x > self.min.x && rect.max.y > self.min.y
    }

    #[inline]
    pub const fn translate(self, offset: Pos) -> Self {
        Self {
            min: Pos::new(self.min.x + offset.x, self.min.y + offset.y),
            max: Pos::new(self.max.x + offset.x, self.max.y + offset.y),
        }
    }

    #[inline]
    pub const fn anchor_pos(&self, anchor: Anchor) -> Pos {
        match anchor {
            Anchor::LeftTop => self.left_top(),
            Anchor::RightTop => self.right_top(),
            Anchor::LeftBottom => self.left_bottom(),
            Anchor::RightBottom => self.right_bottom(),
            Anchor::LeftCenter => self.left_center(),
            Anchor::TopCenter => self.top_center(),
            Anchor::RightCenter => self.right_center(),
            Anchor::BottomCenter => self.bottom_center(),
            Anchor::Center => self.center(),
        }
    }

    #[inline]
    pub const fn shrink(self, shrink: f32) -> Self {
        Self {
            min: Pos::new(self.min.x + shrink, self.min.y + shrink),
            max: Pos::new(self.max.x - shrink, self.max.y - shrink),
        }
    }

    #[inline]
    pub const fn shrink2(self, x: f32, y: f32) -> Self {
        Self {
            min: Pos::new(self.min.x + x, self.min.y + y),
            max: Pos::new(self.max.x - x, self.max.y - y),
        }
    }

    #[inline]
    pub const fn expand(self, expand: f32) -> Self {
        Self {
            min: Pos::new(self.min.x - expand, self.min.y - expand),
            max: Pos::new(self.max.x + expand, self.max.y + expand),
        }
    }

    #[inline]
    pub const fn expand2(self, x: f32, y: f32) -> Self {
        Self {
            min: Pos::new(self.min.x - x, self.min.y - y),
            max: Pos::new(self.max.x + x, self.max.y + y),
        }
    }

    /// Add a [Margin] to a [Rect].
    #[inline]
    pub const fn add_margin(self, margin: Margin) -> Self {
        Self {
            min: Pos::new(self.min.x + margin.left as f32, self.min.y + margin.top as f32),
            max: Pos::new(self.max.x - margin.right as f32, self.max.y - margin.bottom as f32),
        }
    }

    /// Remove a [Margin] from a [Rect].
    /// This is the inverse of `add_margin`.
    #[inline]
    pub const fn sub_margin(self, margin: Margin) -> Self {
        Self {
            min: Pos::new(self.min.x - margin.left as f32, self.min.y - margin.top as f32),
            max: Pos::new(self.max.x + margin.right as f32, self.max.y + margin.bottom as f32),
        }
    }

    /// Applies a [Margin] in-place, mutating the [Rect].
    #[inline]
    pub const fn apply_margin(&mut self, margin: Margin) {
        self.min.x += margin.left as f32;
        self.min.y += margin.top as f32;
        self.max.x -= margin.right as f32;
        self.max.y -= margin.bottom as f32;
    }

    /// Removes a [Margin] in-place, mutating the [Rect].
    #[inline]
    pub const fn remove_margin(&mut self, margin: Margin) {
        self.min.x -= margin.left as f32;
        self.min.y -= margin.top as f32;
        self.max.x += margin.right as f32;
        self.max.y += margin.bottom as f32;
    }

    /// This will return (`left`, `right`).
    #[inline]
    pub const fn split_from_left(self, split: f32) -> (Self, Self) {
        // this will take a rect and split it where the lhs
        // is the left side of the rect with the right at the split.
        // to get the split position, it would be left + split
        let lhs_max = pos(self.min.x + split, self.max.y);
        let rhs_min = pos(lhs_max.x, self.min.y);
        (
            Rect {
                min: self.min,
                max: lhs_max,
            },
            Rect {
                min: rhs_min,
                max: self.max,
            }
        )
    }

    /// This will return (`top`, `bottom`).
    #[inline]
    pub const fn split_from_top(self, split: f32) -> (Self, Self) {
        let lhs_max = pos(self.max.x, self.min.y + split);
        let rhs_min = pos(self.min.x, lhs_max.y);
        (
            Rect {
                min: self.min,
                max: lhs_max,
            },
            Rect {
                min: rhs_min,
                max: self.max,
            }
        )
    }

    /// This will return (`self.right`, `self.left`).
    #[inline]
    pub const fn split_from_right(self, split: f32) -> (Self, Self) {
        let lhs_min = pos(self.max.x - split, self.min.y);
        let rhs_max = pos(lhs_min.x, self.max.y);
        (
            Rect {
                min: lhs_min,
                max: self.max,
            },
            Rect {
                min: self.min,
                max: rhs_max,
            }
        )
    }

    /// This will return (`self.bottom`, `self.top`).
    #[inline]
    pub const fn split_from_bottom(self, split: f32) -> (Self, Self) {
        let lhs_min = pos(self.min.x, self.max.y - split);
        let rhs_max = pos(self.max.x, lhs_min.y);
        (
            Rect {
                min: lhs_min,
                max: self.max,
            },
            Rect {
                min: self.min,
                max: rhs_max,
            }
        )
    }

    #[inline]
    pub const fn left_adjacent(self, length: f32) -> Self {
        Self {
            min: Pos::new(self.min.x - length, self.min.y),
            max: self.left_bottom(),
        }
    }

    #[inline]
    pub const fn top_adjacent(self, length: f32) -> Self {
        Self {
            min: Pos::new(self.min.x, self.min.y - length),
            max: self.right_top(),
        }
    }

    #[inline]
    pub const fn right_adjacent(self, length: f32) -> Self {
        Self {
            min: self.right_top(),
            max: Pos::new(self.max.x + length, self.max.y),
        }
    }

    #[inline]
    pub const fn bottom_adjacent(self, length: f32) -> Self {
        Self {
            min: self.left_bottom(),
            max: Pos::new(self.max.x, self.max.y + length),
        }
    }

    pub const fn anchor_rect(self, anchor: Anchor, placement: Placement, size: f32) -> Self {
        match (anchor, placement) {
            (Anchor::LeftTop, Placement::Inside) => Rect {
                min: self.min,
                max: pos(self.min.x + size, self.min.y + size),
            },
            (Anchor::LeftTop, Placement::Middle) => {
                let half_size = size * 0.5;
                Rect {
                    min: pos(self.min.x - half_size, self.min.y - half_size),
                    max: pos(self.min.x + half_size, self.min.y + half_size),
                }
            },
            (Anchor::LeftTop, Placement::Outside) => Rect {
                min: pos(self.min.x - size, self.min.y - size),
                max: self.min,
            },
            (Anchor::RightTop, Placement::Inside) => Rect {
                min: pos(self.max.x - size, self.min.y),
                max: pos(self.max.x, self.min.y + size),
            },
            (Anchor::RightTop, Placement::Middle) => {
                let half_size = size * 0.5;
                Rect {
                    min: pos(self.max.x - half_size, self.min.y - half_size),
                    max: pos(self.max.x + half_size, self.min.y + half_size),
                }
            },
            (Anchor::RightTop, Placement::Outside) => Rect {
                min: pos(self.max.x, self.min.y - size),
                max: pos(self.max.x + size, self.min.y),
            },
            (Anchor::LeftBottom, Placement::Inside) => Rect {
                min: pos(self.min.x, self.max.y - size),
                max: pos(self.min.x + size, self.max.y),
            },
            (Anchor::LeftBottom, Placement::Middle) => {
                let half_size = size * 0.5;
                Rect {
                    min: pos(self.min.x - half_size, self.max.y - half_size),
                    max: pos(self.min.x + half_size, self.max.y + half_size),
                }
            },
            (Anchor::LeftBottom, Placement::Outside) => Rect {
                min: pos(self.min.x - size, self.max.y),
                max: pos(self.min.x, self.max.y + size),
            },
            (Anchor::RightBottom, Placement::Inside) => Rect {
                min: pos(self.max.x - size, self.max.y - size),
                max: self.max,
            },
            (Anchor::RightBottom, Placement::Middle) => {
                let half_size = size * 0.5;
                Rect {
                    min: pos(self.max.x - half_size, self.max.y - half_size),
                    max: pos(self.max.x + half_size, self.max.y + half_size),
                }
            },
            (Anchor::RightBottom, Placement::Outside) => Rect {
                min: self.max,
                max: pos(self.max.x + size, self.max.y + size),
            },
            (Anchor::LeftCenter, Placement::Inside) => Rect {
                min: pos(self.min.x, self.min.y + size),
                max: pos(self.min.x + size, self.max.y - size),
            },
            (Anchor::LeftCenter, Placement::Middle) => {
                let half_size = size * 0.5;
                Rect {
                    min: pos(self.min.x - half_size, self.min.y + half_size),
                    max: pos(self.min.x + half_size, self.max.y - half_size),
                }
            },
            (Anchor::LeftCenter, Placement::Outside) => Rect {
                min: pos(self.min.x - size, self.min.y),
                max: self.left_bottom(),
            },
            (Anchor::TopCenter, Placement::Inside) => Rect {
                min: pos(self.min.x + size, self.min.y),
                max: pos(self.max.x - size, self.min.y + size),
            },
            (Anchor::TopCenter, Placement::Middle) => {
                let half_size = size * 0.5;
                Rect {
                    min: pos(self.min.x + half_size, self.min.y - half_size),
                    max: pos(self.max.x - half_size, self.min.y + half_size),
                }
            },
            (Anchor::TopCenter, Placement::Outside) => Rect {
                min: pos(self.min.x, self.min.y - size),
                max: self.right_top(),
            },
            (Anchor::RightCenter, Placement::Inside) => Rect {
                min: pos(self.max.x - size, self.min.y + size),
                max: pos(self.max.x, self.max.y - size),
            },
            (Anchor::RightCenter, Placement::Middle) => {
                let half_size = size * 0.5;
                Rect {
                    min: pos(self.max.x - half_size, self.min.y + half_size),
                    max: pos(self.max.x + half_size, self.max.y - half_size),
                }
            },
            (Anchor::RightCenter, Placement::Outside) => Rect {
                min: self.right_top(),
                max: pos(self.max.x + size, self.max.y),
            },
            (Anchor::BottomCenter, Placement::Inside) => Rect {
                min: pos(self.min.x + size, self.max.y - size),
                max: pos(self.max.x - size, self.max.y),
            },
            (Anchor::BottomCenter, Placement::Middle) => {
                let half_size = size * 0.5;
                Rect {
                    min: pos(self.min.x + half_size, self.max.y - half_size),
                    max: pos(self.max.x - half_size, self.max.y + half_size),
                }
            },
            (Anchor::BottomCenter, Placement::Outside) => Rect {
                min: self.left_bottom(),
                max: pos(self.max.x, self.max.y + size),
            },
            (Anchor::Center, Placement::Inside) => Rect {
                min: pos(self.min.x + size, self.min.y + size),
                max: pos(self.max.x - size, self.max.y - size),
            },
            (Anchor::Center, Placement::Middle) => {
                let half_size = half(size);
                Rect {
                    min: pos(self.min.x + half_size, self.min.y + half_size),
                    max: pos(self.max.x - half_size, self.max.y - half_size),
                }
            },
            (Anchor::Center, Placement::Outside) => self,
        }
    }

    /// Recturns the smallest [Rect] that contains both [Rect]s.
    pub const fn combine(self, other: Rect) -> Self {
        let min_x = self.min.x.min(other.min.x);
        let min_y = self.min.y.min(other.min.y);
        let max_x = self.max.x.max(other.max.x);
        let max_y = self.max.y.max(other.max.y);
        Self {
            min: Pos::new(min_x, min_y),
            max: Pos::new(max_x, max_y),
        }
    }
}
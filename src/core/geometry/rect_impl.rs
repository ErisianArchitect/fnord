use crate::core::geometry::util_impl::half;
use crate::core::math::lerp;

use super::pos_impl::*;
use super::size_impl::*;
use super::margin_impl::*;
use super::padding_impl::*;
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
    pub const ZERO: Self = Self::from_min_max(Pos::ZERO, Pos::ZERO);
    pub const ONE: Self = Self::from_min_max(Pos::ZERO, Pos::ONE);

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
            min: points[0].min(points[1]),
            max: points[1].max(points[0]),
        }
    }

    #[inline]
    pub const fn from_points_slice(slice: &[Pos]) -> Self {
        debug_assert!(slice.len() >= 2);
        Self {
            min: slice[0].min(slice[1]),
            max: slice[1].max(slice[0]),
        }
    }

    /// Ensures that the min is the min and the max is the max.
    #[inline]
    pub const fn fix(&mut self) {
        let min = self.min.min(self.max);
        let max = self.max.max(self.min);
        self.min = min;
        self.max = max;
    }

    /// Returns a [Rect] where the `min` and `max` are "fixed".
    /// That is, the `min` is the real minimum bound and the max
    /// is the real maximum bound.
    #[inline]
    pub const fn fixed(self) -> Self {
        let min = self.min.min(self.max);
        let max = self.max.max(self.min);
        Self::from_min_max(min, max)
    }

    #[inline]
    pub const fn size(self) -> Size {
        Size::new(
            self.width(),
            self.height(),
        )
    }

    #[inline]
    pub const fn set_size(&mut self, size: Size) {
        self.max = self.min.add_dims(size.width, size.height);
    }

    #[inline]
    pub const fn set_size_centered(&mut self, size: Size) {
        let mid_point = self.center();
        let half_size = size.half();
        self.min.x = mid_point.x - half_size.width;
        self.min.y = mid_point.y - half_size.height;
        self.max.x = mid_point.x + half_size.width;
        self.max.y = mid_point.y + half_size.height;
    }

    #[inline]
    pub const fn resize(self, size: Size) -> Self {
        Self {
            min: self.min,
            max: self.max.add_dims(size.width, size.height),
        }
    }

    #[inline]
    pub const fn resize_centered(self, size: Size) -> Self {
        let mid_point = self.center();
        let half_size = size.half();
        Self::from_min_max(
            Pos::new(
                mid_point.x - half_size.width,
                mid_point.y - half_size.height
            ),
            Pos::new(
                mid_point.x + half_size.width,
                mid_point.y + half_size.height,
            ),
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
    pub const fn set_width_centered(&mut self, width: f32) {
        let half_width = half(width);
        let cur_half_width = half(self.width());
        let mid_x = self.min.x + cur_half_width;
        self.min.x = mid_x - half_width;
        self.max.x = mid_x + half_width;
    }

    #[inline]
    pub const fn set_width_right(&mut self, width: f32) {
        self.min.x = self.max.x - width;
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
    pub const fn set_height_centered(&mut self, height: f32) {
        let half_height = half(height);
        let cur_half_height = half(self.height());
        let mid_y = self.min.y + cur_half_height;
        self.min.y = mid_y - half_height;
        self.max.y = mid_y + half_height;
    }

    #[inline]
    pub const fn set_height_bottom(&mut self, height: f32) {
        self.min.y = self.max.y - height;
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
    pub const fn set_left_bound(&mut self, left: f32) {
        self.min.x = left;
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
    pub const fn set_right_bound(&mut self, right: f32) {
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
        let center_x = self.min.x + (self.max.x - self.max.x) * 0.5;
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
        let center_x = lerp(self.min.x, self.max.x, 0.5);
        let center_y = lerp(self.min.y, self.max.y, 0.5);
        Pos::new(center_x, center_y)
    }

    #[inline]
    pub const fn set_center(&mut self, center: Pos) {
        let half_size = self.size().half();
        self.min = pos(center.x - half_size.width, center.y - half_size.height);
        self.max = pos(center.x + half_size.width, center.y + half_size.height);
    }

    #[inline]
    pub const fn place_anchor(&mut self, anchor: Anchor, pos: Pos) {
        match anchor {
            Anchor::LeftTop => self.set_left_top(pos),
            Anchor::RightTop => self.set_right_top(pos),
            Anchor::LeftBottom => self.set_left_bottom(pos),
            Anchor::RightBottom => self.set_right_bottom(pos),
            Anchor::LeftCenter => self.set_left_center(pos),
            Anchor::TopCenter => self.set_top_center(pos),
            Anchor::RightCenter => self.set_right_center(pos),
            Anchor::BottomCenter => self.set_bottom_center(pos),
            Anchor::Center => self.set_center(pos),
        }
    }

    /// Takes a uv coordinate (0.0 to 1.0 for x and y) and returns the position at that UV coordinate on the [Rect].
    #[inline]
    pub const fn uv_pos(self, uv: Pos) -> Pos {
        Pos::new(
            lerp(self.min.x, self.max.x, uv.x),
            lerp(self.min.y, self.max.y, uv.y),
        )
    }

    #[inline]
    pub const fn set_uv_pos(&mut self, uv: Pos, pos: Pos) {
        let uv_pos = self.uv_pos(uv);
        let diff = Pos::new(
            pos.x - uv_pos.x,
            pos.y - uv_pos.y,
        );
        self.min.x += diff.x;
        self.min.y += diff.y;
        self.max.x += diff.x;
        self.max.y += diff.y;
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
    pub const fn intersect(self, other: Rect) -> Option<Rect> {
        if !self.intersects(&other) {
            return None;
        }
        let left = self.min.x.max(other.min.x);
        let top = self.min.y.max(other.min.y);
        let right = self.max.x.min(other.max.x);
        let bottom = self.max.y.min(other.max.y);
        Some(Rect {
            min: pos(left, top),
            max: pos(right, bottom),
        })
    }

    #[inline]
    pub const fn translate(self, offset: Pos) -> Self {
        Self {
            min: Pos::new(self.min.x + offset.x, self.min.y + offset.y),
            max: Pos::new(self.max.x + offset.x, self.max.y + offset.y),
        }
    }

    #[inline]
    pub const fn add_offset(self, offset: Pos) -> Self {
        Self {
            min: Pos::new(self.min.x + offset.x, self.min.y + offset.y),
            max: Pos::new(self.max.x + offset.x, self.max.y + offset.y),
        }
    }

    #[inline]
    pub const fn sub_offset(self, offset: Pos) -> Self {
        Self {
            min: Pos::new(self.min.x - offset.x, self.min.y - offset.y),
            max: Pos::new(self.max.x - offset.x, self.max.y - offset.y),
        }
    }

    #[inline]
    pub const fn add_size(self, size: Size) -> Self {
        Self {
            min: self.min,
            max: self.max.add_dims(size.width, size.height),
        }
    }

    #[inline]
    pub const fn sub_size(self, size: Size) -> Self {
        Self {
            min: self.min,
            max: self.max.sub_dims(size.width, size.height),
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
    pub const fn deflate(self, shrink: f32) -> Self {
        Self {
            min: Pos::new(self.min.x + shrink, self.min.y + shrink),
            max: Pos::new(self.max.x - shrink, self.max.y - shrink),
        }
    }

    #[inline]
    pub const fn deflate2(self, x: f32, y: f32) -> Self {
        Self {
            min: Pos::new(self.min.x + x, self.min.y + y),
            max: Pos::new(self.max.x - x, self.max.y - y),
        }
    }

    #[inline]
    pub const fn inflate(self, expand: f32) -> Self {
        Self {
            min: Pos::new(self.min.x - expand, self.min.y - expand),
            max: Pos::new(self.max.x + expand, self.max.y + expand),
        }
    }

    #[inline]
    pub const fn inflate2(self, x: f32, y: f32) -> Self {
        Self {
            min: Pos::new(self.min.x - x, self.min.y - y),
            max: Pos::new(self.max.x + x, self.max.y + y),
        }
    }

    /// Add a [Padding] to a [Rect].
    #[inline]
    pub const fn add_padding(self, padding: Padding) -> Self {
        Self {
            min: Pos::new(self.min.x + padding.left as f32, self.min.y + padding.top as f32),
            max: Pos::new(self.max.x - padding.right as f32, self.max.y - padding.bottom as f32),
        }
    }

    #[inline]
    pub const fn add_margin(self, margin: Margin) -> Self {
        let new_size = self.size().add_margin(margin);
        Self::from_min_size(self.min, new_size)
    }

    #[inline]
    pub const fn add_margin_anchored(self, margin: Margin, anchor: Anchor) -> Self {
        let pivot = self.anchor_pos(anchor);
        let new_size = self.size().add_margin(margin);
        Self::from_anchored_pivot(anchor, pivot, new_size)
    }

    /// Remove [Padding] from a [Rect].
    /// This is the inverse of `add_padding`.
    #[inline]
    pub const fn sub_padding(self, padding: Padding) -> Self {
        Self {
            min: Pos::new(self.min.x - padding.left as f32, self.min.y - padding.top as f32),
            max: Pos::new(self.max.x + padding.right as f32, self.max.y + padding.bottom as f32),
        }
    }

    #[inline]
    pub const fn sub_margin(self, margin: Margin) -> Self {
        let new_size = self.size().sub_margin(margin);
        Self::from_min_size(self.min, new_size)
    }

    #[inline]
    pub const fn sub_margin_anchored(self, margin: Margin, anchor: Anchor) -> Self {
        let pivot = self.anchor_pos(anchor);
        let new_size = self.size().sub_margin(margin);
        Self::from_anchored_pivot(anchor, pivot, new_size)
    }

    /// Applies [Padding] in-place, mutating the [Rect].
    #[inline]
    pub const fn apply_padding(&mut self, padding: Padding) {
        self.min.x += padding.left as f32;
        self.min.y += padding.top as f32;
        self.max.x -= padding.right as f32;
        self.max.y -= padding.bottom as f32;
    }

    #[inline]
    pub const fn apply_margin(&mut self, margin: Margin) {
        self.min.x -= margin.left as f32;
        self.min.y -= margin.top as f32;
        self.max.x += margin.right as f32;
        self.max.y += margin.bottom as f32;
    }

    /// Removes [Padding] in-place, mutating the [Rect].
    #[inline]
    pub const fn remove_padding(&mut self, padding: Padding) {
        self.min.x -= padding.left as f32;
        self.min.y -= padding.top as f32;
        self.max.x += padding.right as f32;
        self.max.y += padding.bottom as f32;
    }

    #[inline]
    pub const fn remove_margin(&mut self, margin: Margin) {
        self.min.x += margin.left as f32;
        self.min.y += margin.top as f32;
        self.max.x -= margin.right as f32;
        self.max.y -= margin.bottom as f32;
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

    #[inline]
    pub const fn handle_rect(self, anchor: Anchor, size: Size) -> Self {
        Self::centered(self.anchor_pos(anchor), size)
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

    #[inline]
    pub const fn aspect_ratio(self) -> f32 {
        self.width() / self.height()
    }

    /// Returns a rect inside of `self` that fits perfectly in the center
    /// by scaling `size`.
    pub const fn scale_inside(self, size: Size) -> Self {
        // determine which scaling method must be used.
        let msize = self.size();
        let mar = msize.aspect_ratio();
        let rar = size.aspect_ratio();
        let scalar = if mar >= rar {
            msize.height / size.height
        } else {
            msize.width / size.width
        };
        let new_size = size.scale(scalar);
        Rect::centered(self.center(), new_size)
    }

    pub const fn scale_middle(self, size: Size) -> Self {
        let msize = self.size();
        let mar = msize.aspect_ratio();
        let square_size = size.min_dims();
        let scale_by = if mar >= 1.0 {
            msize.height
        } else {
            msize.width
        };
        let scalar = scale_by / square_size;
        let new_size = size.scale(scalar);
        Rect::centered(self.center(), new_size)
    }

    /// Returns a rect outside of `self` that fits perfectly in the center
    /// by scaling `size`.
    pub const fn scale_outside(self, size: Size) -> Self {
        // determine which scaling method must be used.
        let msize = self.size();
        let mar = msize.aspect_ratio();
        let rar = size.aspect_ratio();
        let scalar = if mar >= rar {
            msize.width / size.width
        } else {
            msize.height / size.height
        };
        let new_size = size.scale(scalar);
        Rect::centered(self.center(), new_size)
    }

    #[inline]
    pub const fn lerp(self, other: Rect, t: f32) -> Self {
        Self::from_min_max(
            self.min.lerp(other.min, t),
            self.max.lerp(other.max, t),
        )
    }

    #[inline]
    pub const fn clamped_lerp(self, other: Rect, t: f32) -> Self {
        self.lerp(other, t.clamp(0.0, 1.0))
    }

    #[inline]
    pub fn map<R, F: FnOnce(Pos, Pos) -> R>(self, map: F) -> R {
        map(self.min, self.max)
    }

            // if pos.x < self.max.x
            // && pos.x >= self.min.x {
            //     // Check y
            //     let dtt = self.min.y - pos.y;
            //     let dtb = pos.y - self.max.y;
            //     dtt.min(dtb)
            // } else if pos.y < self.max.y
            // && pos.y >= self.max.y {
            //     // Check x
            //     let dtl = self.min.x - pos.x;
            //     let dtr = pos.x - self.min.x;
            //     dtl.min(dtr)
            // } else {

            // }
    pub fn sdf(self, pos: Pos) -> f32 {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn invalid(message: &'static str) -> ! {
            panic!("Invalid Rect: {message}");
        }
        let ge_min_x = pos.x >= self.min.x;
        let ge_min_y = pos.y >= self.min.y;
        let lt_max_x = pos.x < self.max.x;
        let lt_max_y = pos.y < self.max.y;
        match (ge_min_x, lt_max_x, ge_min_y, lt_max_y) {
            (true, true, true, true) => {
                let dtl = self.min.x - pos.x;
                let dtr = pos.x - self.max.x;
                let dtt = self.min.y - pos.y;
                let dtb = pos.y - self.max.y;
                dtl.max(dtr).max(dtt).max(dtb)
            },
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Between left and right, greater than max.y
            (true, true, true, false) => pos.y - self.max.y,
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Between left and right, less than min.y
            (true, true, false, true) => self.min.y - pos.y,
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            (true, true, false, false) => invalid("min.y is greater than max.y"),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Between top and bottom, greater than max.x
            (true, false, true, true) => pos.x - self.max.x,
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Greater than max.x, greater than max.y (right_bottom corner)
            (true, false, true, false) => self.right_bottom().distance(pos),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Greater than max.x, less than min.y (right_top corner)
            (true, false, false, true) => self.right_top().distance(pos),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            (true, false, false, false) => invalid("min.y is greater than max.y"),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Less than min.x, between top and bottom.
            (false, true, true, true) => self.min.x - pos.x,
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Less than min.x, greater than max.y (left_bottom corner)
            (false, true, true, false) => self.left_bottom().distance(pos),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Less than min.x, less than min.y (left_top corner)
            (false, true, false, true) => self.left_top().distance(pos),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            (false, true, false, false) => invalid("min.y is greater than max.y"),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            (false, false, true, true) => invalid("min.x is greater than max.x"),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            (false, false, true, false) => invalid("min.x is greater than max.x"),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            (false, false, false, true) => invalid("min.x is greater than max.x"),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            (false, false, false, false) => invalid("min is greater than max"),
        }
    }
}

impl std::ops::Add<Margin> for Rect {
    type Output = Rect;
    #[inline]
    fn add(self, rhs: Margin) -> Self::Output {
        self.add_margin(rhs)
    }
}

impl std::ops::Sub<Margin> for Rect {
    type Output = Rect;
    #[inline]
    fn sub(self, rhs: Margin) -> Self::Output {
        self.sub_margin(rhs)
    }
}

impl std::ops::Add<Padding> for Rect {
    type Output = Rect;
    #[inline]
    fn add(self, rhs: Padding) -> Self::Output {
        self.add_padding(rhs)
    }
}

impl std::ops::Sub<Padding> for Rect {
    type Output = Rect;
    #[inline]
    fn sub(self, rhs: Padding) -> Self::Output {
        self.sub_padding(rhs)
    }
}
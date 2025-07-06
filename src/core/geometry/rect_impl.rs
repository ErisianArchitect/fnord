use crate::core::geometry::is_positive;
use crate::core::geometry::util_impl::half;
use crate::core::geometry::Intercardinal;
use crate::core::math::lerp;

use super::pos_impl::*;
use super::size_impl::*;
use super::margin_impl::*;
use super::padding_impl::*;
use super::anchor_impl::Anchor;
use super::placement_impl::Placement;
use super::Grid;
use super::Axial;
use super::Cardinal;

#[repr(C)]
pub struct QuadSubdivide<T> {
    quadrants: [T; 4]
}

// 00 01 10 11

struct QuadSubDivideIndices {
    left_top: usize,
    right_top: usize,
    left_bottom: usize,
    right_bottom: usize,
}

impl<T> QuadSubdivide<T> {

    const I: QuadSubDivideIndices = QuadSubDivideIndices {
        left_top: 0,
        right_top: 1,
        left_bottom: 2,
        right_bottom: 3,
    };
    
    pub const fn left_top(&self) -> &T {
        &self.quadrants[Self::I.left_top]
    }

    pub const fn left_top_mut(&mut self) -> &mut T {
        &mut self.quadrants[Self::I.left_top]
    }

    pub const fn right_top(&self) -> &T {
        &self.quadrants[Self::I.right_top]
    }

    pub const fn right_top_mut(&mut self) -> &mut T {
        &mut self.quadrants[Self::I.right_top]
    }

    pub const fn left_bottom(&self) -> &T {
        &self.quadrants[Self::I.left_bottom]
    }

    pub const fn left_bottom_mut(&mut self) -> &mut T {
        &mut self.quadrants[Self::I.left_bottom]
    }

    pub const fn right_bottom(&self) -> &T {
        &self.quadrants[Self::I.right_bottom]
    }

    pub const fn right_bottom_mut(&mut self) -> &mut T {
        &mut self.quadrants[Self::I.right_bottom]
    }

    pub const fn as_slice(&self) -> &[T] {
        &self.quadrants
    }

    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.quadrants
    }
}

impl<T> std::ops::Index<(u32, u32)> for QuadSubdivide<T> {
    type Output = T;
    fn index(&self, index: (u32, u32)) -> &Self::Output {
        debug_assert!((index.0 | index.1) <= 1);
        let index = (index.0 | (index.1 << 1)) as usize;
        &self.quadrants[index]
    }
}

impl<T> std::ops::IndexMut<(u32, u32)> for QuadSubdivide<T> {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        debug_assert!((index.0 | index.1) <= 1);
        let index = (index.0 | (index.1 << 1)) as usize;
        &mut self.quadrants[index]
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Pos,
    pub max: Pos,
}

#[inline]
pub const fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    debug_assert!(Size::new(width, height).is_positive());
    Rect {
        min: Pos::new(x, y),
        max: Pos::new(x + width, y + height),
    }
}

impl Rect {
    pub const ZERO: Self = Self::from_min_max(Pos::ZERO, Pos::ZERO);
    pub const ONE: Self = Self::from_min_max(Pos::ZERO, Pos::ONE);

    /// Creates a [Rect] from the given minimum bound and maximum bound.
    /// 
    /// The following conditions must be met for this to result in a valid [Rect]:
    /// - `min.x <= max.x`
    /// - `min.y <= max.y`
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline]
    pub const fn from_min_max(min: Pos, max: Pos) -> Self {
        debug_assert!(min.le(max));
        Self {
            min,
            max,
        }
    }

    /// Creates a new [Rect] at the given `x` and `y` coordinate with the given `width` and `height`.
    /// 
    /// The following conditions must be met for this to result in a valid [Rect]:
    /// - `width >= 0`
    /// - `height >= 0`
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        debug_assert!(Size::new(width, height).is_positive());
        Self::from_min_max(
            Pos::new(x, y),
            Pos::new(x + width, y + height)
        )
    }

    /// Creates a new [Rect] with the given `min` position with the given `size`.
    /// 
    /// The following conditions must be met for this to result in a valid [Rect]:
    /// - `size.width >= 0`
    /// - `size.height >= 0`
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline]
    pub const fn from_min_size(min: Pos, size: Size) -> Self {
        debug_assert!(size.is_positive());
        Self::from_min_max(
            min,
            min.add_dims(size.width, size.height)
        )
    }

    /// Creates a new square [Rect] with the given `min` position and the given `side_length`.
    /// 
    /// The following condition must be met for this to result in a valid [Rect]:
    /// - `side_length >= 0`
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline]
    pub const fn square_from_min_size(min: Pos, side_length: f32) -> Self {
        debug_assert!(is_positive(side_length));
        Self::from_min_max(
            min,
            Pos::new(min.x + side_length, min.y + side_length)
        )
    }

    /// Creates a new [Rect] with the given `center` and the given `size`.
    /// 
    /// The following conditions must be met for this to result in a valid [Rect]:
    /// - `size.width >= 0`
    /// - `size.height >= 0`
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline]
    pub const fn centered(center: Pos, size: Size) -> Self {
        debug_assert!(size.is_positive());
        let half_size = size.mul_dims(0.5, 0.5);
        Self::from_min_max(
            center.sub_dims(half_size.width, half_size.height),
            center.add_dims(half_size.width, half_size.height)
        )
    }

    /// Creates a new square [Rect] with the given `center` and the given `side_length`.
    /// 
    /// The following condition must be met for this to result in a valid [Rect]:
    /// - `side_length >= 0`
    #[cfg_attr(debug_assertions, track_caller)]
    #[inline]
    pub const fn centered_square(center: Pos, side_length: f32) -> Self {
        debug_assert!(is_positive(side_length));
        let half_size = side_length * 0.5;
        Self::from_min_max(
            center.sub_dims(half_size, half_size),
            center.add_dims(half_size, half_size)
        )
    }

    /// Creates a new [Rect] anchored at the given `anchor` with the given `pivot` position and the given `size`.
    /// 
    #[cfg_attr(debug_assertions, track_caller)]
    pub const fn from_anchored_pivot(anchor: Anchor, pivot: Pos, size: Size) -> Rect {
        debug_assert!(size.is_positive());
        match anchor {
            Anchor::LeftTop => Self::from_min_size(pivot, size),
            Anchor::LeftCenter => {
                let half_height = size.half_height();
                Self::from_min_size(
                    Pos::new(pivot.x, pivot.y - half_height),
                    size,
                )
            },
            Anchor::LeftBottom => Self::from_min_size(
                Pos::new(pivot.x, pivot.y - size.height),
                size,
            ),
            Anchor::BottomCenter => {
                let half_width = size.half_width();
                Self::from_min_size(
                    Pos::new(pivot.x - half_width, pivot.y - size.height),
                    size,
                )
            },
            Anchor::RightBottom => Self::from_min_size(
                Pos::new(pivot.x - size.width, pivot.y - size.height),
                size,
            ),
            Anchor::RightCenter => {
                let half_height = size.half_height();
                Self::from_min_size(
                    Pos::new(pivot.x - size.width, pivot.y - half_height),
                    size,
                )
            },
            Anchor::RightTop => Self::from_min_size(
                Pos::new(pivot.x - size.width, pivot.y),
                size,
            ),
            Anchor::TopCenter => {
                let half_width = size.half_width();
                Self::from_min_size(
                    Pos::new(pivot.x - half_width, pivot.y),
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

    #[cfg_attr(debug_assertions, track_caller)]
    #[inline]
    pub const fn from_points_slice(slice: &[Pos]) -> Self {
        debug_assert!(slice.len() >= 2);
        Self {
            min: slice[0].min(slice[1]),
            max: slice[1].max(slice[0]),
        }
    }

    #[inline]
    pub const fn grid(self) -> Grid {
        Grid::from_rect(self)
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
    pub const fn with_size(self, size: Size) -> Self {
        Self::from_min_max(
            self.min,
            self.max.add_dims(size.width, size.height)
        )
    }

    #[inline]
    pub const fn with_size_centered(self, size: Size) -> Self {
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

    #[inline]
    pub const fn with_size_anchored(self, size: Size, anchor: Anchor) -> Self {
        Self::from_anchored_pivot(anchor, self.anchor(anchor), size)
    }

    #[inline]
    pub const fn set_size(&mut self, size: Size) {
        self.max = self.min.add_dims(size.width, size.height);
    }

    #[inline]
    pub const fn set_size_centered(&mut self, size: Size) {
        *self = self.with_size_centered(size)
    }

    #[inline]
    pub const fn set_size_anchored(&mut self, size: Size, anchor: Anchor) {
        *self = self.with_size_anchored(size, anchor)
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
    pub const fn with_width(mut self, width: f32) -> Self {
        self.set_width(width);
        self
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
    pub const fn with_width_centered(mut self, width: f32) -> Self {
        self.set_width_centered(width);
        self
    }

    /// Sets the width from the right side (so the right side remains the same).
    #[inline]
    pub const fn set_width_right(&mut self, width: f32) {
        self.min.x = self.max.x - width;
    }

    #[inline]
    pub const fn with_width_right(mut self, width: f32) -> Self {
        self.set_width_right(width);
        self
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
    pub const fn with_height(mut self, height: f32) -> Self {
        self.set_height(height);
        self
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
    pub const fn with_height_centered(mut self, height: f32) -> Self {
        self.set_height_centered(height);
        self
    }

    #[inline]
    pub const fn set_height_bottom(&mut self, height: f32) {
        self.min.y = self.max.y - height;
    }

    #[inline]
    pub const fn with_height_bottom(mut self, height: f32) -> Self {
        self.set_height_bottom(height);
        self
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
    pub const fn with_left(mut self, left: f32) -> Self {
        self.set_left(left);
        self
    }

    #[inline]
    pub const fn set_left_bound(&mut self, left: f32) {
        self.min.x = left;
    }

    #[inline]
    pub const fn with_left_bound(mut self, left: f32) -> Self {
        self.set_left_bound(left);
        self
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
    pub const fn with_right(mut self, right: f32) -> Self {
        self.set_right(right);
        self
    }

    #[inline]
    pub const fn set_right_bound(&mut self, right: f32) {
        self.max.x = right;
    }

    #[inline]
    pub const fn with_right_bound(mut self, right: f32) -> Self {
        self.set_right_bound(right);
        self
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
    pub const fn with_top(mut self, top: f32) -> Self {
        self.set_top(top);
        self
    }

    #[inline]
    pub const fn set_top_bound(&mut self, top: f32) {
        self.min.y = top;
    }

    #[inline]
    pub const fn with_top_bound(mut self, top: f32) -> Self {
        self.set_top_bound(top);
        self
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
    pub const fn with_bottom(mut self, bottom: f32) -> Self {
        self.set_bottom(bottom);
        self
    }

    #[inline]
    pub const fn set_bottom_bound(&mut self, bottom: f32) {
        self.max.y = bottom;
    }

    #[inline]
    pub const fn with_bottom_bound(mut self, bottom: f32) -> Self {
        self.set_bottom_bound(bottom);
        self
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
    pub const fn with_left_top(mut self, left_top: Pos) -> Self {
        self.set_left_top(left_top);
        self
    }

    #[inline]
    pub const fn set_left_top_bound(&mut self, left_top: Pos) {
        self.min = left_top;
    }

    #[inline]
    pub const fn with_left_top_bound(mut self, left_top: Pos) -> Self {
        self.set_left_top_bound(left_top);
        self
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
    pub const fn with_right_top(mut self, right_top: Pos) -> Self {
        self.set_right_top(right_top);
        self
    }

    #[inline]
    pub const fn set_right_top_bound(&mut self, right_top: Pos) {
        self.max.x = right_top.x;
        self.min.y = right_top.y;
    }

    #[inline]
    pub const fn with_right_top_bound(mut self, right_top: Pos) -> Self {
        self.set_right_top_bound(right_top);
        self
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
    pub const fn with_left_bottom(mut self, left_bottom: Pos) -> Self {
        self.set_left_bottom(left_bottom);
        self
    }

    #[inline]
    pub const fn set_left_bottom_bound(&mut self, left_bottom: Pos) {
        self.min.x = left_bottom.x;
        self.max.y = left_bottom.y;
    }

    #[inline]
    pub const fn with_left_bottom_bound(mut self, left_bottom: Pos) -> Self {
        self.set_left_bottom_bound(left_bottom);
        self
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
    pub const fn with_right_bottom(mut self, right_bottom: Pos) -> Self {
        self.set_right_bottom(right_bottom);
        self
    }

    #[inline]
    pub const fn set_right_bottom_bound(&mut self, right_bottom: Pos) {
        self.max = right_bottom;
    }

    #[inline]
    pub const fn with_right_bottom_bound(mut self, right_bottom: Pos) -> Self {
        self.set_right_bottom_bound(right_bottom);
        self
    }

    #[inline]
    pub const fn left_center(self) -> Pos {
        // We assume that self.max.y >= self.min.y
        let center_y = self.min.y.midpoint(self.max.y);
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
    pub const fn with_left_center(mut self, left_center: Pos) -> Self {
        self.set_left_center(left_center);
        self
    }

    #[inline]
    pub const fn top_center(self) -> Pos {
        let center_x = self.min.x.midpoint(self.max.x);
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
    pub const fn with_top_center(mut self, top_center: Pos) -> Self {
        self.set_top_center(top_center);
        self
    }

    #[inline]
    pub const fn right_center(self) -> Pos {
        let center_y = self.min.y.midpoint(self.max.y);
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
    pub const fn with_right_center(mut self, right_center: Pos) -> Self {
        self.set_right_center(right_center);
        self
    }

    #[inline]
    pub const fn bottom_center(self) -> Pos {
        let center_x = self.min.x.midpoint(self.max.x);
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
    pub const fn with_bottom_center(mut self, bottom_center: Pos) -> Self {
        self.set_bottom_center(bottom_center);
        self
    }

    #[inline]
    pub const fn center(self) -> Pos {
        let center_x = self.min.x.midpoint(self.max.x);
        let center_y = self.min.y.midpoint(self.max.y);
        Pos::new(center_x, center_y)
    }

    #[inline]
    pub const fn set_center(&mut self, center: Pos) {
        let half_size = self.size().half();
        self.min = pos(center.x - half_size.width, center.y - half_size.height);
        self.max = pos(center.x + half_size.width, center.y + half_size.height);
    }

    #[inline]
    pub const fn with_center(mut self, center: Pos) -> Self {
        self.set_center(center);
        self
    }

    #[inline]
    pub const fn place_anchor(&mut self, anchor: Anchor, pos: Pos) {
        match anchor {
            Anchor::LeftTop => self.set_left_top(pos),
            Anchor::LeftCenter => self.set_left_center(pos),
            Anchor::LeftBottom => self.set_left_bottom(pos),
            Anchor::BottomCenter => self.set_bottom_center(pos),
            Anchor::RightBottom => self.set_right_bottom(pos),
            Anchor::RightCenter => self.set_right_center(pos),
            Anchor::RightTop => self.set_right_top(pos),
            Anchor::TopCenter => self.set_top_center(pos),
            Anchor::Center => self.set_center(pos),
        }
    }

    #[inline]
    pub const fn place_anchor_bound(&mut self, anchor: Anchor, pos: Pos) {
        match anchor {
            Anchor::LeftTop => self.set_left_top_bound(pos),
            Anchor::LeftCenter => self.set_left_bound(pos.x),
            Anchor::LeftBottom => self.set_left_bottom_bound(pos),
            Anchor::BottomCenter => self.set_bottom_bound(pos.y),
            Anchor::RightBottom => self.set_right_bottom_bound(pos),
            Anchor::RightCenter => self.set_right_bound(pos.x),
            Anchor::RightTop => self.set_right_top_bound(pos),
            Anchor::TopCenter => self.set_top_bound(pos.y),
            Anchor::Center => self.set_center(pos),
        }
    }

    #[inline]
    pub const fn with_placed_anchor(mut self, anchor: Anchor, pos: Pos) -> Self {
        self.place_anchor(anchor, pos);
        self
    }

    #[inline]
    pub const fn move_to_anchor(&mut self, anchor: Anchor) {
        match anchor {
            Anchor::LeftTop => {
                // verified
                let pos = self.left_top();
                self.set_right_bottom(pos);
            },
            Anchor::LeftCenter => {
                // verified
                let pos = self.left_center();
                self.set_right_center(pos);
            },
            Anchor::LeftBottom => {
                // verified
                let pos = self.left_bottom();
                self.set_right_top(pos);
            },
            Anchor::BottomCenter => {
                // verified
                let pos = self.bottom_center();
                self.set_top_center(pos);
            },
            Anchor::RightBottom => {
                // verified
                let pos = self.right_bottom();
                self.set_left_top(pos);
            },
            Anchor::RightCenter => {
                // verified
                let pos = self.right_center();
                self.set_left_center(pos);
            },
            Anchor::RightTop => {
                // verified
                let pos = self.right_top();
                self.set_left_bottom(pos);
            },
            Anchor::TopCenter => {
                // verified
                let pos = self.top_center();
                self.set_bottom_center(pos);
            },
            Anchor::Center => (),
        }
    }

    #[must_use]
    #[inline]
    pub const fn moved_to_anchor(mut self, anchor: Anchor) -> Self {
        self.move_to_anchor(anchor);
        self
    }

    pub const fn move_on_grid(&mut self, x: i32, y: i32) {
        let x_offset = self.width() * x as f32;
        let y_offset = self.height() * y as f32;
        self.min.x += x_offset;
        self.min.y += y_offset;
        self.max.x += x_offset;
        self.max.y += y_offset;
    }

    #[must_use]
    #[inline]
    pub const fn moved_on_grid(mut self, x: i32, y: i32) -> Self {
        self.move_on_grid(x, y);
        self
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
    pub const fn with_uv_pos(mut self, uv: Pos, pos: Pos) -> Self {
        self.set_uv_pos(uv, pos);
        self
    }

    #[inline]
    pub const fn contains(self, pos: Pos) -> bool {
        self.min.x <= pos.x && self.min.y <= pos.y
        && self.max.x > pos.x && self.max.y > pos.y
    }

    #[inline]
    pub const fn contains_rect(self, rect: Rect) -> bool {
        self.min.x <= rect.min.x && self.max.x >= rect.max.x
        && self.min.y <= rect.min.y && self.max.y >= rect.max.y
    }

    #[inline]
    pub const fn inside_rect(self, rect: Rect) -> bool {
        rect.contains_rect(self)
    }

    #[inline]
    pub const fn outside_rect(self, rect: Rect) -> bool {
        self.max.x < rect.min.x || self.max.y < rect.min.y
        || self.min.x >= rect.max.x || self.min.y >= rect.max.y
    }

    #[inline]
    pub const fn intersects(self, rect: &Rect) -> bool {
        rect.min.x < self.max.x && rect.min.y < self.max.y
        && rect.max.x > self.min.x && rect.max.y > self.min.y
    }

    #[inline]
    pub const fn intersection(self, other: Rect) -> Option<Rect> {
        if !self.intersects(&other) {
            return None;
        }
        let left = self.min.x.max(other.min.x);
        let top = self.min.y.max(other.min.y);
        let right = self.max.x.min(other.max.x);
        let bottom = self.max.y.min(other.max.y);
        Some(Self::from_min_max(
            pos(left, top),
            pos(right, bottom)
        ))
    }

    #[inline]
    pub const fn intersect_all(rects: &[Rect]) -> Option<Rect> {
        let mut intersection = match rects.len() {
            0 => return None,
            1 => return Some(rects[0]),
            _ => rects[0],
        };
        let mut index = 1;
        while index < rects.len() {
            let Some(new_rect) = intersection.intersection(rects[index]) else {
                return None;
            };
            intersection = new_rect;
            index += 1;
        }
        Some(intersection)
    }

    #[inline]
    pub const fn translate(&mut self, offset: Pos) {
        self.min = Pos::new(self.min.x + offset.x, self.min.y + offset.y);
        self.max = Pos::new(self.max.x + offset.x, self.max.y + offset.y);
    }

    #[inline]
    pub const fn with_translation(mut self, offset: Pos) -> Self {
        self.translate(offset);
        self
    }

    #[inline]
    pub const fn inv_translate(&mut self, offset: Pos) {
        self.min = Pos::new(self.min.x - offset.x, self.min.y - offset.y);
        self.max = Pos::new(self.max.x - offset.x, self.max.y - offset.y);
    }

    #[inline]
    pub const fn with_inv_translation(mut self, offset: Pos) -> Self {
        self.inv_translate(offset);
        self
    }

    #[inline]
    pub const fn add_offset(self, offset: Pos) -> Self {
        Self::from_min_max(
            Pos::new(self.min.x + offset.x, self.min.y + offset.y),
            Pos::new(self.max.x + offset.x, self.max.y + offset.y)
        )
    }

    #[inline]
    pub const fn sub_offset(self, offset: Pos) -> Self {
        Self::from_min_max(
            Pos::new(self.min.x - offset.x, self.min.y - offset.y),
            Pos::new(self.max.x - offset.x, self.max.y - offset.y)
        )
    }

    #[inline]
    pub const fn add_size(self, size: Size) -> Self {
        Self::from_min_max(
            self.min,
            self.max.add_dims(size.width, size.height)
        )
    }

    #[inline]
    pub const fn add_size_centered(self, size: Size) -> Self {
        self.inflate2(size.width, size.height)
    }

    #[inline]
    pub const fn sub_size(self, size: Size) -> Self {
        Self::from_min_max(
            self.min,
            self.max.sub_dims(size.width, size.height)
        )
    }

    #[inline]
    pub const fn sub_size_centered(self, size: Size) -> Self {
        self.deflate2(size.width, size.height)
    }

    #[inline]
    pub const fn anchor(&self, anchor: Anchor) -> Pos {
        match anchor {
            Anchor::LeftTop => self.left_top(),
            Anchor::LeftCenter => self.left_center(),
            Anchor::LeftBottom => self.left_bottom(),
            Anchor::BottomCenter => self.bottom_center(),
            Anchor::RightBottom => self.right_bottom(),
            Anchor::RightCenter => self.right_center(),
            Anchor::RightTop => self.right_top(),
            Anchor::TopCenter => self.top_center(),
            Anchor::Center => self.center(),
        }
    }

    #[inline]
    pub const fn inflate(self, expand: f32) -> Self {
        Self::from_min_max(
            Pos::new(self.min.x - expand, self.min.y - expand),
            Pos::new(self.max.x + expand, self.max.y + expand)
        )
    }

    #[inline]
    pub const fn inflate2(self, x: f32, y: f32) -> Self {
        Self::from_min_max(
            Pos::new(self.min.x - x, self.min.y - y),
            Pos::new(self.max.x + x, self.max.y + y)
        )
    }

    #[inline]
    pub const fn deflate(self, shrink: f32) -> Self {
        Self::from_min_max(
            Pos::new(self.min.x + shrink, self.min.y + shrink),
            Pos::new(self.max.x - shrink, self.max.y - shrink)
        )
    }

    #[inline]
    pub const fn deflate2(self, x: f32, y: f32) -> Self {
        Self::from_min_max(
            Pos::new(self.min.x + x, self.min.y + y),
            Pos::new(self.max.x - x, self.max.y - y)
        )
    }

    /// Scales the rect by the given factor while keeping the same minimum bound.
    #[inline]
    pub const fn set_scale(&mut self, scalar: f32) {
        self.set_size(self.size().scale(scalar));
    }

    #[must_use]
    #[inline]
    pub const fn with_scale(mut self, scalar: f32) -> Self {
        self.set_scale(scalar);
        self
    }

    #[inline]
    pub const fn set_scale_centered(&mut self, scalar: f32) {
        self.set_size_centered(self.size().scale(scalar));
    }

    #[must_use]
    #[inline]
    pub const fn with_scale_centered(mut self, scalar: f32) -> Self {
        self.set_scale_centered(scalar);
        self
    }

    #[inline]
    pub const fn set_scale_anchored(&mut self, scalar: f32, anchor: Anchor) {
        self.set_size_anchored(self.size().scale(scalar), anchor);
    }

    #[must_use]
    #[inline]
    pub const fn with_scale_anchored(mut self, scalar: f32, anchor: Anchor) -> Self {
        self.set_scale_anchored(scalar, anchor);
        self
    }

    /// Sets the position of the rect relative to `pivot` where `pivot` is moved to `pos`
    /// and the rect remains relative to `pivot`.
    #[inline]
    pub const fn set_relative_position(&mut self, pivot: Pos, pos: Pos) {
        let min_offset = self.min.sub(pivot);
        let max_offset = self.max.sub(pivot);
        self.min = pos.add(min_offset);
        self.max = pos.add(max_offset);
    }

    /// Add a [Padding] to a [Rect].
    #[inline]
    pub const fn add_padding(self, padding: Padding) -> Self {
        Self::from_min_max(
            Pos::new(self.min.x + padding.left, self.min.y + padding.top),
            Pos::new(self.max.x - padding.right, self.max.y - padding.bottom)
        )
    }

    /// Adds the [Margin] to the rect while maintaining the min bound.
    #[inline]
    pub const fn add_margin(self, margin: Margin) -> Self {
        Self::from_min_max(
            self.min,
            self.max.add_dims(margin.x(), margin.y())
        )
    }

    #[inline]
    pub const fn add_margin_centered(self, margin: Margin) -> Self {
        Self::from_min_max(
            Pos::new(
                self.min.x - margin.left,
                self.min.y - margin.top,
            ),
            Pos::new(
                self.max.x + margin.right,
                self.max.y + margin.bottom,
            )
        )
    }

    #[inline]
    pub const fn add_margin_anchored(self, margin: Margin, anchor: Anchor) -> Self {
        let pivot = self.anchor(anchor);
        let new_size = self.size().add_margin(margin);
        Self::from_anchored_pivot(anchor, pivot, new_size)
    }

    /// Remove [Padding] from a [Rect].
    /// This is the inverse of `add_padding`.
    #[inline]
    pub const fn sub_padding(self, padding: Padding) -> Self {
        Self::from_min_max(
            Pos::new(self.min.x - padding.left, self.min.y - padding.top),
            Pos::new(self.max.x + padding.right, self.max.y + padding.bottom)
        )
    }

    #[inline]
    pub const fn sub_margin(self, margin: Margin) -> Self {
        Self::from_min_max(
            self.min,
            self.max.sub_dims(margin.x(), margin.y())
        )
    }

    #[inline]
    pub const fn sub_margin_centered(self, margin: Margin) -> Self {
        Self::from_min_max(
            Pos::new(
                self.min.x + margin.left,
                self.min.y + margin.top,
            ),
            Pos::new(
                self.max.x - margin.right,
                self.max.y - margin.bottom,
            )
        )
    }

    #[inline]
    pub const fn sub_margin_anchored(self, margin: Margin, anchor: Anchor) -> Self {
        let pivot = self.anchor(anchor);
        let new_size = self.size().sub_margin(margin);
        Self::from_anchored_pivot(anchor, pivot, new_size)
    }

    /// Applies [Padding] in-place, mutating the [Rect].
    #[inline]
    pub const fn apply_padding(&mut self, padding: Padding) {
        self.min.x += padding.left;
        self.min.y += padding.top;
        self.max.x -= padding.right;
        self.max.y -= padding.bottom;
    }

    #[inline]
    pub const fn apply_margin(&mut self, margin: Margin) {
        self.min.x -= margin.left;
        self.min.y -= margin.top;
        self.max.x += margin.right;
        self.max.y += margin.bottom;
    }

    /// Removes [Padding] in-place, mutating the [Rect].
    #[inline]
    pub const fn remove_padding(&mut self, padding: Padding) {
        self.min.x -= padding.left;
        self.min.y -= padding.top;
        self.max.x += padding.right;
        self.max.y += padding.bottom;
    }

    #[inline]
    pub const fn remove_margin(&mut self, margin: Margin) {
        self.min.x += margin.left;
        self.min.y += margin.top;
        self.max.x -= margin.right;
        self.max.y -= margin.bottom;
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

    #[inline]
    pub const fn left_top_adjacent(self, size: Size) -> Self {
        Self::from_min_max(
            self.min.sub_dims(size.width, size.height),
            self.min,
        )
    }

    #[inline]
    pub const fn right_top_adjacent(self, size: Size) -> Self {
        Self::from_min_max(
            Pos::new(self.max.x, self.min.y - size.height),
            Pos::new(self.max.x + size.width, self.min.y)
        )
    }

    #[inline]
    pub const fn left_bottom_adjacent(self, size: Size) -> Self {
        Self::from_min_max(
            Pos::new(self.min.x - size.width, self.max.y),
            Pos::new(self.min.x, self.max.y + size.height)
        )
    }

    #[inline]
    pub const fn right_bottom_adjacent(self, size: Size) -> Self {
        Self::from_min_max(
            self.max,
            self.max.add_dims(size.width, size.height)
        )
    }

    pub const fn handle_rect(self, anchor: Anchor, placement: Placement, size: f32) -> Self {
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
    pub const fn pivot_rect(self, anchor: Anchor, size: Size) -> Self {
        Self::centered(self.anchor(anchor), size)
    }

    #[inline]
    pub const fn square_pivot_rect(self, anchor: Anchor, size: f32) -> Self {
        Self::centered_square(self.anchor(anchor), size)
    }

    #[inline]
    pub const fn aspect_ratio(self) -> f32 {
        // divide by width by aspect ratio to get height
        // multiply height by aspect ratio to get width
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
                // Since the pos is inside, these are all negative.
                // Then you get the maximum of them all and that will
                // be the closest negative distance.
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

    #[track_caller]
    pub fn closest_point(self, pos: Pos) -> Pos {
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
                // distance-to-left
                let dtl = pos.x - self.min.x;
                let mut min = dtl;
                let mut closest = Pos::new(self.min.x, pos.y);
                // distance-to-right
                let dtr = self.max.x - pos.x;
                if dtr < min {
                    min = dtr;
                    closest.x = self.max.x;
                }
                // distance-to-top
                let dtt = pos.y - self.min.y;
                if dtt < min {
                    min = dtt;
                    closest = Pos::new(pos.x, self.min.y);
                } 
                // distance-to-bottom
                let dtb = self.max.y - pos.y;
                if dtb < min {
                    closest = Pos::new(pos.x, self.max.y)
                }
                closest
            },
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Between left and right, greater than max.y
            (true, true, true, false) => Pos::new(pos.x, self.max.y),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Between left and right, less than min.y
            (true, true, false, true) => Pos::new(pos.x, self.min.y),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            (true, true, false, false) => invalid("min.y is greater than max.y"),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Between top and bottom, greater than max.x
            (true, false, true, true) => Pos::new(self.max.x, pos.y),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Greater than max.x, greater than max.y (right_bottom corner)
            (true, false, true, false) => self.max,
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Greater than max.x, less than min.y (right_top corner)
            (true, false, false, true) => self.right_top(),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            (true, false, false, false) => invalid("min.y is greater than max.y"),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Less than min.x, between top and bottom.
            (false, true, true, true) => Pos::new(self.min.x, pos.y),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Less than min.x, greater than max.y (left_bottom corner)
            (false, true, true, false) => self.left_bottom(),
            // (ge_min_x, lt_max_x, ge_min_y, lt_max_y)
            // Less than min.x, less than min.y (left_top corner)
            (false, true, false, true) => self.left_top(),
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

    /// Effectively swaps the width and height, lefting the min at the same location.
    #[inline]
    pub const fn swapped_lengths(self) -> Self {
        Self::from_min_size(self.min, self.size().swap_dims())
    }

    #[inline]
    pub const fn swap_lengths(&mut self) {
        self.set_size(self.size().swap_dims());
    }

    #[inline]
    pub const fn centered_swapped_length(self) -> Self {
        Self::centered(self.center(), self.size().swap_dims())
    }

    #[inline]
    pub const fn centered_swap_lengths(&mut self) {
        self.set_size_centered(self.size().swap_dims());
    }

    #[inline]
    pub const fn anchored_swapped_lengths(self, anchor: Anchor) -> Self {
        Self::from_anchored_pivot(anchor, self.anchor(anchor), self.size().swap_dims())
    }

    #[inline]
    pub const fn anchored_swap_lengths(&mut self, anchor: Anchor) {
        *self = Self::from_anchored_pivot(anchor, self.anchor(anchor), self.size().swap_dims());
    }

    /// Returns `(left, right)`
    #[must_use]
    #[inline]
    pub const fn split_horizontal(self) -> (Self, Self) {
        let middle = self.min.x.midpoint(self.max.x);
        (
            Rect::from_min_max(
                self.min,
                Pos::new(middle, self.max.y)
            ),
            Rect::from_min_max(
                Pos::new(middle, self.min.y),
                self.max
            )
        )
    }

    /// Returns `(top, bottom)`
    #[must_use]
    #[inline]
    pub const fn split_vertical(self) -> (Self, Self) {
        let middle = self.min.y.midpoint(self.max.y);
        (
            Rect::from_min_max(
                self.min,
                Pos::new(self.max.x, middle)
            ),
            Rect::from_min_max(
                Pos::new(self.min.x, middle),
                self.max
            )
        )
    }

    #[must_use]
    #[inline]
    pub const fn into_quadrants(self) -> QuadSubdivide<Self> {
        let mid_x = self.min.x.midpoint(self.max.x);
        let mid_y = self.min.y.midpoint(self.max.y);
        let mid = Pos::new(mid_x, mid_y);
        QuadSubdivide {
            quadrants: [
                // left_top
                Rect::from_min_max(
                    Pos::new(self.min.x, self.min.y),
                    mid
                ),
                // right_top
                Rect::from_min_max(
                    Pos::new(mid_x, self.min.y),
                    Pos::new(self.max.x, mid_y)
                ),
                // left_bottom
                Rect::from_min_max(
                    Pos::new(self.min.x, mid_y),
                    Pos::new(mid_x, self.max.y)
                ),
                // right_bottom
                Rect::from_min_max(
                    mid,
                    Pos::new(self.max.x, self.max.y),
                )
            ]
        }
    }

    #[must_use]
    #[inline]
    pub fn hypotenuse(self) -> f32 {
        self.min.distance(self.max)
    }

    #[must_use]
    #[inline]
    pub const fn hypotenuse_squared(self) -> f32 {
        self.min.distance_squared(self.max)
    }

    #[must_use]
    #[inline]
    pub const fn corner(self, corner: Intercardinal) -> Pos {
        match corner {
            Intercardinal::Nw => self.left_top(),
            Intercardinal::Ne => self.right_top(),
            Intercardinal::Se => self.right_bottom(),
            Intercardinal::Sw => self.left_bottom(),
        }
    }

    /// Returns corners in the following order:
    /// - Top Left (`self.left_top()`)
    /// - Top Right (`self.right_top()`)
    /// - Bottom Left (`self.left_bottom()`)
    /// - Bottom Right (`self.right_bottom()`)
    #[must_use]
    #[inline]
    pub const fn corners(self) -> [Pos; 4] {
        [
            self.left_top(), self.right_top(),
            self.left_bottom(), self.right_bottom(),
        ]
    }

    /// Returns corners in the following order:
    /// - Top Left (`self.left_top()`)
    /// - Top Right (`self.right_top()`)
    /// - Bottom Right (`self.right_bottom()`)
    /// - Bottom Left (`self.left_bottom()`)
    #[must_use]
    #[inline]
    pub const fn corners_cw(self) -> [Pos; 4] {
        [
            self.left_top(),
            self.right_top(),
            self.right_bottom(),
            self.left_bottom(),
        ]
    }

    /// Returns corners in the following order:
    /// - Top Left (`self.left_top()`)
    /// - Bottom Left (`self.left_bottom()`)
    /// - Bottom Right (`self.right_bottom()`)
    /// - Top Right (`self.right_top()`)
    #[must_use]
    #[inline]
    pub const fn corners_ccw(self) -> [Pos; 4] {
        [
            self.left_top(),
            self.left_bottom(),
            self.right_bottom(),
            self.right_top(),
        ]
    }

    #[must_use]
    #[inline]
    pub const fn edge_midpoint(self, edge: Axial) -> Pos {
        match edge {
            Axial::Left => self.left_center(),
            Axial::Up => self.top_center(),
            Axial::Right => self.right_center(),
            Axial::Down => self.bottom_center(),
        }
    }

    #[must_use]
    #[inline]
    pub const fn edge_points_cw(self, edge: Axial) -> [Pos; 2] {
        match edge {
            Axial::Left => [
                self.left_bottom(),
                self.left_top(),
            ],
            Axial::Up => [
                self.left_top(),
                self.right_top(),
            ],
            Axial::Right => [
                self.right_top(),
                self.right_bottom(),
            ],
            Axial::Down => [
                self.right_bottom(),
                self.left_bottom(),
            ],
        }
    }

    #[must_use]
    #[inline]
    pub const fn edge_points_ccw(self, edge: Axial) -> [Pos; 2] {
        match edge {
            Axial::Left => [
                self.left_top(),
                self.left_bottom(),
            ],
            Axial::Up => [
                self.right_top(),
                self.left_top(),
            ],
            Axial::Right => [
                self.right_bottom(),
                self.right_top(),
            ],
            Axial::Down => [
                self.left_bottom(),
                self.right_bottom(),
            ],
        }
    }

    #[must_use]
    #[inline]
    pub const fn edge_points_min_max(self, edge: Axial) -> [Pos; 2] {
        match edge {
            Axial::Left => [
                self.left_top(),
                self.left_bottom(),
            ],
            Axial::Up => [
                self.left_top(),
                self.right_top(),
            ],
            Axial::Right => [
                self.right_top(),
                self.right_bottom(),
            ],
            Axial::Down => [
                self.left_bottom(),
                self.right_bottom(),
            ],
        }
    }

    #[must_use]
    #[inline]
    pub const fn edge_points_max_min(self, edge: Axial) -> [Pos; 2] {
        match edge {
            Axial::Left => [
                self.left_bottom(),
                self.left_top(),
            ],
            Axial::Up => [
                self.right_top(),
                self.left_top(),
            ],
            Axial::Right => [
                self.right_bottom(),
                self.right_top(),
            ],
            Axial::Down => [
                self.right_bottom(),
                self.left_bottom(),
            ],
        }
    }

    /// Extends the bounds of self so that `rect` fits inside exactly.
    #[inline]
    pub const fn extend_to_fit(&mut self, rect: Rect) {
        self.min.x = self.min.x.min(rect.min.x);
        self.min.y = self.min.y.min(rect.min.y);
        self.max.x = self.max.x.max(rect.max.x);
        self.max.y = self.max.y.max(rect.max.y);
    }

    #[must_use]
    #[inline]
    pub const fn extended_to_fit(mut self, rect: Rect) -> Self {
        self.extend_to_fit(rect);
        self
    }

    /// Gets the smallest [Rect] that can contain all `rects`.
    /// 
    /// Returns [Rect::ZERO] if the slice is empty.
    #[must_use]
    pub const fn min_rect(rects: &[Self]) -> Self {
        let Some((min_rect, rects)) = rects.split_first() else {
            return Self::ZERO;
        };
        let mut min_rect = *min_rect;
        let mut index = 0;
        while index < rects.len() {
            min_rect.extend_to_fit(rects[index]);
            index += 1;
        }
        min_rect
    }

    #[must_use]
    pub fn subdivision_containing(self, pos: Pos, cols: u32, rows: u32) -> Option<Self> {
        if !self.contains(pos) || cols == 0 || rows == 0 {
            return None;
        }
        let size = self.size();
        let cell_width = size.width / cols as f32;
        let cell_height = size.height / rows as f32;
        let inner_pos = pos.sub(self.min);
        let cell_min = inner_pos
            .div_dims(cell_width, cell_height)
            .floor()
            .mul_dims(cell_width, cell_height)
            .add(self.min);
        Some(Self::from_min_size(cell_min, Size::new(cell_width, cell_height)))
    }

    #[must_use]
    pub fn subdivision_containing_rect(self, rect: Self, cols: u32, rows: u32) -> Option<Self> {
        if !self.contains_rect(rect) {
            return None;
        }
        let size = self.size();
        let cell_width = size.width / cols as f32;
        let cell_height = size.height / rows as f32;
        let inner_rect = rect.sub_offset(self.min);
        let cell_min = inner_rect.min
            .div_dims(cell_width, cell_height)
            .floor()
            .to_ituple();
        let cell_max = inner_rect.max
            .div_dims(cell_width, cell_height)
            .floor()
            .to_ituple();
        if cell_min != cell_max {
            return Some(self);
        }
        let min = Pos::new(cell_min.0 as f32 * cell_width, cell_min.1 as f32 * cell_height);
        let max = min.add_dims(cell_width, cell_height);
        Some(Self::from_min_max(min, max))
        
    }

    #[must_use]
    pub fn floor(self) -> Self {
        Self::from_min_max(
            self.min.floor(),
            self.max.floor()
        )
    }

    #[must_use]
    pub fn ceil(self) -> Self {
        Self::from_min_max(
            self.min.ceil(),
            self.max.ceil()
        )
    }

    /// Floors the min bound and ceils the max bound.
    #[must_use]
    pub fn floor_ceil(self) -> Self {
        Self::from_min_max(
            self.min.floor(),
            self.max.ceil()
        )
    }

    #[must_use]
    pub fn ceil_floor(self) -> Self {
        Self::from_min_max(
            self.min.ceil(),
            self.max.floor()
        )
    }

    #[must_use]
    pub fn round(self) -> Self {
        Self::from_min_max(
            self.min.round(),
            self.max.round()
        )
    }
}

impl std::ops::Add<Margin> for Rect {
    type Output = Rect;
    #[inline]
    fn add(self, rhs: Margin) -> Self::Output {
        self.add_margin_centered(rhs)
    }
}

impl std::ops::Sub<Margin> for Rect {
    type Output = Rect;
    #[inline]
    fn sub(self, rhs: Margin) -> Self::Output {
        self.sub_margin_centered(rhs)
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

impl std::ops::Add<Pos> for Rect {
    type Output = Rect;
    #[inline]
    fn add(self, rhs: Pos) -> Self::Output {
        self.add_offset(rhs)
    }
}

impl std::ops::Sub<Pos> for Rect {
    type Output = Rect;
    #[inline]
    fn sub(self, rhs: Pos) -> Self::Output {
        self.sub_offset(rhs)
    }
}

impl std::ops::AddAssign<Pos> for Rect {
    fn add_assign(&mut self, rhs: Pos) {
        self.translate(rhs)
    }
}

impl std::ops::SubAssign<Pos> for Rect {
    fn sub_assign(&mut self, rhs: Pos) {
        self.min = Pos::new(self.min.x - rhs.x, self.min.y - rhs.y);
        self.max = Pos::new(self.max.x - rhs.x, self.max.y - rhs.y);
    }
}

impl std::ops::AddAssign<Size> for Rect {
    fn add_assign(&mut self, rhs: Size) {
        self.max = self.max.add_dims(rhs.width, rhs.height)
    }
}

impl std::ops::SubAssign<Size> for Rect {
    fn sub_assign(&mut self, rhs: Size) {
        self.max = self.max.sub_dims(rhs.width, rhs.height)
    }
}

impl std::ops::BitAnd<Rect> for Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: Rect) -> Self::Output {
        self.intersection(rhs)
    }
}

impl std::ops::BitAnd<Rect> for Option<Rect> {
    type Output = Option<Rect>;
    fn bitand(self, rhs: Rect) -> Self::Output {
        self?.intersection(rhs)
    }
}

impl std::ops::BitAnd<Option<Rect>> for Rect {
    type Output = Option<Rect>;
    fn bitand(self, rhs: Option<Rect>) -> Self::Output {
        self.intersection(rhs?)
    }
}
use super::pos::*;
use super::size::*;
use super::grid::*;
use super::margin::*;
use super::anchor::Anchor;
use super::placement::Placement;

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

    // Dimensions
    #[inline]
    pub const fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub const fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[inline]
    pub const fn left(&self) -> f32 {
        self.min.x
    }

    #[inline]
    pub const fn right(&self) -> f32 {
        self.max.x
    }

    #[inline]
    pub const fn top(&self) -> f32 {
        self.min.y
    }

    #[inline]
    pub const fn bottom(&self) -> f32 {
        self.max.y
    }

    #[inline]
    pub const fn left_top(&self) -> Pos {
        self.min
    }

    #[inline]
    pub const fn right_top(&self) -> Pos {
        Pos::new(self.max.x, self.min.y)
    }

    #[inline]
    pub const fn left_bottom(&self) -> Pos {
        Pos::new(self.min.x, self.max.y)
    }

    #[inline]
    pub const fn right_bottom(&self) -> Pos {
        self.max
    }

    #[inline]
    pub const fn left_center(&self) -> Pos {
        // We assume that self.max.y >= self.min.y
        let center_y = self.min.y + (self.max.y - self.min.y) * 0.5;
        Pos::new(self.min.x, center_y)
    }

    #[inline]
    pub const fn top_center(&self) -> Pos {
        let center_x = self.min.x + (self.max.x - self.max.y) * 0.5;
        Pos::new(center_x, self.min.y)
    }

    #[inline]
    pub const fn right_center(&self) -> Pos {
        let center_y = self.min.y + (self.max.y - self.min.y) * 0.5;
        Pos::new(self.max.x, center_y)
    }

    #[inline]
    pub const fn bottom_center(&self) -> Pos {
        let center_x = self.min.x + (self.max.x - self.min.x) * 0.5;
        Pos::new(center_x, self.max.y)
    }

    #[inline]
    pub const fn center(&self) -> Pos {
        let center_x = self.min.x + (self.max.x - self.min.x) * 0.5;
        let center_y = self.min.y + (self.max.y - self.min.y) * 0.5;
        Pos::new(center_x, center_y)
    }

    /// Takes a uv coordinate (0.0 to 1.0 for x and y) and returns the position at that UV coordinate on the [Rect].
    #[inline]
    pub const fn uv_pos(&self, uv: Pos) -> Pos {
        let x = self.min.x + (self.max.x - self.min.x) * uv.x;
        let y = self.min.y + (self.max.y - self.min.y) * uv.y;
        Pos::new(x, y)
    }

    #[inline]
    pub const fn contains(&self, pos: Pos) -> bool {
        self.min.x <= pos.x && self.min.y <= pos.y
        && self.max.x > pos.x && self.max.y > pos.y
    }

    #[inline]
    pub const fn intersects(&self, rect: &Rect) -> bool {
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

    /// Applies a [Margin] in-place, mutating the [Rect].
    #[inline]
    pub const fn apply_margin(&mut self, margin: Margin) {
        self.min.x += margin.left as f32;
        self.min.y += margin.top as f32;
        self.max.x -= margin.right as f32;
        self.max.y -= margin.bottom as f32;
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

    pub const fn anchor_rect(self, anchor: Anchor, size: f32, placement: Placement) -> Self {
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
            (Anchor::Center, Placement::Outside) => Rect {
                min: pos(self.min.x - size, self.min.y - size),
                max: pos(self.max.x + size, self.max.y + size),
            },
            (Anchor::Center, _) => Rect {
                min: pos(self.min.x + size, self.min.y + size),
                max: pos(self.max.x - size, self.max.y - size),
            },
        }
    }
}
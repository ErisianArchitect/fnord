use super::size::*;
use super::pos::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Grid {
    pub offset: Pos,
    pub cell_size: Size,
}

#[inline]
pub const fn grid(offset: Pos, cell_size: Size) -> Grid {
    Grid { offset, cell_size }
}

impl Grid {
    pub const UNIT: Self = Self::square(Pos::ZERO, 1.0);
    /// A unit grid (`width` and `height` are `1.0`) where the offset if half the size less than `(0.0, 0.0)`.
    pub const UNIT_CENTERED: Self = Self::centered_square(Pos::ZERO, 1.0);
    #[inline]
    pub const fn new(offset: Pos, cell_size: Size) -> Self {
        Self { offset, cell_size }
    }

    #[inline]
    pub const fn centered(center: Pos, cell_size: Size) -> Self {
        let half_size = cell_size.mul_dims(0.5, 0.5);
        Self::new(center.sub_dims(half_size.width, half_size.height), cell_size)
    }

    #[inline]
    pub const fn square(offset: Pos, size: f32) -> Self {
        Self::new(offset, Size::square(size))
    }

    #[inline]
    pub const fn centered_square(offset: Pos, size: f32) -> Self {
        let half = size * 0.5;
        Self::new(offset.sub_dims(half, half), Size::square(size))
    }

    #[inline]
    pub fn snap(self, pos: Pos) -> Pos {
        let offset_pos = pos.sub_dims(self.offset.x, self.offset.y);
        let offset_pos_rem = offset_pos.rem_euclid_dims(self.cell_size.width, self.cell_size.height);
        let snapped_offset_pos = offset_pos.sub_dims(offset_pos_rem.x, offset_pos_rem.y);
        snapped_offset_pos.add_dims(self.offset.x, self.offset.y)
    }

    #[inline]
    pub fn snap_left_top(self, pos: Pos) -> Pos {
        self.snap(pos)
    }

    #[inline]
    pub fn snap_right_top(self, pos: Pos) -> Pos {
        let offset_pos = Pos::new(pos.x + self.cell_size.width, pos.y);
        self.snap(offset_pos)
    }

    #[inline]
    pub fn snap_left_bottom(self, pos: Pos) -> Pos {
        let offset_pos = Pos::new(pos.x, pos.y + self.cell_size.height);
        self.snap(offset_pos)
    }

    #[inline]
    pub fn snap_right_bottom(self, pos: Pos) -> Pos {
        let offset_pos = Pos::new(pos.x + self.cell_size.width, pos.y + self.cell_size.height);
        self.snap(offset_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn snap_test() {
        let grid = Grid::new(Pos::NEG_HALF, Size::ONE);
        let snap_me = Pos::new(4.3, 2.1);
        let left_top = grid.snap_left_top(snap_me);
        let right_top = grid.snap_right_top(snap_me);
        let left_bottom = grid.snap_left_bottom(snap_me);
        let right_bottom = grid.snap_right_bottom(snap_me);
        println!("{left_top:?}");
        println!("{right_top:?}");
        println!("{left_bottom:?}");
        println!("{right_bottom:?}");
    }
}
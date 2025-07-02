
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Placement {
    Inside = 0,
    Middle = 1,
    Outside = 2,
}
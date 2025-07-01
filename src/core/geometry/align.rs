
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Align {
    #[default]
    Min = 0,
    Center = 1,
    Max = 2,
}
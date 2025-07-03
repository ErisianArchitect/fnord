
#[inline(always)]
pub const fn half(value: f32) -> f32 {
    value * 0.5
}

#[inline]
pub const fn quarter(value: f32) -> f32 {
    value * 0.25
}

#[inline]
pub const fn third(value: f32) -> f32 {
    value * 0.3333333333333333
}

#[inline]
pub const fn fifth(value: f32) -> f32 {
    value * 0.2
}

#[inline]
pub const fn tenth(value: f32) -> f32 {
    value * 0.1
}
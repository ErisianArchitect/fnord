pub use std::f32::consts::*;

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

#[inline]
pub const fn is_positive(value: f32) -> bool {
    value >= 0.0
}

#[inline]
pub const fn clamp_unit(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

#[must_use]
#[inline]
pub const fn normalize_angle(radians: f32) -> f32 {
    (radians % TAU + TAU) % TAU
}
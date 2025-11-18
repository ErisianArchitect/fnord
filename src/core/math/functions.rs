
#[must_use]
#[inline(always)]
pub const fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[must_use]
#[inline(always)]
pub const fn lerp_f64(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
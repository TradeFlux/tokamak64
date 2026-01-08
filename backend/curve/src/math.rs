//! Scaling helpers for mapping capacity units into the curve domain.
//!
//! The master LUT defines a fixed cumulative-cost span `[0, Smax]`. We treat
//! an external capacity `Cmax` as linearly mapping onto this span so that:
//! `ds = dc * Smax / Cmax`. This keeps the curve shape constant while allowing
//! different total capacities.
//!
//! All values use unsigned arithmetic:
//! - `x`: u32 in range [0, 12] (Q8.24)
//! - `s`: u64 cumulative cost (Q16.48)
//! - `dx`, `ds`, `dc`: unsigned deltas, with two's complement for negation

use crate::lut::{LUT_S_MAX, S_LUT, X_LUT};

/// Calculates `dx` from a capacity delta `dc` by scaling through `ds`,
/// given that the current curve state is `(x0, s0)` and total capacity is `cmax`.
///
/// Arguments:
/// - `x0`: Current x position in Q8.24 (u32).
/// - `s0`: Current cumulative cost at `x0` in Q16.48 (u64).
/// - `dc`: Delta capacity (u64, interpreted as two's complement for negative deltas).
/// - `cmax`: Total capacity that maps to `Smax` (u64).
///
/// Returns: `(dx, ds)` where dx is two's complement encoded (u32), ds is u64.
///
/// Constraints:
/// - `x0` is within `[LUT_X_MIN, LUT_X_MAX]`.
/// - `s0` equals the LUT evaluation at `x0`.
/// - `cmax > 0`.
/// - `dc * Smax` fits in `u128`.
pub fn dx_for_dc(x0: u32, s0: u64, dc: u64, cmax: u64) -> (u32, u64) {
    let ds = ds_for_dc(dc, cmax);
    let dx = dx_for_ds(x0, s0, ds);
    (dx, ds)
}

/// Calculates `dc` (delta capacity) for moving from `x0` to `x0 + dx`,
/// given that capacity maps linearly onto the LUT range.
///
/// Arguments:
/// - `x0`: Current x position in Q8.24 (u32).
/// - `dx`: Delta x in Q8.24 (u32, two's complement for negative).
/// - `cmax`: Total capacity that maps to `Smax` (u64).
///
/// Returns: `dc` as u64 (two's complement encoded for negative values).
///
/// Constraints:
/// - `x0 + dx` is within `[LUT_X_MIN, LUT_X_MAX]`.
/// - `cmax > 0`.
/// - `ds * Cmax` fits in `u128`.
pub fn dc_for_dx(x0: u32, dx: u32, cmax: u64) -> u64 {
    let ds = ds_for_dx(x0, dx);
    dc_for_ds(ds, cmax)
}

/// Calculates `ds` (delta cumulative cost) for moving from `x0` to `x0 + dx`,
/// given that the move stays within the LUT domain.
///
/// Arguments:
/// - `x0`: Current x position in Q8.24 (u32).
/// - `dx`: Delta x in Q8.24 (u32, two's complement for negative).
///
/// Returns: `ds` as u64 (two's complement encoded for negative values).
///
/// Constraints:
/// - `x0 + dx` is within `[LUT_X_MIN, LUT_X_MAX]`.
pub(crate) fn ds_for_dx(x0: u32, dx: u32) -> u64 {
    let x1 = x0.wrapping_add(dx);
    let s1 = evaluate_cost(x1);
    let s0 = evaluate_cost(x0);
    s1.wrapping_sub(s0)
}

/// Calculates `dx` given `x0`, its cumulative cost `s0`, and a desired `ds`,
/// given that the target cumulative cost remains within the LUT range.
///
/// Arguments:
/// - `x0`: Current x position in Q8.24 (u32).
/// - `s0`: Current cumulative cost at `x0` in Q16.48 (u64).
/// - `ds`: Desired delta in cumulative cost (u64, two's complement for negative).
///
/// Returns: `dx` as u32 (two's complement encoded for negative values).
///
/// Constraints:
/// - `x0` is within `[LUT_X_MIN, LUT_X_MAX]`.
/// - `s0` equals the LUT evaluation at `x0`.
/// - `s0 + ds` is within valid range (two's complement).
pub(crate) fn dx_for_ds(x0: u32, s0: u64, ds: u64) -> u32 {
    let s = s0.wrapping_add(ds);
    let x1 = x_for_s(s);
    x1.wrapping_sub(x0)
}

/// Calculates the cumulative cost at `x` (Q8.24),
/// given that `x` is within the LUT domain.
#[inline]
pub(crate) fn evaluate_cost(x: u32) -> u64 {
    match X_LUT.binary_search(&x) {
        Ok(i) => S_LUT[i],
        Err(i) => {
            // x is between X_LUT[i-1] and X_LUT[i]
            // (since x is in-domain, i is in 1..len)
            let x0 = X_LUT[i - 1];
            let x1 = X_LUT[i];
            let s0 = S_LUT[i - 1];
            let s1 = S_LUT[i];
            interp_s_for_x(x, x0, s0, x1, s1)
        }
    }
}

/// Calculates `ds` (delta cumulative cost) from a capacity delta `dc`,
/// given that the total capacity `cmax` maps to `Smax`.
///
/// Arguments:
/// - `dc`: Delta capacity (u64, two's complement for negative).
/// - `cmax`: Total capacity that maps to `Smax` (u64).
///
/// Returns: `ds` as u64 (two's complement encoded for negative).
///
/// Constraints:
/// - `cmax > 0`.
/// - `dc * Smax` fits in `u128`.
pub(crate) fn ds_for_dc(dc: u64, cmax: u64) -> u64 {
    let s_max = LUT_S_MAX as u128;
    let num = dc as u128 * s_max;
    div_round_u128(num, cmax as u128) as u64
}

/// Calculates `dc` (delta capacity) from a cumulative cost delta `ds`,
/// given that the total capacity `cmax` maps to `Smax`.
fn dc_for_ds(ds: u64, cmax: u64) -> u64 {
    let num = ds as u128 * cmax as u128;
    div_round_u128(num, LUT_S_MAX as u128) as u64
}

/// Integer (floor) lerp of s between (x0,s0) and (x1,s1) at x.
/// Preconditions (must be enforced by caller):
///   - x1 > x0
///   - x0 <= x <= x1
///   - s1 >= s0
#[inline]
fn interp_s_for_x(x: u32, x0: u32, s0: u64, x1: u32, s1: u64) -> u64 {
    let dx = (x1 - x0) as u128;
    let t = (x - x0) as u128;
    let ds = (s1 - s0) as u128;

    (s0 as u128 + (ds * t + dx / 2) / dx) as u64
}

#[inline]
fn x_for_s(s_target: u64) -> u32 {
    match S_LUT.binary_search(&s_target) {
        Ok(i) => X_LUT[i],
        Err(i) => {
            let x0 = X_LUT[i - 1];
            let x1 = X_LUT[i];
            let s0 = S_LUT[i - 1];
            let s1 = S_LUT[i];
            interp_x_for_s(s_target, x0, s0, x1, s1)
        }
    }
}

/// Integer lerp of x between (s0,x0) and (s1,x1) at s, rounded to nearest.
/// Preconditions (must be enforced by caller):
///   - s1 > s0
///   - s0 <= s <= s1
///   - x1 >= x0
#[inline]
fn interp_x_for_s(s: u64, x0: u32, s0: u64, x1: u32, s1: u64) -> u32 {
    let ds = (s1 - s0) as u128;
    let t = (s - s0) as u128;
    let dx = (x1 - x0) as u128;

    (x0 as u128 + (t * dx + ds / 2) / ds) as u32
}

/// Round-to-nearest division for unsigned 128-bit integers (ties away from zero).
fn div_round_u128(num: u128, den: u128) -> u128 {
    (num + (den / 2)) / den
}

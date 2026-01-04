//! Scaling helpers for mapping capacity units into the curve domain.
//!
//! The master LUT defines a fixed cumulative-cost span `[0, Smax]`. We treat
//! an external capacity `Cmax` as linearly mapping onto this span so that:
//! `ds = dc * Smax / Cmax`. This keeps the curve shape constant while allowing
//! different total capacities.

use crate::lut::{LUT_S_MAX, S_LUT, X_LUT};

/// Calculates `dx` from a capacity delta `dc` by scaling through `ds`,
/// given that the current curve state is `(x0, s0)` and total capacity is `cmax`.
///
/// Arguments:
/// - `x0`: Current x position in Q16.16.
/// - `s0`: Current cumulative cost at `x0` in Q16.48.
/// - `dc`: Delta capacity in external units.
/// - `cmax`: Total capacity that maps to `Smax`.
///
/// Constraints:
/// - `x0` is within `[LUT_X_MIN, LUT_X_MAX]`.
/// - `s0` equals the LUT evaluation at `x0`.
/// - `cmax > 0`.
/// - `dc * Smax` fits in `i128`.
pub fn dx_for_dc(x0: i32, s0: u64, dc: i64, cmax: u64) -> i32 {
    let ds = ds_for_dc(dc, cmax);
    dx_for_ds(x0, s0, ds)
}

/// Calculates `dc` (delta capacity) for moving from `x0` to `x0 + dx`,
/// given that capacity maps linearly onto the LUT range.
///
/// Arguments:
/// - `x0`: Current x position in Q16.16.
/// - `dx`: Delta x in Q16.16.
/// - `cmax`: Total capacity that maps to `Smax`.
///
/// Constraints:
/// - `x0 + dx` is within `[LUT_X_MIN, LUT_X_MAX]`.
/// - `cmax > 0`.
/// - `ds * Cmax` fits in `i128`.
pub fn dc_for_dx(x0: i32, dx: i32, cmax: u64) -> i64 {
    let ds = ds_for_dx(x0, dx);
    dc_for_ds(ds, cmax)
}

/// Calculates `ds` (delta cumulative cost) for moving from `x0` to `x0 + dx`,
/// given that the move stays within the LUT domain.
///
/// Arguments:
/// - `x0`: Current x position in Q16.16.
/// - `dx`: Delta x in Q16.16.
///
/// Constraints:
/// - `x0 + dx` is within `[LUT_X_MIN, LUT_X_MAX]`.
pub(crate) fn ds_for_dx(x0: i32, dx: i32) -> i64 {
    let x1 = x0 + dx;
    let s1 = evaluate_cost(x1) as i128;
    let s0 = evaluate_cost(x0) as i128;
    (s1 - s0) as i64
}

/// Calculates `dx` given `x0`, its cumulative cost `s0`, and a desired `ds`,
/// given that the target cumulative cost remains within the LUT range.
///
/// Arguments:
/// - `x0`: Current x position in Q16.16.
/// - `s0`: Current cumulative cost at `x0` in Q16.48.
/// - `ds`: Desired delta in cumulative cost (Q16.48).
///
/// Constraints:
/// - `x0` is within `[LUT_X_MIN, LUT_X_MAX]`.
/// - `s0` equals the LUT evaluation at `x0`.
/// - `s0 + ds` is within `[0, LUT_S_MAX]`.
pub(crate) fn dx_for_ds(x0: i32, s0: u64, ds: i64) -> i32 {
    let s_target = s0 as i128 + ds as i128;
    let x1 = x_for_s(s_target as u64);
    x1 - x0
}

/// Calculates the cumulative cost at `x` (Q16.16),
/// given that `x` is within the LUT domain.
#[inline]
pub(crate) fn evaluate_cost(x: i32) -> u64 {
    match X_LUT.binary_search_by(|value| value.cmp(&x)) {
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
/// - `dc`: Delta capacity in external units.
/// - `cmax`: Total capacity that maps to `Smax`.
///
/// Constraints:
/// - `cmax > 0`.
/// - `dc * Smax` fits in `i128`.
pub(crate) fn ds_for_dc(dc: i64, cmax: u64) -> i64 {
    let s_max = LUT_S_MAX as i128;
    let cmax_i128 = cmax as i128;
    let num = (dc as i128) * s_max;
    div_round_i128(num, cmax_i128) as i64
}

/// Calculates `dc` (delta capacity) from a cumulative cost delta `ds`,
/// given that the total capacity `cmax` maps to `Smax`.
fn dc_for_ds(ds: i64, cmax: u64) -> i64 {
    let cmax_i128 = cmax as i128;
    let num = (ds as i128) * (cmax_i128);
    div_round_i128(num, LUT_S_MAX as i128) as i64
}

/// Calculates an interpolated `s` between two LUT samples at `x`.
#[inline]
fn interp_s_for_x(x: i32, x0: i32, s0: u64, x1: i32, s1: u64) -> u64 {
    let dx = (x1 as i64 - x0 as i64) as i128;
    let t = (x as i64 - x0 as i64) as i128;

    let s0i = s0 as i128;
    let s1i = s1 as i128;

    let out = s0i + ((s1i - s0i) * t) / dx;
    out as u64
}

#[inline]
fn x_for_s(s_target: u64) -> i32 {
    match S_LUT.binary_search_by(|value| value.cmp(&s_target)) {
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

/// Calculates an interpolated `x` between two LUT samples at `s`.
#[inline]
fn interp_x_for_s(s: u64, x0: i32, s0: u64, x1: i32, s1: u64) -> i32 {
    let ds = (s1 - s0) as i128;
    let t = (s - s0) as i128;

    let x0i = x0 as i128;
    let x1i = x1 as i128;

    let x = x0i + (t * (x1i - x0i) + ds / 2) / ds;
    x as i32
}

/// Round-to-nearest division for signed integers (ties to away from zero).
fn div_round_i128(num: i128, den: i128) -> i128 {
    if num >= 0 {
        (num + den / 2) / den
    } else {
        (num - den / 2) / den
    }
}

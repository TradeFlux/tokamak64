//! Tests for LUT delta mapping and inversion properties.

use super::lut::*;
use super::math::*;

// 0.25 in Q16.16
const MARGIN: i32 = (1i32 << 16) / 4;

/// Deterministic PRNG (no external crates).
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next_u32(&mut self) -> u32 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        ((x.wrapping_mul(0x2545F4914F6CDD1D)) >> 32) as u32
    }
    fn gen_i32(&mut self, min: i32, max: i32) -> i32 {
        debug_assert!(min <= max);
        let span = (max as i64 - min as i64 + 1) as u64;
        let v = (self.next_u32() as u64) % span;
        (min as i64 + v as i64) as i32
    }
}

fn gen_in_domain_x(rng: &mut Rng) -> i32 {
    rng.gen_i32(LUT_X_MIN, LUT_X_MAX)
}

fn gen_dx_in_domain(rng: &mut Rng, x0: i32) -> i32 {
    rng.gen_i32(LUT_X_MIN - x0, LUT_X_MAX - x0)
}

fn step_bounds_in_domain(x: i32, max_step: i32) -> Option<(i32, i32)> {
    let min = (LUT_X_MIN - x).max(-max_step);
    let max = (LUT_X_MAX - x).min(max_step);
    if min > max {
        None
    } else {
        Some((min, max))
    }
}

/// Verifies the zero step is a fixed point for delta mapping.
/// We pick a midpoint x in-domain and ensure no-op movement returns zero delta.
#[test]
fn delta_zero_is_zero() {
    let x0 = (LUT_X_MIN + LUT_X_MAX) / 2;
    assert_eq!(ds_for_dx(x0, 0), 0);
}

/// Ensures delta mapping matches the explicit cumulative cost difference.
/// This ties `ds_for_dx` to `evaluate_cost` across random in-domain steps.
#[test]
fn delta_matches_eval_difference_in_domain() {
    let mut rng = Rng::new(1);
    for _ in 0..50_000 {
        let x0 = gen_in_domain_x(&mut rng);
        let dx = gen_dx_in_domain(&mut rng, x0);
        let x1 = x0 + dx;

        let lhs = ds_for_dx(x0, dx);
        let rhs = evaluate_cost(x1) as i64 - evaluate_cost(x0) as i64;
        assert_eq!(lhs, rhs);
    }
}

/// Confirms additivity across two steps when all points stay in-domain.
/// The delta for a+b should equal the sum of deltas for a then b.
#[test]
fn path_independence_two_steps_in_domain() {
    let mut rng = Rng::new(2);

    for _ in 0..200_000 {
        let x0 = rng.gen_i32(LUT_X_MIN + MARGIN, LUT_X_MAX - MARGIN);
        let dx1 = rng.gen_i32(-MARGIN + 1, MARGIN - 1);
        let x_mid = x0 + dx1;

        let dx2_min = LUT_X_MIN - x_mid + 1;
        let dx2_max = LUT_X_MAX - x_mid - 1;
        if dx2_min > dx2_max {
            continue;
        }
        let dx2 = rng.gen_i32(dx2_min, dx2_max);

        let direct = ds_for_dx(x0, dx1 + dx2);
        let step = ds_for_dx(x0, dx1) + ds_for_dx(x_mid, dx2);
        assert_eq!(direct, step);
    }
}

/// Confirms additivity across many steps with bounded movement.
/// This exercises longer paths to catch accumulated rounding errors.
#[test]
fn path_independence_many_steps_in_domain() {
    let mut rng = Rng::new(3);

    for _ in 0..50_000 {
        let mut x = rng.gen_i32(LUT_X_MIN + MARGIN, LUT_X_MAX - MARGIN);
        let x_start = x;

        let mut sum: i64 = 0;
        let steps = 8;
        for _ in 0..steps {
            let (step_min, step_max) = match step_bounds_in_domain(x, MARGIN / 2 - 1) {
                Some(bounds) => bounds,
                None => continue,
            };
            let dx = rng.gen_i32(step_min, step_max);
            sum += ds_for_dx(x, dx);
            x += dx;
        }

        let direct = ds_for_dx(x_start, x - x_start);
        assert_eq!(direct, sum);
    }
}

/// Checks antisymmetry of the delta mapping under inversion of direction.
/// Moving forward then backward should negate the delta when in-bounds.
#[test]
fn antisymmetry_in_domain() {
    let mut rng = Rng::new(4);

    for _ in 0..200_000 {
        let x = rng.gen_i32(LUT_X_MIN + MARGIN, LUT_X_MAX - MARGIN);
        let dx = rng.gen_i32(-MARGIN, MARGIN);
        let x2 = x + dx;
        if x2 <= LUT_X_MIN || x2 >= LUT_X_MAX {
            continue;
        }

        let a = ds_for_dx(x, dx);
        let b = ds_for_dx(x2, -dx);
        assert_eq!(a, -b);
    }
}

/// Ensures exact inversion when targets land on LUT samples.
/// When both endpoints are exact LUT points, inversion should be exact.
#[test]
fn invert_delta_at_lut_points() {
    let indices = [
        0usize,
        X_LUT.len() / 4,
        X_LUT.len() / 2,
        (3 * X_LUT.len()) / 4,
        X_LUT.len() - 1,
    ];

    for &i in &indices {
        let x0 = X_LUT[i];
        let s0 = S_LUT[i];

        for &j in &indices {
            let x1 = X_LUT[j];
            let ds = S_LUT[j] as i64 - s0 as i64;
            let dx = dx_for_ds(x0, s0, ds);
            assert_eq!(x0 + dx, x1);
        }
    }
}

/// Ensures round-trip inversion stays within one LSB of x.
/// The inverse mapping is linear between samples, so 1 LSB tolerance is expected.
#[test]
fn invert_delta_round_trip_near_exact() {
    let mut rng = Rng::new(5);
    for _ in 0..50_000 {
        let x0 = gen_in_domain_x(&mut rng);
        let dx = gen_dx_in_domain(&mut rng, x0);
        let x1 = x0 + dx;
        let s0 = evaluate_cost(x0);
        let ds = evaluate_cost(x1) as i64 - s0 as i64;

        let dx_inv = dx_for_ds(x0, s0, ds);
        let x1_inv = x0 + dx_inv;

        let diff = (x1_inv - x1).abs();
        assert!(diff <= 1, "x1={}, x1_inv={}", x1, x1_inv);
    }
}

/// Confirms mid-s interpolation maps near the x midpoint.
/// This checks the interpolation logic used by the inverse mapping.
#[test]
fn invert_delta_midpoint_between_samples() {
    let step = X_LUT.len() / 64;
    for i in (0..X_LUT.len() - 1).step_by(step) {
        let x0 = X_LUT[i];
        let x1 = X_LUT[i + 1];
        let s0 = S_LUT[i];
        let s1 = S_LUT[i + 1];
        let s_mid = (s0 + s1) / 2;
        let x_mid_expected = (x0 + x1) / 2;

        let dx = dx_for_ds(x0, s0, s_mid as i64 - s0 as i64);
        let x_mid = x0 + dx;

        let diff = (x_mid - x_mid_expected).abs();
        assert!(diff <= 1, "x_mid={}, expected={}", x_mid, x_mid_expected);
    }
}

/// Confirms capacity scaling maps the full `Cmax` span to the LUT's `S_max`.
#[test]
fn capacity_scale_maps_full_range() {
    let cmax = 1_000_000u64;
    let ds = ds_for_dc(cmax as i64, cmax);
    assert_eq!(ds, LUT_S_MAX as i64);
}

/// Ensures the composed mapping (dc -> ds -> dx) matches direct delta usage.
#[test]
fn dx_for_dc_matches_delta_mapping() {
    let x0 = (LUT_X_MIN + LUT_X_MAX) / 2;
    let s0 = evaluate_cost(x0);
    let cmax = 1_000_000u64;
    let dc = 12_345i64;

    let ds = ds_for_dc(dc, cmax);
    let dx_expected = dx_for_ds(x0, s0, ds);
    let dx = dx_for_dc(x0, s0, dc, cmax);

    assert_eq!(dx, dx_expected);
}

/// Ensures the capacity inversion matches the scaled ds mapping.
#[test]
fn dc_for_dx_matches_scaled_ds() {
    let x0 = (LUT_X_MIN + LUT_X_MAX) / 2;
    let dx = (LUT_X_MAX - LUT_X_MIN) / 8;
    let cmax = 1_000_000u64;

    let ds = ds_for_dx(x0, dx);
    let num = ds as i128 * cmax as i128;
    let den = LUT_S_MAX as i128;
    let dc_expected = if num >= 0 {
        (num + den / 2) / den
    } else {
        (num - den / 2) / den
    };
    let dc = dc_for_dx(x0, dx, cmax);

    assert_eq!(dc, dc_expected as i64);
}

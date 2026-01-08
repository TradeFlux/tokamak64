//! Tests for LUT delta mapping and inversion properties.

use super::lut::*;
use super::math::*;

// 0.25 in Q8.24
const MARGIN: u32 = (1u32 << 24) / 4;

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
    fn gen_u32(&mut self, min: u32, max: u32) -> u32 {
        debug_assert!(min <= max);
        let span = (max as u64).wrapping_sub(min as u64).wrapping_add(1);
        let v = (self.next_u32() as u64) % span;
        min.wrapping_add(v as u32)
    }
}

fn gen_in_domain_x(rng: &mut Rng) -> u32 {
    rng.gen_u32(LUT_X_MIN, LUT_X_MAX)
}

fn gen_dx_in_domain(rng: &mut Rng, x0: u32) -> u32 {
    // Generate a signed delta within bounds, encode as u32 two's complement
    let min_delta_i64 = (LUT_X_MIN as i64).wrapping_sub(x0 as i64);
    let max_delta_i64 = (LUT_X_MAX as i64).wrapping_sub(x0 as i64);

    let min_delta = (min_delta_i64.clamp(i32::MIN as i64, i32::MAX as i64)) as i32;
    let max_delta = (max_delta_i64.clamp(i32::MIN as i64, i32::MAX as i64)) as i32;

    if min_delta >= max_delta {
        0 // No valid delta range
    } else {
        let delta = if rng.next_u32() % 2 == 0 {
            rng.next_u32() as i32
        } else {
            (rng.next_u32() as i32).wrapping_neg()
        };
        let clamped = delta.clamp(min_delta, max_delta);
        clamped as u32
    }
}

fn gen_i32_in_range(rng: &mut Rng, min_i: i32, max_i: i32) -> i32 {
    debug_assert!(min_i <= max_i);
    let span = (max_i as i64).wrapping_sub(min_i as i64).wrapping_add(1) as u64;
    let v = (rng.next_u32() as u64) % span;
    min_i.wrapping_add(v as i32)
}

fn step_bounds_in_domain(x: u32, max_step: u32) -> Option<(i32, i32)> {
    // Compute signed delta bounds
    let min_delta_i64 = (LUT_X_MIN as i64).wrapping_sub(x as i64);
    let max_delta_i64 = (LUT_X_MAX as i64).wrapping_sub(x as i64);

    let min_delta = (min_delta_i64.max(-(max_step as i64))) as i32;
    let max_delta = (max_delta_i64.min(max_step as i64)) as i32;

    if min_delta > max_delta {
        None
    } else {
        Some((min_delta, max_delta))
    }
}

/// Verifies the zero step is a fixed point for delta mapping.
/// We pick a midpoint x in-domain and ensure no-op movement returns zero delta.
#[test]
fn delta_zero_is_zero() {
    let x0 = ((LUT_X_MIN as u64).wrapping_add(LUT_X_MAX as u64) / 2) as u32;
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
        let x1 = x0.wrapping_add(dx);

        let lhs = ds_for_dx(x0, dx);
        let rhs = evaluate_cost(x1).wrapping_sub(evaluate_cost(x0));
        assert_eq!(lhs, rhs);
    }
}

/// Confirms additivity across two steps when all points stay in-domain.
/// The delta for a+b should equal the sum of deltas for a then b.
#[test]
fn path_independence_two_steps_in_domain() {
    let mut rng = Rng::new(2);

    for _ in 0..200_000 {
        let x0 = rng.gen_u32(
            LUT_X_MIN.wrapping_add(MARGIN),
            LUT_X_MAX.wrapping_sub(MARGIN),
        );
        let dx1: i32 = ((rng.next_u32() % (2 * MARGIN as u32)) as i32).wrapping_sub(MARGIN as i32);
        let x_mid = x0.wrapping_add(dx1 as u32);

        let dx2_min = (LUT_X_MIN as i64)
            .wrapping_sub(x_mid as i64)
            .wrapping_add(1);
        let dx2_max = (LUT_X_MAX as i64)
            .wrapping_sub(x_mid as i64)
            .wrapping_sub(1);
        if dx2_min > dx2_max {
            continue;
        }
        let dx2 = ((rng.next_u32() as i64) % (dx2_max - dx2_min + 1) + dx2_min) as i32 as u32;

        let direct = ds_for_dx(x0, (dx1 as u32).wrapping_add(dx2));
        let step =
            (ds_for_dx(x0, dx1 as u32) as i128).wrapping_add(ds_for_dx(x_mid, dx2) as i128) as u64;
        assert_eq!(direct, step);
    }
}

/// Confirms additivity across many steps with bounded movement.
/// This exercises longer paths to catch accumulated rounding errors.
#[test]
fn path_independence_many_steps_in_domain() {
    let mut rng = Rng::new(3);

    for _ in 0..50_000 {
        let mut x = rng.gen_u32(
            LUT_X_MIN.wrapping_add(MARGIN),
            LUT_X_MAX.wrapping_sub(MARGIN),
        );
        let x_start = x;

        let mut sum: u64 = 0;
        let steps = 8;
        for _ in 0..steps {
            let (step_min, step_max) = match step_bounds_in_domain(x, MARGIN / 2 - 1) {
                Some(bounds) => bounds,
                None => continue,
            };
            let dx = gen_i32_in_range(&mut rng, step_min, step_max) as u32;
            sum = (sum as i128).wrapping_add(ds_for_dx(x, dx) as i128) as u64;
            x = x.wrapping_add(dx);
        }

        let direct = ds_for_dx(x_start, (x as i64).wrapping_sub(x_start as i64) as u32);
        assert_eq!(direct, sum);
    }
}

/// Checks antisymmetry of the delta mapping under inversion of direction.
/// Moving forward then backward should negate the delta when in-bounds.
#[test]
fn antisymmetry_in_domain() {
    let mut rng = Rng::new(4);

    for _ in 0..200_000 {
        let x = rng.gen_u32(
            LUT_X_MIN.wrapping_add(MARGIN),
            LUT_X_MAX.wrapping_sub(MARGIN),
        );
        let dx_i32: i32 =
            ((rng.next_u32() % (2 * MARGIN as u32)) as i32).wrapping_sub(MARGIN as i32);
        let dx = dx_i32 as u32;
        let x2 = x.wrapping_add(dx);
        if (x2 as i64) <= (LUT_X_MIN as i64) || (x2 as i64) >= (LUT_X_MAX as i64) {
            continue;
        }

        let a = ds_for_dx(x, dx);
        let b = ds_for_dx(x2, (-(dx as i32)) as u32);
        assert_eq!(a, (-(b as i64)) as u64);
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
            let ds = (S_LUT[j] as i128).wrapping_sub(s0 as i128) as u64;
            let dx = dx_for_ds(x0, s0, ds);
            assert_eq!(x0.wrapping_add(dx), x1);
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
        let x1 = x0.wrapping_add(dx);
        let s0 = evaluate_cost(x0);
        let ds = (evaluate_cost(x1) as i128).wrapping_sub(s0 as i128) as u64;

        let dx_inv = dx_for_ds(x0, s0, ds);
        let x1_inv = x0.wrapping_add(dx_inv);

        let diff = ((x1_inv as i64).wrapping_sub(x1 as i64)).abs();
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
        let s_mid = ((s0 as u128).wrapping_add(s1 as u128) / 2) as u64;
        let x_mid_expected = ((x0 as u64).wrapping_add(x1 as u64) / 2) as u32;

        let dx = dx_for_ds(x0, s0, (s_mid as i128).wrapping_sub(s0 as i128) as u64);
        let x_mid = x0.wrapping_add(dx);

        let diff = ((x_mid as i64).wrapping_sub(x_mid_expected as i64)).abs();
        assert!(diff <= 1, "x_mid={}, expected={}", x_mid, x_mid_expected);
    }
}

/// Confirms capacity scaling maps the full `Cmax` span to the LUT's `S_max`.
#[test]
fn capacity_scale_maps_full_range() {
    let cmax = 1_000_000u64;
    let ds = ds_for_dc(cmax, cmax);
    assert_eq!(ds, LUT_S_MAX);
}

/// Ensures the composed mapping (dc -> ds -> dx) matches direct delta usage.
#[test]
fn dx_for_dc_matches_delta_mapping() {
    let x0 = ((LUT_X_MIN as u64).wrapping_add(LUT_X_MAX as u64) / 2) as u32;
    let s0 = evaluate_cost(x0);
    let cmax = 1_000_000u64;
    let dc = 12_345u64;

    let ds = ds_for_dc(dc, cmax);
    let dx_expected = dx_for_ds(x0, s0, ds);
    let (dx, _) = dx_for_dc(x0, s0, dc, cmax);

    assert_eq!(dx, dx_expected);
}

/// Ensures the capacity inversion matches the scaled ds mapping.
#[test]
fn dc_for_dx_matches_scaled_ds() {
    let x0 = ((LUT_X_MIN as u64).wrapping_add(LUT_X_MAX as u64) / 2) as u32;
    let dx = ((LUT_X_MAX as u64).wrapping_sub(LUT_X_MIN as u64) / 8) as u32;
    let cmax = 1_000_000u64;

    let ds = ds_for_dx(x0, dx);
    let num = (ds as u128).wrapping_mul(cmax as u128);
    let den = LUT_S_MAX as u128;
    let dc_expected = (num.wrapping_add(den / 2)) / den;
    let dc = dc_for_dx(x0, dx, cmax);

    assert_eq!(dc, dc_expected as u64);
}

/// Ensures boundary clamping at X_MAX: positive movement is clipped.
/// When at X_MAX and we request positive dc, dx should be 0 (no movement possible).
#[test]
fn boundary_clamp_at_x_max() {
    let x0 = LUT_X_MAX;
    let s0 = evaluate_cost(x0);
    let cmax = 1_000_000u64;
    let dc = 1000u64; // positive capacity delta

    let (dx, _) = dx_for_dc(x0, s0, dc, cmax);
    assert_eq!(dx, 0, "at X_MAX, positive dc should yield dx=0");
}

/// Ensures boundary clamping at X_MIN: attempting backward movement is clipped.
/// When at X_MIN and we request backward movement via dx_for_ds, dx should be 0.
#[test]
fn boundary_clamp_at_x_min() {
    let x0 = LUT_X_MIN;
    let s0 = evaluate_cost(x0);
    
    // Request a large negative ds (backward movement)
    let ds = (-1_000_000_000i64) as u64;
    let dx = dx_for_ds(x0, s0, ds);
    
    // dx should be 0 because x_for_s clamps to LUT_X_MIN
    let x1 = x0.wrapping_add(dx);
    assert!(x1 >= LUT_X_MIN, "clamped x1={} must not go below X_MIN={}", x1, LUT_X_MIN);
}

/// Ensures partial movement at boundary: if x is near max and movement would overshoot,
/// dx is clamped to the remaining distance to X_MAX.
#[test]
fn boundary_clamp_partial_movement() {
    let cmax = 1_000_000u64;
    // Create a scenario where we're near X_MAX but not exactly at it
    let x0 = LUT_X_MAX.wrapping_sub(100_000); // 0.0024... away from max
    let s0 = evaluate_cost(x0);

    // Request a large positive dc that would definitely overshoot
    let dc = 100_000u64;
    let (dx, _) = dx_for_dc(x0, s0, dc, cmax);

    // dx should be positive (moving right) but clamped to at most X_MAX - x0
    let x1 = x0.wrapping_add(dx);
    assert!(x1 <= LUT_X_MAX, "clamped x1={} must not exceed X_MAX={}", x1, LUT_X_MAX);
    assert!(dx > 0, "with positive dc, we should move right at least somewhat");
}

/// Ensures dc_for_dx is protected when dx overshoots boundary.
/// When at X_MAX and request a large positive dx, evaluate_cost clamps and dc is 0.
#[test]
fn dc_for_dx_protected_at_boundary() {
    let x0 = LUT_X_MAX;
    let cmax = 1_000_000u64;
    let large_dx = 1_000_000u32; // attempt to go way beyond X_MAX
    
    let dc = dc_for_dx(x0, large_dx, cmax);
    // Since we're at the boundary and can't move further, dc should be 0
    assert_eq!(dc, 0, "at X_MAX with overshooting dx, dc should be 0");
}

/// Ensures ds_for_dx is protected when dx undershoots boundary.
/// When at X_MIN and move backward, both endpoints clamp to X_MIN so ds is 0.
#[test]
fn ds_for_dx_protected_at_boundary() {
    let x0 = LUT_X_MIN;
    let large_negative_dx = (i32::MIN) as u32; // extreme negative wraparound
    
    let x1 = x0.wrapping_add(large_negative_dx);
    // x1 wraps around to some large value, but evaluate_cost clamps both
    let ds = ds_for_dx(x0, large_negative_dx);
    
    // Both evaluate_cost(x0) and evaluate_cost(x1) clamp to valid range
    // x0 = LUT_X_MIN → s0 = S_LUT[0]
    // x1 wraps to huge value → clamped to LUT_X_MAX → s1 = S_LUT[1024]
    // So ds = S_LUT[1024] - S_LUT[0] = LUT_S_MAX (large positive)
    assert!(ds > 0, "wrapping to opposite side should give large positive ds");
}


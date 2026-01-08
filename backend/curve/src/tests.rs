//! Tests for LUT delta mapping and inversion properties.

use super::lut::*;
use super::math::*;

const MARGIN: u32 = (1u32 << 24) / 4; // 0.25 in Q8.24

/// Deterministic PRNG (xorshift64*).
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        ((x.wrapping_mul(0x2545F4914F6CDD1D)) >> 32) as u32
    }

    /// Generate u32 uniformly in [min, max].
    fn gen_u32(&mut self, min: u32, max: u32) -> u32 {
        let span = (max as u64) - (min as u64) + 1;
        let v = (self.next_u32() as u64) % span;
        min + (v as u32)
    }
}

// ===== Unsigned Range Checking =====
//
// Using two's complement semantics with wrapping arithmetic, we can check
// membership in [min, max] for u32 without casting to signed types.
// The trick: x in [min, max] iff (x - min) <= (max - min) using wrapping subtraction.

/// Check if u32 value is in range [min, max] using wrapping arithmetic.
fn in_range(x: u32, min: u32, max: u32) -> bool {
    x.wrapping_sub(min) <= max.wrapping_sub(min)
}

// ===== Test Helpers =====

fn midpoint() -> u32 {
    ((LUT_X_MIN as u64 + LUT_X_MAX as u64) / 2) as u32
}

fn gen_x_in_domain(rng: &mut Rng) -> u32 {
    rng.gen_u32(LUT_X_MIN, LUT_X_MAX)
}

fn gen_x_with_margin(rng: &mut Rng) -> u32 {
    rng.gen_u32(LUT_X_MIN + MARGIN, LUT_X_MAX - MARGIN)
}

/// Generate a bounded dx (as u32 two's complement) such that x0 + dx stays in [LUT_X_MIN, LUT_X_MAX].
/// Uses randomized search with retry limit.
fn gen_dx_bounded(rng: &mut Rng, x0: u32) -> u32 {
    for _ in 0..100 {
        let dx = rng.next_u32();
        let x1 = x0.wrapping_add(dx);
        if in_range(x1, LUT_X_MIN, LUT_X_MAX) {
            return dx;
        }
    }
    0 // fallback: no movement
}

/// Generate a bounded dx with magnitude limit (for path tests).
/// Returns zero if no valid delta exists.
fn gen_dx_bounded_magnitude(rng: &mut Rng, x0: u32, mag_limit: u32) -> u32 {
    for _ in 0..100 {
        let dx = rng.next_u32();
        let x1 = x0.wrapping_add(dx);

        // Check both domain bounds and magnitude bounds using wrapping arithmetic
        if in_range(x1, LUT_X_MIN, LUT_X_MAX) {
            // Check magnitude: both forward and backward movements must be within limit
            let forward_mag = x1.wrapping_sub(x0); // positive delta
            let backward_mag = x0.wrapping_sub(x1); // negative delta (as u32)

            if forward_mag <= mag_limit || backward_mag <= mag_limit {
                return dx;
            }
        }
    }
    0
}

// ===== Delta Mapping Tests =====
//
// These tests verify that `ds_for_dx` correctly computes cumulative cost deltas
// for movement deltas in the x domain. The core property is that ds(x, dx) should
// equal the cost difference: evaluate_cost(x + dx) - evaluate_cost(x).

/// Zero movement yields zero cost delta (fixed point of the mapping).
#[test]
fn zero_delta_is_identity() {
    assert_eq!(ds_for_dx(midpoint(), 0), 0);
}

/// `ds_for_dx` matches direct cost function evaluation across 50k random in-domain steps.
/// This ties the delta function to the LUT evaluations; any discrepancy would indicate
/// a bug in either the lookup or interpolation logic.
#[test]
fn delta_matches_cost_difference() {
    let mut rng = Rng::new(1);
    for _ in 0..50_000 {
        let x0 = gen_x_in_domain(&mut rng);
        let dx = gen_dx_bounded(&mut rng, x0);
        let x1 = x0.wrapping_add(dx);

        let ds = ds_for_dx(x0, dx);
        let cost_diff = evaluate_cost(x1).wrapping_sub(evaluate_cost(x0));
        assert_eq!(ds, cost_diff);
    }
}

/// Path independence: taking two steps (dx1 then dx2) has the same cost as one step (dx1+dx2).
/// Verifies additivity: ds(x, dx1 + dx2) == ds(x, dx1) + ds(x + dx1, dx2).
/// This is critical for composability; game logic often chains multiple moves.
/// Run across 200k iterations to catch accumulated rounding errors.
#[test]
fn path_independence_two_steps() {
    let mut rng = Rng::new(2);
    for _ in 0..200_000 {
        let x0 = gen_x_with_margin(&mut rng);
        let dx1 = gen_dx_bounded_magnitude(&mut rng, x0, MARGIN);
        // Skip invalid paths (no valid delta found within margin)
        if dx1 == 0 {
            continue;
        }
        let x_mid = x0.wrapping_add(dx1);
        let dx2 = gen_dx_bounded(&mut rng, x_mid);
        // Skip invalid paths
        if dx2 == 0 {
            continue;
        }

        let direct = ds_for_dx(x0, dx1.wrapping_add(dx2));
        let step = ds_for_dx(x0, dx1).wrapping_add(ds_for_dx(x_mid, dx2));
        assert_eq!(direct, step);
    }
}

/// Path independence with many steps: the sum of deltas across 8 sequential steps
/// equals the delta of the total movement. Tests longer paths to catch accumulated
/// rounding errors. Run 50k times with different paths.
#[test]
fn path_independence_many_steps() {
    let mut rng = Rng::new(3);
    for _ in 0..50_000 {
        let mut x = gen_x_with_margin(&mut rng);
        let x_start = x;
        let mut sum: u64 = 0;

        for _ in 0..8 {
            let dx = gen_dx_bounded_magnitude(&mut rng, x, MARGIN / 2);
            if dx == 0 {
                continue;
            }
            sum = sum.wrapping_add(ds_for_dx(x, dx));
            x = x.wrapping_add(dx);
        }

        let direct = ds_for_dx(x_start, x.wrapping_sub(x_start));
        assert_eq!(direct, sum);
    }
}

/// Antisymmetry: moving forward by dx and then backward by -dx cancels out.
/// Specifically: ds(x, dx) == -ds(x + dx, -dx) (under two's complement).
/// This is essential for the curve to be invertible and for movement reversals.
/// Test 200k random paths with both forward and backward steps.
#[test]
fn antisymmetry() {
    let mut rng = Rng::new(4);
    for _ in 0..200_000 {
        let x = gen_x_with_margin(&mut rng);
        let dx = gen_dx_bounded_magnitude(&mut rng, x, MARGIN);
        if dx == 0 {
            continue;
        }
        let x2 = x.wrapping_add(dx);

        let a = ds_for_dx(x, dx);
        let b = ds_for_dx(x2, (0u32).wrapping_sub(dx)); // two's complement negation: -dx
        assert_eq!(a, (0u64).wrapping_sub(b));
    }
}

// ===== Inversion Tests =====
//
// These tests verify the inverse mapping: given a cumulative cost delta `ds`,
// compute the x movement `dx` that produces it. This is used by `dx_for_dc`.
// Key invariant: x_for_s(s_for_x(x)) ~= x (within 1 LSB due to interpolation).

/// Exact inversion at LUT sample points: testing all pairwise combinations
/// of the 5 key indices (0, 1/4, 1/2, 3/4, end). When both endpoints are exact
/// LUT samples, inversion must be exact with zero error.
#[test]
fn invert_exact_at_lut_points() {
    for &i in &[
        0,
        X_LUT.len() / 4,
        X_LUT.len() / 2,
        3 * X_LUT.len() / 4,
        X_LUT.len() - 1,
    ] {
        let (x0, s0) = (X_LUT[i], S_LUT[i]);
        for &j in &[
            0,
            X_LUT.len() / 4,
            X_LUT.len() / 2,
            3 * X_LUT.len() / 4,
            X_LUT.len() - 1,
        ] {
            let x1 = X_LUT[j];
            let ds = S_LUT[j].wrapping_sub(s0);
            let dx = dx_for_ds(x0, s0, ds);
            assert_eq!(x0.wrapping_add(dx), x1);
        }
    }
}

/// Round-trip inversion for 50k random movements: start at x0, compute ds to reach x1,
/// then invert back. The result x1_inv should equal x1 within 1 LSB, accounting for
/// the fact that interpolation between LUT samples introduces rounding.
#[test]
fn invert_round_trip_near_exact() {
    let mut rng = Rng::new(5);
    for _ in 0..50_000 {
        let x0 = gen_x_in_domain(&mut rng);
        let dx = gen_dx_bounded(&mut rng, x0);
        let x1 = x0.wrapping_add(dx);
        let s0 = evaluate_cost(x0);
        let ds = evaluate_cost(x1).wrapping_sub(s0);

        let dx_inv = dx_for_ds(x0, s0, ds);
        let x1_inv = x0.wrapping_add(dx_inv);
        let diff = x1_inv.abs_diff(x1);

        assert!(diff <= 1, "round-trip error: x1={}, x1_inv={}", x1, x1_inv);
    }
}

/// Midpoint interpolation: when inverting to a cumulative cost exactly halfway
/// between two LUT samples, the resulting x should map to approximately the x midpoint.
/// Tests interpolation accuracy for the inverse function (x_for_s).
#[test]
fn invert_midpoint_between_samples() {
    for i in (0..X_LUT.len() - 1).step_by(X_LUT.len() / 64) {
        let (x0, x1) = (X_LUT[i], X_LUT[i + 1]);
        let (s0, s1) = (S_LUT[i], S_LUT[i + 1]);
        let s_mid = ((s0 as u128 + s1 as u128) / 2) as u64;
        let x_mid_expected = ((x0 as u64 + x1 as u64) / 2) as u32;

        let dx = dx_for_ds(x0, s0, s_mid - s0);
        let x_mid = x0 + dx;
        let diff = x_mid.abs_diff(x_mid_expected);

        assert!(
            diff <= 1,
            "midpoint error: x_mid={}, expected={}",
            x_mid,
            x_mid_expected
        );
    }
}

// ===== Capacity Scaling Tests =====
//
// These tests verify that capacity-to-cumulative-cost mapping is correct.
// The contract: moving the full cmax (capacity) maps to full Smax (cumulative cost).
// Intermediate capacities scale linearly: ds = dc * Smax / cmax.

/// Full capacity maps to full cumulative cost span.
/// This is the anchor point for the linear scaling: cmax units of capacity
/// should produce exactly Smax units of cumulative cost.
#[test]
fn capacity_scale_full_range() {
    let cmax = 1_000_000u64;
    assert_eq!(ds_for_dc(cmax, cmax), LUT_S_MAX);
}

/// Composition test: dx_for_dc breaks down as dc → ds → dx.
/// Verifies that the composed function matches the direct calculation.
#[test]
fn dx_for_dc_consistency() {
    let (x0, cmax, dc) = (midpoint(), 1_000_000u64, 12_345u64);
    let s0 = evaluate_cost(x0);

    let ds = ds_for_dc(dc, cmax);
    let dx_direct = dx_for_ds(x0, s0, ds);
    let (dx, _) = dx_for_dc(x0, s0, dc, cmax);

    assert_eq!(dx, dx_direct);
}

/// Inverse composition test: dc_for_dx breaks down as dx → ds → dc.
/// Verifies that scaling back from movement delta to capacity delta is consistent.
#[test]
fn dc_for_dx_consistency() {
    let dx = ((LUT_X_MAX as u64 - LUT_X_MIN as u64) / 8) as u32;
    let (x0, cmax) = (midpoint(), 1_000_000u64);

    let ds = ds_for_dx(x0, dx);
    let num = (ds as u128) * (cmax as u128);
    let den = LUT_S_MAX as u128;
    let dc_expected = (num + den / 2) / den;
    let dc = dc_for_dx(x0, dx, cmax);

    assert_eq!(dc, dc_expected as u64);
}

// ===== Boundary Clamping Tests =====
//
// These tests verify that the curve is safely bounded: attempting to move beyond
// the domain boundaries [LUT_X_MIN, LUT_X_MAX] is gracefully clamped.
// Positive movement at X_MAX returns dx=0 (no movement possible).
// Negative movement at X_MIN returns dx=0 (no movement possible).
// Movements that overshoot are clamped to partial movement (not truncated to zero).

/// At X_MAX, requesting forward movement yields dx=0 (can't move right).
/// This guards against out-of-bounds lookups in the forward direction.
#[test]
fn clamp_at_x_max() {
    let (x0, cmax) = (LUT_X_MAX, 1_000_000u64);
    let (dx, _) = dx_for_dc(x0, evaluate_cost(x0), 1000, cmax);
    assert_eq!(dx, 0, "can't move beyond X_MAX");
}

/// At X_MIN, requesting backward movement stays within bounds.
/// Verifies that x_for_s clamps out-of-bounds cumulative costs safely.
#[test]
fn clamp_at_x_min() {
    let x0 = LUT_X_MIN;
    let s0 = evaluate_cost(x0);
    let dx = dx_for_ds(x0, s0, (-1_000_000_000i64) as u64);
    let x1 = x0.wrapping_add(dx);
    assert!(
        in_range(x1, LUT_X_MIN, LUT_X_MAX),
        "clamped position must stay in domain"
    );
}

/// Near boundary with overshooting movement: partial movement is allowed.
/// If x is at 11.9 and movement would go to 12.3, dx is clamped to 0.1.
/// This ensures graceful degradation rather than truncation.
#[test]
fn clamp_partial_movement() {
    let x0 = LUT_X_MAX - 100_000;
    let (dx, _) = dx_for_dc(x0, evaluate_cost(x0), 100_000, 1_000_000u64);
    let x1 = x0.wrapping_add(dx);

    assert!(
        in_range(x1, LUT_X_MIN, LUT_X_MAX),
        "result clamped within bounds"
    );
    assert!(dx > 0, "partial movement allowed");
}

/// `dc_for_dx` is protected: overshooting dx at boundary yields zero dc.
/// Tests that the evaluate_cost clamping propagates through dc_for_dx correctly.
#[test]
fn clamp_dc_for_dx_at_boundary() {
    let dc = dc_for_dx(LUT_X_MAX, 1_000_000, 1_000_000u64);
    assert_eq!(dc, 0, "overshooting dx yields zero dc");
}

/// `ds_for_dx` is protected: extreme wraparound is handled safely.
/// Tests that evaluate_cost clamping protects against out-of-bounds wrapping.
#[test]
fn clamp_ds_for_dx_wraps_safely() {
    let ds = ds_for_dx(LUT_X_MIN, (i32::MIN) as u32);
    assert!(ds > 0, "extreme wraparound handled safely");
}

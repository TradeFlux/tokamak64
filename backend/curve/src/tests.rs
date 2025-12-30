//! Tests for LUT delta mapping and inversion properties.

use super::lut::*;

// LUT domain helpers.

// 0.25 in Q16.16
const MARGIN: i32 = (1i32 << 16) / 4;

fn lo_x() -> i32 {
    COST_LUT.first().unwrap().x
}
fn hi_x() -> i32 {
    COST_LUT.last().unwrap().x
}

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

/// Verifies the zero step is a fixed point for delta mapping.
/// We pick a midpoint x in-domain and ensure no-op movement returns zero delta.
#[test]
fn delta_zero_is_zero() {
    let x0 = (lo_x() + hi_x()) / 2;
    assert_eq!(delta_cost_for_dx(x0, 0), 0);
}

/// Ensures delta mapping matches the explicit cumulative cost difference.
/// This ties `delta_cost_for_dx` to `evaluate_cost` across random in-domain steps.
#[test]
fn delta_matches_eval_difference_in_domain() {
    let mut rng = Rng::new(1);
    for _ in 0..50_000 {
        let x0 = rng.gen_i32(lo_x(), hi_x());
        let dx = rng.gen_i32(lo_x() - x0, hi_x() - x0);
        let x1 = x0 + dx;

        let lhs = delta_cost_for_dx(x0, dx);
        let rhs = evaluate_cost(x1) as i128 - evaluate_cost(x0) as i128;
        assert_eq!(lhs, rhs);
    }
}

/// Confirms additivity across two steps when all points stay in-domain.
/// The delta for a+b should equal the sum of deltas for a then b.
#[test]
fn path_independence_two_steps_in_domain() {
    let mut rng = Rng::new(2);

    for _ in 0..200_000 {
        let x0 = rng.gen_i32(lo_x() + MARGIN, hi_x() - MARGIN);
        let dx1 = rng.gen_i32(-MARGIN + 1, MARGIN - 1);
        let x_mid = x0 + dx1;

        let dx2_min = lo_x() - x_mid + 1;
        let dx2_max = hi_x() - x_mid - 1;
        if dx2_min > dx2_max {
            continue;
        }
        let dx2 = rng.gen_i32(dx2_min, dx2_max);

        let direct = delta_cost_for_dx(x0, dx1 + dx2);
        let step = delta_cost_for_dx(x0, dx1) + delta_cost_for_dx(x_mid, dx2);
        assert_eq!(direct, step);
    }
}

/// Confirms additivity across many steps with bounded movement.
/// This exercises longer paths to catch accumulated rounding errors.
#[test]
fn path_independence_many_steps_in_domain() {
    let mut rng = Rng::new(3);

    for _ in 0..50_000 {
        let mut x = rng.gen_i32(lo_x() + MARGIN, hi_x() - MARGIN);
        let x_start = x;

        let mut sum: i128 = 0;
        let steps = 8;
        for _ in 0..steps {
            let step_min = (lo_x() - x).max(-MARGIN / 2 + 1);
            let step_max = (hi_x() - x).min(MARGIN / 2 - 1);
            if step_min > step_max {
                continue;
            }
            let dx = rng.gen_i32(step_min, step_max);
            sum += delta_cost_for_dx(x, dx);
            x += dx;
        }

        let direct = delta_cost_for_dx(x_start, x - x_start);
        assert_eq!(direct, sum);
    }
}

/// Checks antisymmetry of the delta mapping under inversion of direction.
/// Moving forward then backward should negate the delta when in-bounds.
#[test]
fn antisymmetry_in_domain() {
    let mut rng = Rng::new(4);

    for _ in 0..200_000 {
        let x = rng.gen_i32(lo_x() + MARGIN, hi_x() - MARGIN);
        let dx = rng.gen_i32(-MARGIN, MARGIN);
        let x2 = x + dx;
        if x2 <= lo_x() || x2 >= hi_x() {
            continue;
        }

        let a = delta_cost_for_dx(x, dx);
        let b = delta_cost_for_dx(x2, -dx);
        assert_eq!(a, -b);
    }
}

/// Ensures exact inversion when targets land on LUT samples.
/// When both endpoints are exact LUT points, inversion should be exact.
#[test]
fn invert_delta_at_lut_points() {
    let indices = [
        0usize,
        COST_LUT.len() / 4,
        COST_LUT.len() / 2,
        (3 * COST_LUT.len()) / 4,
        COST_LUT.len() - 1,
    ];

    for &i in &indices {
        let x0 = COST_LUT[i].x;
        let s0 = COST_LUT[i].s as i128;

        for &j in &indices {
            let x1 = COST_LUT[j].x;
            let ds = COST_LUT[j].s as i128 - s0;
            let dx = dx_for_delta_cost(x0, s0, ds);
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
        let x0 = rng.gen_i32(lo_x(), hi_x());
        let dx = rng.gen_i32(lo_x() - x0, hi_x() - x0);
        let x1 = x0 + dx;
        let s0 = evaluate_cost(x0) as i128;
        let ds = evaluate_cost(x1) as i128 - s0;

        let dx_inv = dx_for_delta_cost(x0, s0, ds);
        let x1_inv = x0 + dx_inv;

        let diff = (x1_inv - x1).abs();
        assert!(diff <= 1, "x1={}, x1_inv={}", x1, x1_inv);
    }
}

/// Confirms mid-s interpolation maps near the x midpoint.
/// This checks the interpolation logic used by the inverse mapping.
#[test]
fn invert_delta_midpoint_between_samples() {
    let step = COST_LUT.len() / 64;
    for i in (0..COST_LUT.len() - 1).step_by(step) {
        let a = COST_LUT[i];
        let b = COST_LUT[i + 1];
        let s_mid = (a.s + b.s) / 2;
        let x_mid_expected = (a.x + b.x) / 2;

        let dx = dx_for_delta_cost(a.x, a.s as i128, s_mid as i128 - a.s as i128);
        let x_mid = a.x + dx;

        let diff = (x_mid - x_mid_expected).abs();
        assert!(diff <= 1, "x_mid={}, expected={}", x_mid, x_mid_expected);
    }
}

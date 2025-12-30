use super::lut::*;

fn lo() -> i32 {
    COST_X_LUT.first().unwrap().x
}
fn hi() -> i32 {
    COST_X_LUT.last().unwrap().x
}

fn clamp_x(x: i32) -> i32 {
    x.clamp(lo(), hi())
}

/// Deterministic PRNG (no external crates)
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

/// Margin so steps won't hit clamping in "unclamped" tests.
fn margin() -> i32 {
    // 0.25 in Q16.16
    (1i32 << 16) / 4
}

#[test]
fn delta_zero_is_zero() {
    let x0 = (lo() + hi()) / 2;
    assert_eq!(delta_s_for_dx(x0, 0), 0);
}

#[test]
fn delta_matches_eval_difference_including_clamp() {
    let mut rng = Rng::new(1);
    for _ in 0..50_000 {
        let x0 = rng.gen_i32(lo() - (5 << 16), hi() + (5 << 16)); // allow outside
        let dx = rng.gen_i32(-(40 << 16), 40 << 16); // big moves
        let x1 = x0.saturating_add(dx);

        let lhs = delta_s_for_dx(x0, dx);
        let rhs = eval_s(x1) as i128 - eval_s(x0) as i128;

        assert_eq!(lhs, rhs);
    }
}

#[test]
fn path_independence_two_step_without_clamping() {
    // Property: ΔS(x, a+b) == ΔS(x, a) + ΔS(x+a, b)
    // This holds exactly if we never hit clamp boundaries.
    let mut rng = Rng::new(2);
    let m = margin();

    for _ in 0..200_000 {
        // pick x0 safely inside the domain
        let x0 = rng.gen_i32(lo() + m, hi() - m);

        // choose dx1, dx2 small enough so intermediate positions stay inside domain
        let dx1 = rng.gen_i32(-m + 1, m - 1);
        let x_mid = x0.saturating_add(dx1);

        let max_dx2_lo = lo() - x_mid + 1;
        let max_dx2_hi = hi() - x_mid - 1;
        if max_dx2_lo > max_dx2_hi {
            continue;
        }
        let dx2 = rng.gen_i32(max_dx2_lo, max_dx2_hi);

        let direct = delta_s_for_dx(x0, dx1.saturating_add(dx2));
        let step = delta_s_for_dx(x0, dx1) + delta_s_for_dx(x_mid, dx2);

        assert_eq!(direct, step);
    }
}

#[test]
fn path_independence_many_steps_without_clamping() {
    // Property: sum of incremental deltas equals direct delta, when staying in-bounds.
    let mut rng = Rng::new(3);
    let m = margin();

    for _case in 0..50_000 {
        let mut x = rng.gen_i32(lo() + m, hi() - m);
        let x_start = x;

        let mut sum: i128 = 0;
        let steps = 8;

        for _ in 0..steps {
            // step small enough to stay well inside bounds
            let dx = rng.gen_i32(-m / 2 + 1, m / 2 - 1);
            sum += delta_s_for_dx(x, dx);
            x = x.saturating_add(dx);
        }

        let direct = delta_s_for_dx(x_start, x.saturating_sub(x_start));
        assert_eq!(direct, sum);
    }
}

#[test]
fn path_independence_with_clamping() {
    // With clamping, the "state" is effectively clamp(x).
    // So the correct path-independence statement is:
    //
    //   eval(clamp(x_end)) - eval(clamp(x_start))
    // equals the sum of step deltas if you update x each step (and each delta uses eval which clamps).
    //
    let mut rng = Rng::new(4);

    for _case in 0..100_000 {
        let mut x = rng.gen_i32(lo() - (10 << 16), hi() + (10 << 16));
        let x_start = x;

        let mut sum: i128 = 0;
        let steps = 6;

        for _ in 0..steps {
            let dx = rng.gen_i32(-(10 << 16), 10 << 16);
            sum += delta_s_for_dx(x, dx);
            x = x.saturating_add(dx);
        }

        let direct = eval_s(x) as i128 - eval_s(x_start) as i128;
        assert_eq!(direct, sum);

        // Also explicitly show it's "clamped endpoints" that matter:
        let direct_clamped = eval_s(clamp_x(x)) as i128 - eval_s(clamp_x(x_start)) as i128;
        assert_eq!(direct, direct_clamped);
    }
}

#[test]
fn antisymmetry_without_clamping() {
    // If we stay in-bounds:
    //   ΔS(x, dx) == -ΔS(x+dx, -dx)
    let mut rng = Rng::new(5);
    let m = margin();

    for _ in 0..200_000 {
        let x = rng.gen_i32(lo() + m, hi() - m);
        let dx = rng.gen_i32(-m, m);
        let x2 = x.saturating_add(dx);
        if x2 <= lo() || x2 >= hi() {
            continue;
        }

        let a = delta_s_for_dx(x, dx);
        let b = delta_s_for_dx(x2, -dx);
        assert_eq!(a, -b);
    }
}

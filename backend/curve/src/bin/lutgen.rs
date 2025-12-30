//! LUT generator for the curve crate.
//!
//! Generates a symmetric lookup table for cumulative sigmoid mass over
//! `x ∈ [-X_MAX, X_MAX]` in fixed-point:
//! - x: Q16.16
//! - s: Q16.48
//!
//! Sampling is geometric away from zero to concentrate points where curvature
//! is highest while keeping the tails sparse. Output is Rust source that can be
//! redirected into `backend/curve/src/lut.rs`.
//!
//! Usage:
//!   cargo run -p curve --bin lutgen --release > backend/curve/src/lut-samples.rs

const X_MAX: f64 = 6.0;
const SAMPLE_COUNT: usize = 1025; // odd -> symmetric with a single 0
const GEOM_RATIO: f64 = 1.005;

const X_FRAC_BITS: u32 = 16; // Q16.16
const S_FRAC_BITS: u32 = 48; // Q16.48

/// Numerically stable softplus used for the cumulative sigmoid integral.
fn softplus(x: f64) -> f64 {
    if x > 0.0 {
        x + (1.0 + (-x).exp()).ln()
    } else {
        (1.0 + x.exp()).ln()
    }
}

/// Convert a floating-point value to unsigned Q16.48.
fn to_q16_48(v: f64) -> u64 {
    let scaled = (v * ((1u128 << S_FRAC_BITS) as f64)).round();
    if scaled <= 0.0 {
        0
    } else if scaled >= u64::MAX as f64 {
        u64::MAX
    } else {
        scaled as u64
    }
}

/// Geometric spacing from 0 to X_MAX, inclusive.
fn geometric_distance(i: usize, count: usize) -> f64 {
    if i == 0 {
        return 0.0;
    }
    if i == count - 1 {
        return X_MAX;
    }
    let num = GEOM_RATIO.powi(i as i32) - 1.0;
    let den = GEOM_RATIO.powi((count - 1) as i32) - 1.0;
    X_MAX * (num / den)
}

/// Build the strictly increasing non-negative grid in Q16.16.
fn build_positive_x_grid() -> Vec<i32> {
    // SAMPLE_COUNT = 2 * (half - 1) + 1  =>  half = (SAMPLE_COUNT + 1) / 2
    let half = (SAMPLE_COUNT + 1) / 2;
    let xmax_q = (X_MAX * ((1u64 << X_FRAC_BITS) as f64)).round() as i32;

    let mut grid = Vec::with_capacity(half);
    grid.push(0);

    let mut prev = 0i32;
    for i in 1..(half - 1) {
        let d = geometric_distance(i, half);
        let mut x_q = (d * ((1u64 << X_FRAC_BITS) as f64)).round() as i32;

        // Enforce strict monotonicity after rounding.
        if x_q <= prev {
            x_q = prev.saturating_add(1);
        }
        if x_q >= xmax_q {
            x_q = xmax_q - 1;
        }

        grid.push(x_q);
        prev = x_q;
    }
    grid.push(xmax_q);

    for w in grid.windows(2) {
        assert!(w[0] < w[1]);
    }

    grid
}

fn main() {
    let positive = build_positive_x_grid();

    // Mirror around zero: negatives (excluding 0), 0, positives (excluding 0).
    let mut full_x_grid: Vec<i32> = Vec::with_capacity(SAMPLE_COUNT);
    for &x in positive.iter().skip(1).rev() {
        full_x_grid.push(-x);
    }
    full_x_grid.push(0);
    full_x_grid.extend(positive.iter().skip(1).copied());

    assert_eq!(full_x_grid.len(), SAMPLE_COUNT);
    for w in full_x_grid.windows(2) {
        assert!(w[0] < w[1]);
    }

    // Baseline so cumulative cost starts at zero at -X_MAX.
    let s_baseline = softplus(-X_MAX);

    println!("pub(crate) struct CostLutPoint {{ pub(crate) x: i32, pub(crate) s: u64 }}");
    println!();
    println!("pub(crate) static COST_LUT: [CostLutPoint; {SAMPLE_COUNT}] = [",);

    for x_q in full_x_grid {
        let x = (x_q as f64) / ((1u64 << X_FRAC_BITS) as f64);
        let s_q16_48 = to_q16_48(softplus(x) - s_baseline);
        println!("    CostLutPoint {{ x: {x_q}, s: {s_q16_48} }},");
    }

    println!("];\n");
}

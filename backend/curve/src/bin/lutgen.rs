//! LUT generator for the curve crate.
//!
//! Generates a lookup table for cumulative sigmoid mass over
//! `x ∈ [0, X_MAX]` in fixed-point with an inflection point at 3.0:
//! - x: Q8.24 (stored as u32, representing [0, 6])
//! - s: Q16.48 (stored as u64)
//!
//! The sigmoid is the standard logistic function. We generate it symmetrically
//! around 0, then shift all x-values by +3.0 to move the inflection point
//! to x=3.0 while keeping cumulative cost values unchanged.
//!
//! Usage:
//!   cargo run -p curve --bin lutgen --release > backend/curve/src/lut-samples.rs

const X_MAX: f64 = 3.0;
const X_OFFSET: f64 = X_MAX; // Shift to move inflection point from 0 to 3.0
const SAMPLE_COUNT: usize = 1025; // odd -> symmetric with a single 0
const GEOM_RATIO: f64 = 1.005;

const X_FRAC_BITS: u32 = 24; // Q8.24
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

/// Build the strictly increasing non-negative grid in Q8.24, as i32.
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

    // Shift x-values by X_OFFSET (move inflection point from 0 to 6.0)
    let x_offset_q = (X_OFFSET * ((1u64 << X_FRAC_BITS) as f64)).round() as i32;
    let shifted_x_grid: Vec<u32> = full_x_grid
        .iter()
        .map(|&x| (x + x_offset_q) as u32)
        .collect();

    println!("pub(crate) static X_LUT: [u32; {SAMPLE_COUNT}] = [");
    for &x_q in &shifted_x_grid {
        println!("    {x_q},");
    }
    println!("];\n");

    println!("pub(crate) static S_LUT: [u64; {SAMPLE_COUNT}] = [");
    for x_q in &full_x_grid {
        let x = (*x_q as f64) / ((1u64 << X_FRAC_BITS) as f64);
        let s_q16_48 = to_q16_48(softplus(x) - s_baseline);
        println!("    {s_q16_48},");
    }
    println!("];\n");
}

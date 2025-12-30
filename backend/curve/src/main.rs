// Generates a strictly symmetric LUT for cumulative sigmoid mass over x ∈ [-12, +12].
//
// Stored values:
//   x: i32 Q16.16 (strictly symmetric: for every +x there is exactly -x)
//   s: u64 Q16.48
//
//   s(x) = ∫_{-12}^{x} σ(t) dt
//        = softplus(x) - softplus(-12)
//   softplus(z) = ln(1 + e^z)
//
// N = 1025 (perfect symmetry with a single 0).
//
// Run:
//   cargo run --release > cost_x_lut.rs

use std::f64;

const XMAX: f64 = 6.0;
const N: usize = 1025;
const R: f64 = 1.005;

const X_FRAC_BITS: u32 = 16; // Q16.16
const S_FRAC_BITS: u32 = 48; // Q16.48

fn softplus(x: f64) -> f64 {
    // numerically stable
    if x > 0.0 {
        x + (1.0 + (-x).exp()).ln()
    } else {
        (1.0 + x.exp()).ln()
    }
}

// Geometric distances from 0 to XMAX:
// d(0)=0, d(m-1)=XMAX, geometric in between.
fn geom_distance(i: usize, m: usize) -> f64 {
    if i == 0 {
        return 0.0;
    }
    if i == m - 1 {
        return XMAX;
    }
    let num = R.powi(i as i32) - 1.0;
    let den = R.powi((m - 1) as i32) - 1.0;
    XMAX * (num / den)
}

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

fn main() {
    // N = 2*(m-1) + 1  => m = (N+1)/2 = 513
    let m = (N + 1) / 2;

    let xmax_q: i32 = (XMAX * ((1u64 << X_FRAC_BITS) as f64)).round() as i32;

    // Build strictly increasing, unique positive x-grid in integer Q16.16.
    // pos[0] = 0, pos[m-1] = +12 exactly.
    let mut pos: Vec<i32> = Vec::with_capacity(m);
    pos.push(0);

    let mut prev = 0i32;
    for i in 1..(m - 1) {
        let d = geom_distance(i, m);
        let mut x_q = (d * ((1u64 << X_FRAC_BITS) as f64)).round() as i32;

        // Enforce strict monotonicity in integer space (avoid duplicates from rounding).
        if x_q <= prev {
            x_q = prev.saturating_add(1);
        }
        // Keep within bounds
        if x_q >= xmax_q {
            x_q = xmax_q - 1; // leave room for the final endpoint
        }

        pos.push(x_q);
        prev = x_q;
    }
    pos.push(xmax_q);

    // Sanity: strictly increasing
    for w in pos.windows(2) {
        assert!(w[0] < w[1]);
    }

    // Build full symmetric xs in increasing order:
    // negatives (excluding 0), then 0, then positives (excluding 0)
    let mut xs: Vec<i32> = Vec::with_capacity(N);

    for &x in pos.iter().skip(1).rev() {
        xs.push(-x);
    }
    xs.push(0);
    for &x in pos.iter().skip(1) {
        xs.push(x);
    }

    assert_eq!(xs.len(), N);
    for w in xs.windows(2) {
        assert!(w[0] < w[1]);
    }

    // Precompute baseline softplus(-12)
    let sp_min = softplus(-XMAX);

    println!("pub struct CostPoint {{ pub x: i32, pub s: u64 }}");
    println!();
    println!("pub static COST_X_LUT: [CostPoint; {}] = [", N);

    for x_q in xs {
        let x = (x_q as f64) / ((1u64 << X_FRAC_BITS) as f64);
        let s = softplus(x) - sp_min;
        let s_q = to_q16_48(s);
        println!("    CostPoint {{ x: {}, s: {} }},", x_q, s_q);
    }

    println!("];");
}

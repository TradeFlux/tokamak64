use curve::lut::LUT_X_MAX;

const MAX_Z: u64 = 26;
const MAX_X: u64 = LUT_X_MAX as u64 * 2;
const D: u64 = MAX_X * MAX_Z;
const SPEED_DECAY_SECS: u64 = 32;
const MAX_DELTA_TS: u64 = 8;
const DECAY_MULTIPLIER: u64 = 2 << MAX_DELTA_TS;

struct Movement {
    balance: u64,
    delta_z: u8,
    saturation: u32,
    delta_ts: u64,
}

fn calculate_fee(mv: Movement) -> u64 {
    let n = mv.delta_z as u64 * mv.saturation as u64 * DECAY_MULTIPLIER;
    let d = 2 << mv.delta_ts.min(MAX_DELTA_TS);
    mul_div_round_nearest_u64(n, mv.balance, d)
}

#[inline]
pub fn mul_div_round_nearest_u64(n: u64, k: u64, d: u64) -> u64 {
    let nd = (n as u128) * (k as u128);
    let d = d as u128;
    ((nd + d / 2) / d) as u64
}

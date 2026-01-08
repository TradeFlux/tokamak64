use curve::lut::LUT_X_MAX;

pub const MAX_Z: u64 = 26;
// LUT_X_MAX is u32 in Q8.24 (range [0, 12])
pub const MAX_X: u32 = LUT_X_MAX;
pub const MIN_FEE: u64 = 100_000;
pub const DECIMALS: u8 = 6;
pub const MAX_SPEED_MULTIPLIER: u64 = 127;
pub const MAX_DELTA_TS: u64 = 1024;

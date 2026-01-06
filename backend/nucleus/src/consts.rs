use curve::lut::LUT_X_MAX;

pub const MAX_Z: u64 = 26;
pub const MAX_X: u64 = LUT_X_MAX as u64 * 2;
pub const MIN_FEE: u64 = 100_000;
pub const MAX_SPEED_MULTIPLIER: u64 = 127;
pub const MAX_DELTA_TS: u64 = 1024;

/// Maximum atomic number (0-indexed, so 27 distinct elements).
pub const MAX_ATOMIC_NUMBER: u64 = 26;

/// Maximum curve position (Q8.24, range [0, 12]).
pub const MAX_SATURATION: u32 = curve::lut::LUT_X_MAX;

/// Minimum fee (in Gluon) to prevent dust.
pub const MIN_FEE: u64 = 100_000;

/// Token decimals (Gluon precision).
pub const DECIMALS: u8 = 6;

/// Maximum speed multiplier (applied based on action timing).
pub const MAX_SPEED_MULTIPLIER: u64 = 127;

/// Maximum elapsed time window for speed tax (slots).
pub const MAX_DELTA_TIMESTAMP: u64 = 1024;

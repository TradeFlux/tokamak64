pub mod action;
pub mod board;
pub mod consts;
pub mod fees;
pub mod player;
pub mod types;

#[cfg(test)]
mod tests;

/// Two's complement negation: bitwise inverse + 1.
/// Converts signed values (encoded in u64) to their opposite for curve calculations.
/// Caller must ensure no overflow occurs (typically only called on interpreted signed values).
#[inline]
pub fn twos_comp_negate(s: u64) -> u64 {
    // Bitwise negation: !s
    // Then add 1: !s + 1 (mathematically equivalent to 0 - s, but using bitwise ops)
    (!s).wrapping_add(1)
}

/// Unsigned round-divide: `(mul1 * mul2 / div)` with nearest rounding (ties away from zero).
/// All values treated as unsigned u64. For signed arithmetic, convert before calling.
#[inline]
pub fn round_divide(mul1: u64, mul2: u64, div: u64) -> u64 {
    let product = (mul1 as u128) * (mul2 as u128);
    let divisor = div as u128;
    ((product + divisor / 2) / divisor) as u64
}

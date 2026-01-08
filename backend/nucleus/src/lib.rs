pub mod action;
pub mod board;
pub mod consts;
pub mod fees;
pub mod player;
pub mod types;

/// Negates a value using two's complement (wrapping negation).
/// Used when converting signed game values (e.g., balance) to unsigned curve parameters.
///
/// For a signed value `s` encoded as two's complement in `u64`:
/// - Positive: represented as-is
/// - Negative: represented with high bit set, semantically `-(2^64 - s)`
///
/// This function computes the bitwise negation: `!s + 1` = `0_u64.wrapping_sub(s)`
#[inline]
pub fn tc(s: u64) -> u64 {
    s.wrapping_neg()
}

/// Unsigned round-to-nearest multiply-divide: (mul1 * mul2 / div) rounded nearest.
/// All operands and result are treated as unsigned u64 (ties away from zero).
///
/// For signed arithmetic, convert signed values to unsigned before calling,
/// then interpret the result as two's complement if needed.
#[inline]
pub fn round_div(mul1: u64, mul2: u64, div: u64) -> u64 {
    let product = (mul1 as u128) * (mul2 as u128);
    let divisor = div as u128;
    ((product + divisor / 2) / divisor) as u64
}

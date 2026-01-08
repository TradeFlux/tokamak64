//! Movement and action fees: injection, ejection, rebind, compression, and speed tax bonuses.

use crate::{
    board::Element,
    consts::{
        MAX_ATOMIC_NUMBER, MAX_DELTA_TIMESTAMP, MAX_SATURATION, MAX_SPEED_MULTIPLIER, MIN_FEE,
    },
    player::Charge,
    round_divide,
    types::Gluon,
};

/// Rebind fee: cost to move a charge between two elements (distance-based).
/// Higher atomic number distance and higher saturation = higher fee.
pub fn rebind_fee(charge: &Charge, src: &Element, dst: &Element) -> Gluon {
    let src_z = src.index.atomic_number();
    let dst_z = dst.index.atomic_number();
    let delta_z = dst_z.wrapping_sub(src_z);
    let curve = if src.index > dst.index {
        &src.curve
    } else {
        &dst.curve
    };
    calculate_base_fee(charge.balance, delta_z, curve.saturation)
}

/// Injection fee: cost to bind a charge to an element (first commitment).
/// Prevents spam and seeds the element pot.
pub fn injection_fee(charge: &Charge, dst: &Element) -> Gluon {
    calculate_base_fee(
        charge.balance,
        dst.index.atomic_number(),
        dst.curve.saturation,
    )
}

/// Ejection fee: cost to unbind a charge from an element (abandoning commitment).
/// Prevents rapid cycling and ensures skin-in-game.
pub fn ejection_fee(charge: &Charge, src: &Element) -> Gluon {
    calculate_base_fee(
        charge.balance,
        src.index.atomic_number(),
        src.curve.saturation,
    )
}

/// Compression fee: cost to compress an element inward (consolidate into deeper element).
/// Accelerates element convergence toward center.
pub fn compression_fee(src: &Element) -> Gluon {
    let numerator = src.curve.saturation as u64 * 5;
    let denominator = (MAX_SATURATION as u64) * 100;
    let result = round_divide(src.pot, numerator, denominator);
    result.max(MIN_FEE)
}

/// Speed bonus: increases with time since last action. Rewards patience.
/// Elapsed time is quadratic capped at MAX_DELTA_TIMESTAMP. Returns multiplier >= 1.
pub fn speed_bonus(charge: &Charge, now: u64) -> u64 {
    let elapsed = now.saturating_sub(charge.timestamp);
    let time_factor = elapsed.min(MAX_DELTA_TIMESTAMP).pow(2);
    let max_factor = (MAX_DELTA_TIMESTAMP).pow(2);
    1 + round_divide(MAX_SPEED_MULTIPLIER, time_factor, max_factor)
}

/// Calculate base fee: balance * (distance * saturation) / (MAX_ATOMIC_NUMBER * MAX_POSITION).
/// Ensures fees scale with commitment, depth, and element's curve saturation.
fn calculate_base_fee(balance: Gluon, distance: u64, saturation: u32) -> Gluon {
    let numerator = distance * (saturation as u64);
    let denominator = MAX_ATOMIC_NUMBER * (MAX_SATURATION as u64);
    let result = round_divide(balance, numerator, denominator);
    result.max(MIN_FEE)
}

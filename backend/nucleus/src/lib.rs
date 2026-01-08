//! # Nucleus: Core Game Data Model and Mechanics
//!
//! Nucleus defines the fundamental data structures, types, constants, and action logic that drive TOKAMAK64.
//! It is a pure data layer with no blockchain dependencies; core game rules are expressed as deterministic functions.
//!
//! ## Modules
//!
//! - **types**: Fixed-point math types (Q8.24, Q16.48), addresses, element indices, and board coordinates
//! - **player**: Wallet and Charge account structures; player balance and binding state
//! - **board**: Element curve state, pot mechanics, and global Board/Artefact snapshots
//! - **action**: Core game logic for rebinding, claiming rewards, compressing elements, and venting value
//! - **fees**: Movement cost and fee calculation logic based on depth and speed tax
//! - **consts**: Game parameters and constants (max saturation, depth multipliers, etc.)

pub mod action;
pub mod board;
pub mod consts;
pub mod fees;
pub mod player;
pub mod types;

#[cfg(test)]
mod tests;

/// Unsigned round-divide: `(mul1 * mul2 / div)` with nearest rounding (ties away from zero).
/// All values treated as unsigned u64. For signed arithmetic, convert before calling.
#[inline]
pub fn round_divide(mul1: u64, mul2: u64, div: u64) -> u64 {
    let product = (mul1 as u128) * (mul2 as u128);
    let divisor = div as u128;
    ((product + divisor / 2) / divisor) as u64
}

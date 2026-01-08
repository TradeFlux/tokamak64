//! Common utilities for instruction processors.

use nucleus::{fees::fee_multiplier, player::Charge, types::Gluon};
use pinocchio::error::ProgramError;
use pinocchio::sysvars::{clock::Clock, Sysvar};

/// Apply speed tax multiplier to a base fee and update charge timestamp.
///
/// 1. Gets current slot from Clock sysvar
/// 2. Computes fee multiplier based on time since last action
/// 3. Multiplies base fee by multiplier (saturating)
/// 4. Updates charge.timestamp to current slot
///
/// Returns the final fee after speed tax.
#[inline]
pub fn apply_speed_tax(charge: &mut Charge, base_fee: Gluon) -> Result<Gluon, ProgramError> {
    let clock = Clock::get()?;
    let multiplier = fee_multiplier(charge, clock.slot);
    let fee = base_fee.saturating_mul(multiplier);
    charge.timestamp = clock.slot;
    Ok(fee)
}

/// Deduct fee from charge balance, failing if insufficient funds.
#[inline]
pub fn deduct_fee(charge: &mut Charge, fee: Gluon) -> Result<(), ProgramError> {
    charge.balance = charge
        .balance
        .checked_sub(fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    Ok(())
}

/// Apply speed tax and deduct from charge balance in one operation.
#[inline]
pub fn charge_fee(charge: &mut Charge, base_fee: Gluon) -> Result<Gluon, ProgramError> {
    let fee = apply_speed_tax(charge, base_fee)?;
    deduct_fee(charge, fee)?;
    Ok(fee)
}

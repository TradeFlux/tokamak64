//! Rebind a charge to a different element.

use nucleus::{action, fees::rebind_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, RebindAccounts};

/// Move a bound charge from source Element to an adjacent Element; incurs movement cost.
pub(crate) fn rebind<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let RebindAccounts { charge, src, dst } = RebindAccounts::extract(it)?;

    let fee = rebind_fee(charge, src, dst);
    src.coordinates
        .adjacent(dst.coordinates)
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;

    // Charge must be bound to source element
    if charge.index != src.index {
        return Err(ProgramError::Custom(1)); // Charge not in source element
    }

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::rebind(charge, src, dst);
    // Fee routing: moving inward (src.index > dst.index, higher atomic #) pays to dst;
    // moving outward (src.index < dst.index, lower atomic #) pays to src.
    if src.index > dst.index {
        src.pot += fee; // Moving outward: fee stays with departing element
    } else {
        dst.pot += fee; // Moving inward: fee funds deeper element
    }

    Ok(())
}

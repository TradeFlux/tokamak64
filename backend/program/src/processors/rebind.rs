use nucleus::{action, fees::migration_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, RebindAccounts};

/// Move a bound charge from source Element to an adjacent Element; incurs movement cost.
pub(crate) fn rebind<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let RebindAccounts { charge, src, dst } = RebindAccounts::parse(it)?;
    let fee = migration_fee(charge, src, dst);
    src.coordinates
        .adjacent(dst.coordinates)
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::rebind(charge, src, dst);
    if src.index > dst.index {
        src.pot += fee;
    } else {
        dst.pot += fee;
    }

    Ok(())
}

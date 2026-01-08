//! Merge element pots via compression between adjacent elements.

use nucleus::{
    action,
    fees::{merge_fee, migration_fee},
};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, CompressionAccounts, FromAccounts};

/// Move Element's pot inward to deeper destination and rebind charge; adds fees to moving pot.
pub(crate) fn compress<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let CompressionAccounts {
        charge,
        src,
        dst,
    } = CompressionAccounts::extract(it)?;

    if src.index > dst.index {
        // TODO proper handling of compression error (only towards increasing Z)
        return Err(ProgramError::Custom(42));
    }
    let shift_fee = migration_fee(charge, src, dst);
    let merge = merge_fee(src);

    let remainder = charge.balance.checked_sub(shift_fee + merge);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::compress(charge, src, dst);
    dst.pot += shift_fee + merge;

    Ok(())
}

//! Merge element pots via compression between adjacent elements.

use nucleus::{
    action,
    fees::{compression_fee, rebind_fee},
};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, CompressionAccounts, FromAccounts};

/// Move Element's pot inward to deeper destination and rebind charge; adds fees to moving pot.
/// Migration fee (depth-based movement cost) + merge fee (consolidation tax) both go to destination.
pub(crate) fn compress<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let CompressionAccounts { charge, src, dst } = CompressionAccounts::extract(it)?;

    // Charge must be bound to source element
    if charge.index != src.index {
        return Err(ProgramError::Custom(1)); // Charge not in source element
    }

    if src.index > dst.index {
        // TODO proper handling of compression error (only towards increasing Z)
        return Err(ProgramError::Custom(42));
    }
    // Rebind fee: cost to move inward, scales with destination depth and saturation
    // Compression fee: cost to consolidate source pot, scales with source saturation and pot size
    let fee = rebind_fee(charge, src, dst) + compression_fee(src);

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::compress(charge, src, dst);
    // Both fees accumulate in destination pot (investment in deeper element)
    dst.pot += fee;

    Ok(())
}

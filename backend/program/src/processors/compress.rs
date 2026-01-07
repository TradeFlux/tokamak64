use nucleus::{
    action,
    fees::{compression_fee, shift_fee},
};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, CompressionAccounts, FromAccounts};

pub(crate) fn process_compress<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let CompressionAccounts { charge, src, dst } = CompressionAccounts::parse(it)?;
    if src.index > dst.index {
        // TODO proper handling of compression error (only towards increasing Z)
        return Err(ProgramError::Custom(42));
    }
    let shift_fee = shift_fee(charge, src, dst);
    let compression_fee = compression_fee(src);

    let remainder = charge.balance.checked_sub(shift_fee + compression_fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::compress(charge, src, dst);
    dst.pot += shift_fee + compression_fee;

    Ok(())
}

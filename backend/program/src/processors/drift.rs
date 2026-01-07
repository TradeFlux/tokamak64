use nucleus::{action, fees::shift_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, DriftAccounts, FromAccounts};

pub(crate) fn process_drift<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let DriftAccounts { charge, src, dst } = DriftAccounts::parse(it)?;
    let fee = shift_fee(charge, src, dst);

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::drift(charge, src, dst);
    if src.index > dst.index {
        src.pot += fee;
    } else {
        dst.pot += fee;
    }

    Ok(())
}

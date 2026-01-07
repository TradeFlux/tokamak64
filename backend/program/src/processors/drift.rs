use nucleus::{action, fees::shift_fee};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::accounts::{DriftAccounts, FromAccounts};

pub(crate) fn process_drift<'a, I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> ProgramResult {
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

use nucleus::{action, fees::shift_fee};
use pinocchio::{program_error::ProgramError, ProgramResult};

use crate::accounts::DriftAccounts;

fn drift(accounts: DriftAccounts) -> ProgramResult {
    let DriftAccounts { charge, src, dst } = accounts;
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

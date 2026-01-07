use nucleus::{fees::shift_fee, movement};
use pinocchio::{program_error::ProgramError, ProgramResult};

use crate::accounts::ShiftAccounts;

fn shift(accounts: ShiftAccounts) -> ProgramResult {
    let ShiftAccounts { charge, src, dst } = accounts;
    let fee = shift_fee(charge, src, dst);

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    movement::shift(charge, src, dst);
    if src.index > dst.index {
        src.pot += fee;
    } else {
        dst.pot += fee;
    }

    Ok(())
}

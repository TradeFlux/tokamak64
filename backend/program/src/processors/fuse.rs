use nucleus::fees::entry_fee;
use pinocchio::{program_error::ProgramError, ProgramResult};

use crate::accounts::EnterAccounts;

fn enter(accounts: EnterAccounts) -> ProgramResult {
    let EnterAccounts { charge, dst, board } = accounts;
    let fee = entry_fee(charge, dst);

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(())
}

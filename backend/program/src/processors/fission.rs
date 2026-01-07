use nucleus::{action, fees::fission_fee};
use pinocchio::{program_error::ProgramError, ProgramResult};

use crate::accounts::FissionAccounts;

fn fission(accounts: FissionAccounts) -> ProgramResult {
    let FissionAccounts { charge, src, board } = accounts;
    let fee = fission_fee(charge, src);
    board.tvl += charge.balance;
    board.charges += 1;

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::fission(charge, src);
    src.pot += fee;

    Ok(())
}

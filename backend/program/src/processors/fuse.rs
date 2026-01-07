use nucleus::{fees::fusion_fee, action};
use pinocchio::{program_error::ProgramError, ProgramResult};

use crate::accounts::FusionAccounts;

fn fuse(accounts: FusionAccounts) -> ProgramResult {
    let FusionAccounts { charge, dst, board } = accounts;
    let fee = fusion_fee(charge, dst);
    board.tvl += charge.balance;
    board.charges += 1;

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::fuse(charge, dst);
    dst.pot += fee;

    Ok(())
}

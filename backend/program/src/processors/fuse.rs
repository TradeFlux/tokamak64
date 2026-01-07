use nucleus::{action, fees::fusion_fee};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::accounts::{FromAccounts, FusionAccounts};

pub(crate) fn process_fuse<'a, I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> ProgramResult {
    let FusionAccounts { charge, dst, board } = FusionAccounts::parse(it)?;
    let fee = fusion_fee(charge, dst);
    board.tvl += charge.balance;
    board.charges += 1;

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::fuse(charge, dst);
    dst.pot += fee;

    Ok(())
}

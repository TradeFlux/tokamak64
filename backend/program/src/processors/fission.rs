use bytemuck::Zeroable;
use nucleus::{action, board::Element, fees::fission_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FissionAccounts, FromAccounts};

pub(crate) fn process_fission<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let FissionAccounts { charge, src, board } = FissionAccounts::parse(it)?;
    let fee = fission_fee(charge, src);
    board.tvl += charge.balance;
    board.charges += 1;

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    let mut dst = Element::zeroed();
    action::translate(charge, src, &mut dst);
    src.pot += fee;

    Ok(())
}

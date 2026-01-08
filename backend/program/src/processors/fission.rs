use bytemuck::Zeroable;
use nucleus::{action, board::Element, fees::exit_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FissionAccounts, FromAccounts};

pub(crate) fn process_fission<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let FissionAccounts { charge, src, board } = FissionAccounts::parse(it)?;
    src.coordinates
        .is_peripheral()
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;
    let fee = exit_fee(charge, src);
    board.tvl += charge.balance;
    board.charge_count += 1;

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    let mut dst = Element::zeroed();
    action::rebind(charge, src, &mut dst);
    src.pot += fee;

    Ok(())
}

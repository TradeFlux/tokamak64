use bytemuck::Zeroable;
use nucleus::{action, board::Element, fees::fusion_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, FusionAccounts};

pub(crate) fn process_fuse<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let FusionAccounts { charge, dst, board } = FusionAccounts::parse(it)?;
    dst.coordinates
        .peripheral()
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;
    let fee = fusion_fee(charge, dst);
    board.tvl += charge.balance;
    board.charges += 1;

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    let mut src = Element::zeroed();
    action::translate(charge, &mut src, dst);
    dst.pot += fee;

    Ok(())
}

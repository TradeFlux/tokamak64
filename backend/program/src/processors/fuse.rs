//! Bind charge onto board into edge Element; charge becomes bound.

use bytemuck::Zeroable;
use nucleus::{action, board::Element, fees::entry_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, FusionAccounts};

/// Bind a charge onto the board into an edge Element; charge becomes bound for pressure/overload mechanics.
pub(crate) fn fuse<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let FusionAccounts {
        charge,
        dst,
        board,
    } = FusionAccounts::extract(it)?;

    dst.coordinates
        .is_peripheral()
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;
    let fee = entry_fee(charge, dst);
    board.tvl += charge.balance;
    board.charge_count += 1;

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    let mut src = Element::zeroed();
    action::rebind(charge, &mut src, dst);
    dst.pot += fee;

    Ok(())
}

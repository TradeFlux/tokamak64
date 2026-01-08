//! Bind charge onto board into edge Element; charge becomes bound.

use bytemuck::Zeroable;
use nucleus::{action, board::Element, fees::entry_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, InjectionAccounts};

/// Bind a charge onto the board into an edge Element; charge becomes bound for pressure/overload mechanics.
pub(crate) fn inject<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let InjectionAccounts { charge, dst, board } = InjectionAccounts::extract(it)?;

    dst.coordinates
        .on_edge()
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

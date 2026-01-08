//! Unbind charge from board and move it outside; edge Elements only.

use bytemuck::Zeroable;
use nucleus::{action, board::Element, fees::exit_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, EjectionAccounts, FromAccounts};

/// Unbind a charge from its current Element and move it outside the board; only from edge Elements.
pub(crate) fn eject<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let EjectionAccounts { charge, src, board } = EjectionAccounts::extract(it)?;

    src.coordinates
        .is_peripheral()
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;
    let fee = exit_fee(charge, src);
    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;

    board.tvl -= charge.balance;
    board.charge_count -= 1;

    let mut dst = Element::zeroed();
    action::rebind(charge, src, &mut dst);
    src.pot += fee;

    Ok(())
}

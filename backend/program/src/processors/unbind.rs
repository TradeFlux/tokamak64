//! Unbind charge from board and move it outside; edge Elements only.

use bytemuck::Zeroable;
use nucleus::{action, board::Element, fees::unbind_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use super::common::charge_fee;
use crate::accounts::{AccountIter, FromAccounts, UnbindAccounts};

/// Unbind a charge from its current Element and move it outside the board; only from edge Elements.
pub(crate) fn unbind<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let UnbindAccounts { charge, src, board } = UnbindAccounts::extract(it)?;

    src.coordinates
        .on_edge()
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;

    let fee = charge_fee(charge, unbind_fee(charge, src))?;

    board.tvl -= charge.balance;
    board.charge_count -= 1;

    let mut dst = Element::zeroed();
    action::rebind(charge, src, &mut dst);
    src.pot += fee;

    Ok(())
}

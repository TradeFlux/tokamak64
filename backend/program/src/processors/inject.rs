//! Bind charge onto board into edge Element; charge becomes bound.

use bytemuck::Zeroable;
use nucleus::{action, board::Element, fees::injection_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use super::common::charge_fee;
use crate::accounts::{AccountIter, FromAccounts, InjectionAccounts};

/// Bind a charge onto the board into an edge Element;
/// charge becomes bound for pressure/overload mechanics.
pub(crate) fn inject<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let InjectionAccounts { charge, dst, board } = InjectionAccounts::extract(it)?;

    dst.coordinates
        .on_edge()
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;
    if !charge.index.is_zero() {
        // charge needs to be out of the board and no outstanding claims
        return Err(ProgramError::Custom(43));
    }

    board.tvl += charge.balance;
    board.charge_count += 1;

    let fee = charge_fee(charge, injection_fee(charge, dst))?;

    let mut src = Element::zeroed();
    action::rebind(charge, &mut src, dst);
    dst.pot += fee;

    Ok(())
}

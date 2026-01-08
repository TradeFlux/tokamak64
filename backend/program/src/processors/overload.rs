use bytemuck::Zeroable;
use nucleus::{
    action::{self, claim},
    board::{Curve, Element},
    consts::MAX_SATURATION,
};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, OverloadAccounts};

/// Forcefully trigger an Element to break and reset, distributing its accumulated pot.
/// Validates saturation threshold, snapshots breaking event in Artefact, advances generation.
pub(crate) fn overload<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let OverloadAccounts {
        charge,
        target,
        artefact,
        board,
    } = OverloadAccounts::extract(it)?;

    // 1. Validate that the move causes capacity overflow
    if target.curve.saturation < MAX_SATURATION {
        return Err(ProgramError::Custom(1)); // Not an overload condition
    }

    // 2. Create artefact: snapshot the state of the overloaded element
    artefact.pot = target.pot;
    artefact.index = target.index;
    claim(charge, artefact);

    // 3. Advance target element to next generation
    // Reset curve to genesis state and clear pot (moved to artefact)
    target.curve = Curve::zeroed();
    // TODO recompute curve capacity from the board
    target.pot = 0;

    // Increment generation in the element index
    target.index.advance_generation();
    let mut src = Element::zeroed();
    action::rebind(charge, &mut src, target);

    board.tvl -= target.pot + target.curve.tvl;

    Ok(())
}

use bytemuck::Zeroable;
use nucleus::{
    action::{self, claim},
    board::{Curve, Element},
    consts::MAX_SATURATION,
};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, OverloadAccounts};

/// Process an Overload instruction: trigger a reset on an element when saturation exceeds threshold.
/// This occurs atomically during a charge rebind operation.
///
/// Preconditions:
/// - charge has been moved to target and saturation exceeds LUT_X_MAX
/// - artefact account is initialized to store the reset snapshot
///
/// Steps:
/// 1. Validate that saturation exceeds overload threshold
/// 2. Create artefact snapshot of the overloaded element
/// 3. Advance target element to next generation
/// 4. Rebind charge to the newly created target element
/// 5. Update board stats
pub(crate) fn process_overload<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let OverloadAccounts {
        charge,
        target,
        artefact,
        board,
    } = OverloadAccounts::parse(it)?;

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
    target.index.next_gen();
    let mut src = Element::zeroed();
    action::rebind(charge, &mut src, target);

    board.tvl -= target.pot + target.curve.tvl;

    Ok(())
}

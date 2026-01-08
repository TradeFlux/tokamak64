use bytemuck::Zeroable;
use curve::lut::{LUT_X_MAX, LUT_X_MIN};
use nucleus::{
    action::{self, claim},
    board::{Curve, Element},
};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, OverloadAccounts};

/// Process an Overload instruction: trigger a reset on an element when capacity is exceeded.
/// This occurs atomically during a charge move operation.
///
/// Preconditions:
/// - charge has been moved to src and would cause overflow
/// - dst is the next generation element (new capacity)
/// - tomb account is initialized for storing the snapshot
///
/// Steps:
/// 1. Validate that the move causes capacity overflow
/// 2. Create tombstone snapshot of src element
/// 3. Advance src element to next generation
/// 4. Bind charge to the newly created dst element
/// 5. Update board stats
pub(crate) fn process_overload<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let OverloadAccounts {
        charge,
        target,
        artefact,
        board,
    } = OverloadAccounts::parse(it)?;

    // 1. Validate that the move causes capacity overflow
    if target.curve.position < LUT_X_MAX {
        return Err(ProgramError::Custom(1)); // Not an overload condition
    }

    // 2. Create tombstone: snapshot the state of the overloaded element
    artefact.pot = target.pot;
    artefact.index = target.index;
    claim(charge, artefact);

    // 3. Advance src element to next generation
    // Reset curve to genesis state and clear pot (moved to tombstone)
    target.curve = Curve::zeroed();
    // TODO recompute curve capacity from the board
    target.pot = 0;

    // Increment generation in the element index
    target.index.nextgen();
    let mut src = Element::zeroed();
    action::translate(charge, &mut src, target);

    board.tvl -= target.pot + target.curve.volume;

    Ok(())
}

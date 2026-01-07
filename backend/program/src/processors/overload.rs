use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, OverloadAccounts};

/// Process an Overload instruction: trigger a reset on an element when capacity is exceeded.
/// This transitions the element to a tombstone and distributes the pot to shareholders.
pub(crate) fn process_overload<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let OverloadAccounts { src, dst, board, .. } = OverloadAccounts::parse(it)?;

    // TODO: Validate that the element has actually overloaded (pressure > capacity)
    // TODO: Implement tombstone creation and pot distribution logic
    // TODO: Reset the element's curve and transition shareholders to claim phase

    // Placeholder: move pot to destination and clear source
    dst.pot = dst.pot.checked_add(src.pot)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    src.pot = 0;

    // Update board state
    board.charges = board.charges.saturating_sub(1);

    Ok(())
}

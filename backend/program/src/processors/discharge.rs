use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, DischargeAccounts, FromAccounts};

/// Process a Discharge instruction: withdraw GLUON from the system back to stable tokens.
/// This decreases the player's balance and the board's TVL.
pub(crate) fn process_discharge<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let DischargeAccounts {
        charge,
        wallet,
        board,
    } = DischargeAccounts::parse(it)?;

    // TODO: Parse amount from instruction_data
    let amount = 0u64;

    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // Check sufficient balance
    if charge.index.atomic_number() != 0 {
        // TODO: proper error handling - can only withdraw when not bound to an element
        return Err(ProgramError::Custom(50));
    }

    // Transfer from charge balance back to wallet
    charge.balance = charge
        .balance
        .checked_sub(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    wallet.balance = wallet
        .balance
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Update board TVL
    board.tvl = board
        .tvl
        .checked_sub(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(())
}

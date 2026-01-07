use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, ChargeAccounts, FromAccounts};

/// Process a Charge instruction: deposit stable tokens into the system.
/// This increases the player's liquid balance and the board's TVL.
pub(crate) fn process_charge<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let ChargeAccounts {
        charge,
        wallet,
        board,
    } = ChargeAccounts::parse(it)?;

    // TODO: Parse amount from instruction_data
    let amount = 0u64;

    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // Transfer from wallet to charge balance
    wallet.balance = wallet
        .balance
        .checked_sub(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    charge.balance = charge
        .balance
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Update board TVL
    board.tvl = board
        .tvl
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(())
}

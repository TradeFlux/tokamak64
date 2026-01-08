use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::{
    accounts::{AccountIter, ChargeAccounts, FromAccounts},
    instruction::IxData,
};

/// Process a Charge instruction: deposit stable tokens into the system.
/// This increases the player's liquid balance and the board's TVL.
pub(crate) fn process_charge<'a, I>(it: &mut I, mut data: IxData) -> ProgramResult
where
    I: AccountIter<'a>,
{
    let ChargeAccounts { charge, wallet } = ChargeAccounts::parse(it)?;

    let amount = data.read()?;

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

    Ok(())
}

//! Release a charge from element binding and reset its state.

use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::{
    accounts::{AccountIter, DischargeAccounts, FromAccounts},
    instruction::IxData,
};

/// Merge a charge's remaining Gluon back into the wallet account; unbound charges only.
pub(crate) fn discharge<'a, I>(it: &mut I, mut data: IxData) -> ProgramResult
where
    I: AccountIter<'a>,
{
    let DischargeAccounts { charge, wallet } = DischargeAccounts::extract(it)?;

    let amount = data.read()?;

    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // Charge must be unbound (off-board)
    if charge.index.atomic_number() != 0 {
        // TODO: proper error handling - can only withdraw when not bound to an element
        return Err(ProgramError::Custom(50));
    }

    // Check sufficient balance
    if charge.balance < amount {
        return Err(ProgramError::InsufficientFunds);
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

    Ok(())
}

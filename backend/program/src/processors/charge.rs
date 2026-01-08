//! Add funds to a charge account from a wallet.

use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::{
    accounts::{AccountIter, ChargeAccounts, FromAccounts},
    instruction::IxData,
};

/// Create a new charge by allocating Gluon from wallet to charge account.
pub(crate) fn charge<'a, I>(it: &mut I, mut data: IxData) -> ProgramResult
where
    I: AccountIter<'a>,
{
    let ChargeAccounts {
        charge,
        wallet,
    } = ChargeAccounts::extract(it)?;

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

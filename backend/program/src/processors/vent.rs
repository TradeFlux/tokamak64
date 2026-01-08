use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::{
    accounts::{AccountIter, FromAccounts, VentAccounts},
    instruction::IxData,
};

pub(crate) fn process_vent<'a, I>(it: &mut I, mut data: IxData) -> ProgramResult
where
    I: AccountIter<'a>,
{
    let VentAccounts { charge, target } = VentAccounts::parse(it)?;

    if charge.index != target.index {
        // TODO proper error on leaking to a wrong target
        return Err(ProgramError::Custom(32));
    }

    let amount = data.read()?;

    let remainder = charge.balance.checked_sub(amount);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    target.pot += amount;
    Ok(())
}

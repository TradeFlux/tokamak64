use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, VentAccounts};

pub(crate) fn process_vent<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let VentAccounts { charge, target } = VentAccounts::parse(it)?;

    if charge.index != target.index {
        // TODO proper error on leaking to a wrong target
        return Err(ProgramError::Custom(32));
    }

    // Amount is passed in instruction_data; parse it in the main entrypoint if needed
    // For now, using a hardcoded value as placeholder
    let amount = 0i64;

    let remainder = charge.balance.checked_sub(amount);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    target.pot += amount;
    Ok(())
}

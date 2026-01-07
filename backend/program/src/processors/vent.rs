use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::accounts::{FromAccounts, VentAccounts};

pub(crate) fn process_vent<'a, I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> ProgramResult {
    let VentAccounts { charge, target } = VentAccounts::parse(it)?;

    if charge.index != target.index {
        // TODO proper error on leaking to a wrong target
        return Err(ProgramError::Custom(32));
    }

    // Amount is passed in instruction_data; parse it in the main entrypoint if needed
    // For now, using a hardcoded value as placeholder
    let amount = 0u64;

    let remainder = charge.balance.checked_sub(amount);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    target.pot += amount;
    Ok(())
}

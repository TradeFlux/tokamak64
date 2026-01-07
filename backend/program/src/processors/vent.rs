use nucleus::types::Gluon;
use pinocchio::{program_error::ProgramError, ProgramResult};

use crate::accounts::VentAccounts;

fn vent(accounts: VentAccounts, amount: Gluon) -> ProgramResult {
    let VentAccounts { charge, target } = accounts;
    if charge.index != target.index {
        // TODO proper error on leaking to a wrong target
        return Err(ProgramError::Custom(32));
    }

    let remainder = charge.balance.checked_sub(amount);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    target.pot += amount;
    Ok(())
}

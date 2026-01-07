use nucleus::action;
use pinocchio::{program_error::ProgramError, ProgramResult};

use crate::accounts::ClaimAccounts;

fn claim(accounts: ClaimAccounts) -> ProgramResult {
    let ClaimAccounts { charge, artefact } = accounts;
    if charge.share == 0 {
        // TODO proper handling of nothing to claim
        return Err(ProgramError::Custom(42));
    }

    if charge.index == artefact.index {
        // TODO proper handling of claim vialotion
        return Err(ProgramError::Custom(42));
    }

    action::claim(charge, artefact);

    Ok(())
}

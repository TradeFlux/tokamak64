use nucleus::action;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::accounts::{ClaimAccounts, FromAccounts};

pub(crate) fn process_claim<'a, I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> ProgramResult {
    let ClaimAccounts { charge, artefact } = ClaimAccounts::parse(it)?;
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

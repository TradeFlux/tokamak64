use nucleus::action;
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, ClaimAccounts, FromAccounts};

/// Collect accumulated rewards from an Element's breaking event based on accumulated share.
pub(crate) fn claim<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let ClaimAccounts { charge, artefact } = ClaimAccounts::extract(it)?;

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

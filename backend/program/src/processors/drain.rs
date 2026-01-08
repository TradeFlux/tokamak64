//! Convert Gluon from wallet back to stable tokens and withdraw.

use nucleus::consts::DECIMALS;
use pinocchio::cpi::Seed;
use pinocchio::ProgramResult;
use pinocchio::{cpi::Signer, error::ProgramError};
use pinocchio_token::instructions::TransferChecked;

use crate::{
    accounts::{AccountIter, DrainAccounts, FromAccounts},
    addresses,
    instruction::IxData,
};

/// Convert Gluon from wallet back to stable tokens (USDT/USDC) and withdraw to ATA.
/// Completes withdrawal cycle, reducing player's in-game balance and system TVL.
pub(crate) fn drain<'a, I>(it: &mut I, mut data: IxData) -> ProgramResult
where
    I: AccountIter<'a>,
{
    let DrainAccounts {
        wallet,
        vault,
        mint,
        dst,
        authority,
    } = DrainAccounts::extract(it)?;

    let amount = data.read()?;

    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // Check sufficient GLUON balance in wallet
    if (wallet.balance as u64) < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    let Some((auth, seeds)) = addresses::resolve_vault(vault.address().as_array()) else {
        return Err(ProgramError::InvalidArgument);
    };
    if auth == *authority.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }
    // Convert 1:1 from GLUON to stable token
    wallet.balance = wallet
        .balance
        .checked_sub(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let seeds = &[Seed::from(seeds[0]), Seed::from(seeds[1])];
    let signer = Signer::from(seeds);

    // Execute token transfer: vault -> dst via CPI
    TransferChecked {
        from: vault,
        mint,
        to: dst,
        authority,
        amount,
        decimals: DECIMALS,
    }
    .invoke_signed(&[signer])?;

    Ok(())
}

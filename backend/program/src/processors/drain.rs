use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;
use pinocchio_token::instructions::TransferChecked;

use crate::accounts::{AccountIter, DrainAccounts, FromAccounts};

/// Process a Drain instruction: convert GLUON in wallet to stable tokens and withdraw.
/// This decreases the player's liquid balance and transfers stable tokens to their ATA.
///
/// Expected accounts in order:
/// 1. wallet - Player's wallet account (writable)
/// 2. vault - Program's vault token ATA (writable, source of stable tokens)
/// 3. mint - Token mint account (USDT/USDC)
/// 4. dst - Player's token ATA (writable, destination)
/// 5. auth - Vault authority/PDA (signer for vault token ATA)
pub(crate) fn process_drain<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let DrainAccounts {
        wallet,
        vault,
        mint,
        dst,
        authority: auth,
    } = DrainAccounts::parse(it)?;

    // TODO: Parse amount from instruction_data
    let amount = 0u64;
    let decimals = 6u8; // Standard for USDT/USDC; TODO: read from mint account

    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // Check sufficient GLUON balance in wallet
    if (wallet.balance as u64) < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // Execute token transfer: vault -> dst via CPI
    // vault is program-owned PDA, so auth (vault authority) needs invoke_signed with seeds
    let transfer = TransferChecked {
        from: vault,
        mint,
        to: dst,
        authority: auth,
        amount,
        decimals,
    };
    // TODO: Pass vault PDA seeds to invoke_signed
    // let seeds = &[b"vault", <mint_bytes>, &[bump_seed]];
    // transfer.invoke_signed(&[seeds])?;
    transfer.invoke()?;

    // Convert 1:1 from GLUON to stable token
    wallet.balance = wallet
        .balance
        .checked_sub(amount as i64)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(())
}

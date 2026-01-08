use nucleus::consts::DECIMALS;
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;
use pinocchio_token::instructions::TransferChecked;

use crate::{
    accounts::{AccountIter, DrainAccounts, FromAccounts},
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

    // Execute token transfer: vault -> dst via CPI
    // vault is program-owned PDA, so auth (vault authority) needs invoke_signed with seeds
    let transfer = TransferChecked {
        from: vault,
        mint,
        to: dst,
        authority,
        amount,
        decimals: DECIMALS,
    };
    // TODO: Pass vault PDA seeds to invoke_signed
    // let seeds = &[b"vault", <mint_bytes>, &[bump_seed]];
    // transfer.invoke_signed(&[seeds])?;
    transfer.invoke()?;

    // Convert 1:1 from GLUON to stable token
    wallet.balance = wallet
        .balance
        .checked_sub(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(())
}

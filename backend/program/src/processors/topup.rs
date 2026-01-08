//! Add funds to a wallet from a token account.

use nucleus::consts::DECIMALS;
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;
use pinocchio_token::instructions::TransferChecked;

use crate::{
    accounts::{AccountIter, FromAccounts, TopUpAccounts},
    instruction::IxData,
};

/// Convert stable tokens (USDT/USDC) to Gluon and deposit into wallet (1:1 conversion).
/// Transfers stable tokens to program vault; wallet balance increases for Charge/Inject actions.
pub(crate) fn topup<'a, I>(it: &mut I, mut data: IxData) -> ProgramResult
where
    I: AccountIter<'a>,
{
    let TopUpAccounts {
        src,
        mint,
        vault,
        authority,
        wallet,
    } = TopUpAccounts::extract(it)?;
    let amount = data.read()?;

    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // Execute token transfer: src -> vault via CPI
    let transfer = TransferChecked {
        from: src,
        mint,
        to: vault,
        authority,
        amount,
        decimals: DECIMALS,
    };
    transfer.invoke()?;

    // Convert 1:1 from stable token to GLUON and deposit into wallet
    wallet.balance = wallet
        .balance
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(())
}

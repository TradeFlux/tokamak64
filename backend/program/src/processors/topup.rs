use nucleus::consts::DECIMALS;
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;
use pinocchio_token::instructions::TransferChecked;

use crate::{
    accounts::{AccountIter, FromAccounts, TopUpAccounts},
    instruction::IxData,
};

/// Process a TopUp instruction: convert stable tokens to GLUON and deposit into wallet.
/// This increases the player's liquid balance and the system's TVL.
///
/// Expected accounts in order:
/// 1. wallet - Player's wallet account (writable)
/// 2. src - Player's token ATA (writable, source of stable tokens)
/// 3. mint - Token mint account (USDT/USDC)
/// 4. vault - Program's vault token ATA (writable, destination)
/// 5. auth - Player's authority (signer, owner of src)
pub(crate) fn process_topup<'a, I>(it: &mut I, mut data: IxData) -> ProgramResult
where
    I: AccountIter<'a>,
{
    let TopUpAccounts {
        wallet,
        src,
        mint,
        vault,
        authority: auth,
    } = TopUpAccounts::parse(it)?;

    let amount = data.read()?;

    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    // Execute token transfer: src -> vault via CPI
    let transfer = TransferChecked {
        from: src,
        mint,
        to: vault,
        authority: auth,
        amount,
        decimals: DECIMALS,
    };
    transfer.invoke()?;

    // TODO wallet might not exist, create if necessary
    // Convert 1:1 from stable token to GLUON and deposit into wallet
    wallet.balance = wallet
        .balance
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    Ok(())
}

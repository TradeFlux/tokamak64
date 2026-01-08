use nucleus::player::Wallet;
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, InitWalletAccounts};

/// Initialize a new wallet account for a player.
/// Sets signer as authority and stores mint reference with zero balance.
pub(crate) fn init_wallet<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let InitWalletAccounts {
        signer,
        wallet,
        mint,
    } = InitWalletAccounts::extract(it)?;

    // TODO actually create the wallet account with allocating enough data
    // use "wallet" + signer.address() + mint.address() as seed
    let wallet: Wallet = todo!();

    wallet.authority.copy_from_slice(mint.address().as_ref());
    wallet.mint.copy_from_slice(signer.address().as_ref());

    Ok(())
}

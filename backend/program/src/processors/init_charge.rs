use nucleus::player::Charge;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, InitChargeAccounts};

/// Initialize a new charge account for a player.
/// Sets signer as authority with counter=0 and zero balance.
pub(crate) fn init_charge<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let InitChargeAccounts {
        signer,
        charge,
        wallet,
    } = InitChargeAccounts::extract(it)?;

    // TODO actually create the charge account with allocating enough data
    // use "charge" + signer.address() + wallet.charge as seed
    let charge: Charge = todo!();

    wallet.charges += 1;

    charge.authority.copy_from_slice(signer.address().as_ref());

    Ok(())
}

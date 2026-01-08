//! Initialize charge and wallet accounts.

use core::{iter, slice};

use nucleus::player::{Charge, Wallet};
use pinocchio::cpi::{Seed, Signer};
use pinocchio::sysvars::rent::Rent;
use pinocchio::sysvars::Sysvar;
use pinocchio::ProgramResult;
use pinocchio_system::instructions::CreateAccount;

use crate::accounts::{parse, AccountIter, FromAccounts, InitChargeAccounts, InitWalletAccounts};
use crate::instruction::IxData;

/// Initialize a new wallet account for a player.
/// Sets signer as authority and stores mint reference with zero balance.
pub(crate) fn wallet<'a, I>(it: &mut I, mut data: IxData) -> ProgramResult
where
    I: AccountIter<'a>,
{
    let InitWalletAccounts {
        signer,
        wallet,
        mint,
    } = InitWalletAccounts::extract(it)?;

    let bump = data.read()?;
    let rent = Rent::get()?;
    let space = size_of::<Wallet>();
    let lamports = rent.try_minimum_balance(space)?;
    let seeds = [
        Seed::from(signer.address().as_ref()),
        Seed::from(mint.address().as_ref()),
        Seed::from(slice::from_ref(&bump)),
    ];
    let invoker = Signer::from(&seeds);
    CreateAccount {
        from: signer,
        to: wallet,
        lamports,
        space: space as u64,
        owner: &crate::ID,
    }
    .invoke_signed(&[invoker])?;

    let wallet: &mut Wallet = parse(&mut iter::once(wallet))?;
    wallet.authority = signer.address().to_bytes();
    wallet.mint = mint.address().to_bytes();

    Ok(())
}

/// Initialize a new charge account for a player.
/// Sets signer as authority with counter=0 and zero balance.
pub(crate) fn charge<'a, I>(it: &mut I, mut data: IxData) -> ProgramResult
where
    I: AccountIter<'a>,
{
    let InitChargeAccounts {
        signer,
        charge,
        wallet,
    } = InitChargeAccounts::extract(it)?;

    let bump = data.read()?;
    let rent = Rent::get()?;
    let space = size_of::<Charge>();
    let lamports = rent.try_minimum_balance(space)?;
    let id = wallet.charges.to_le_bytes();
    let seeds = [
        Seed::from(signer.address().as_ref()),
        Seed::from(&wallet.mint),
        Seed::from(&id),
        Seed::from(slice::from_ref(&bump)),
    ];
    let invoker = Signer::from(&seeds);
    CreateAccount {
        from: signer,
        to: charge,
        lamports,
        space: space as u64,
        owner: &crate::ID,
    }
    .invoke_signed(&[invoker])?;

    let charge: &mut Charge = parse(&mut iter::once(charge))?;
    charge.authority = signer.address().to_bytes();
    charge.mint = wallet.mint;

    wallet.charges += 1;

    Ok(())
}

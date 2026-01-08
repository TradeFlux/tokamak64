use core::slice;
use nucleus::{
    board::{Board, Element, Tombstone},
    player::{Charge, Wallet},
};
use pinocchio::{account::AccountView, error::ProgramError};

/// Trait alias for account iterators used in processor signatures.
pub trait AccountIter<'a>: Iterator<Item = &'a AccountView> {}
impl<'a, I: Iterator<Item = &'a AccountView>> AccountIter<'a> for I {}

pub struct FusionAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) dst: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

pub struct FissionAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

pub struct CompressionAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) dst: &'a mut Element,
}

pub struct OverloadAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) target: &'a mut Element,
    pub(crate) artefact: &'a mut Tombstone,
    pub(crate) board: &'a mut Board,
}

pub struct TranslationAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) dst: &'a mut Element,
}

pub struct VentAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) target: &'a mut Element,
}

pub struct ClaimAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) artefact: &'a mut Tombstone,
}

pub struct ChargeAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) wallet: &'a mut Wallet,
}

pub struct DischargeAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) wallet: &'a mut Wallet,
}

pub struct TopUpAccounts<'a> {
    pub(crate) wallet: &'a mut Wallet,
    pub(crate) src: &'a AccountView,
    pub(crate) mint: &'a AccountView,
    pub(crate) vault: &'a AccountView,
    pub(crate) authority: &'a AccountView,
}

pub struct DrainAccounts<'a> {
    pub(crate) wallet: &'a mut Wallet,
    pub(crate) vault: &'a AccountView,
    pub(crate) mint: &'a AccountView,
    pub(crate) dst: &'a AccountView,
    pub(crate) authority: &'a AccountView,
}

pub trait FromAccounts<'a>: Sized {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError>;
}

impl<'a> FromAccounts<'a> for FusionAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            dst: parse(it)?,
            board: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for FissionAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            src: parse(it)?,
            board: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for CompressionAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            src: parse(it)?,
            dst: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for OverloadAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            target: parse(it)?,
            artefact: parse(it)?,
            board: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for TranslationAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            src: parse(it)?,
            dst: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for VentAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            target: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for ClaimAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            artefact: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for ChargeAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            wallet: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for DischargeAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            wallet: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for TopUpAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            wallet: parse(it)?,
            src: next(it)?,
            mint: next(it)?,
            vault: next(it)?,
            authority: next(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for DrainAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            wallet: parse(it)?,
            vault: next(it)?,
            mint: next(it)?,
            dst: next(it)?,
            authority: next(it)?,
        })
    }
}

fn parse<'a, T, I>(it: &mut I) -> Result<&'a mut T, ProgramError>
where
    T: bytemuck::Pod,
    I: Iterator<Item = &'a AccountView>,
{
    let info = it.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    // Check if account has enough data
    if info.data_len() < size_of::<T>() {
        return Err(ProgramError::InvalidAccountData);
    }
    unsafe {
        let s = slice::from_raw_parts_mut(info.data_ptr() as *mut u8, size_of::<T>());
        bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)
    }
}

fn next<'a, I>(it: &mut I) -> Result<&'a AccountView, ProgramError>
where
    I: Iterator<Item = &'a AccountView>,
{
    it.next().ok_or(ProgramError::NotEnoughAccountKeys)
}

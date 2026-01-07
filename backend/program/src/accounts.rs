use core::slice;
use nucleus::{
    board::{Board, Element, Tombstone},
    player::Charge,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use std::mem::size_of;

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
    pub(crate) src: &'a mut Element,
    pub(crate) dst: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

pub struct DriftAccounts<'a> {
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

pub trait FromAccounts<'a>: Sized {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError>;
}

impl<'a> FromAccounts<'a> for FusionAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let mut it = accounts.iter();
        Ok(Self {
            charge: parse(&mut it)?,
            dst: parse(&mut it)?,
            board: parse(&mut it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for FissionAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let mut it = accounts.iter();
        Ok(Self {
            charge: parse(&mut it)?,
            src: parse(&mut it)?,
            board: parse(&mut it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for CompressionAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let mut it = accounts.iter();
        Ok(Self {
            charge: parse(&mut it)?,
            src: parse(&mut it)?,
            dst: parse(&mut it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for OverloadAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let mut it = accounts.iter();
        Ok(Self {
            charge: parse(&mut it)?,
            src: parse(&mut it)?,
            dst: parse(&mut it)?,
            board: parse(&mut it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for DriftAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let mut it = accounts.iter();
        Ok(Self {
            charge: parse(&mut it)?,
            src: parse(&mut it)?,
            dst: parse(&mut it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for VentAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let mut it = accounts.iter();
        Ok(Self {
            charge: parse(&mut it)?,
            target: parse(&mut it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for ClaimAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let mut it = accounts.iter();
        Ok(Self {
            charge: parse(&mut it)?,
            artefact: parse(&mut it)?,
        })
    }
}

fn parse<'a, T: bytemuck::Pod>(
    it: &mut std::slice::Iter<'a, &'a AccountInfo>,
) -> Result<&'a mut T, ProgramError> {
    let info = *it.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    info.is_writable()
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;
    unsafe {
        let s = slice::from_raw_parts_mut(info.data_ptr(), size_of::<T>());
        bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)
    }
}

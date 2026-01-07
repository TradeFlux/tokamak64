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
    fn parse<I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> Result<Self, ProgramError>;
}

impl<'a> FromAccounts<'a> for FusionAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            dst: parse(it)?,
            board: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for FissionAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            src: parse(it)?,
            board: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for CompressionAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            src: parse(it)?,
            dst: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for OverloadAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            src: parse(it)?,
            dst: parse(it)?,
            board: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for DriftAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            src: parse(it)?,
            dst: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for VentAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            target: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for ClaimAccounts<'a> {
    fn parse<I: Iterator<Item = &'a AccountInfo>>(it: &mut I) -> Result<Self, ProgramError> {
        Ok(Self {
            charge: parse(it)?,
            artefact: parse(it)?,
        })
    }
}

fn parse<'a, T, I>(it: &mut I) -> Result<&'a mut T, ProgramError>
where
    T: bytemuck::Pod,
    I: Iterator<Item = &'a AccountInfo>,
{
    let info = it.next().ok_or(ProgramError::NotEnoughAccountKeys)?;
    info.is_writable()
        .then_some(())
        .ok_or(ProgramError::InvalidArgument)?;
    unsafe {
        let s = slice::from_raw_parts_mut(info.data_ptr(), size_of::<T>());
        bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)
    }
}

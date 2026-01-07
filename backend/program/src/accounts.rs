use nucleus::{
    board::{Board, Element, Tombstone},
    player::Charge,
};
use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use std::mem::size_of;

pub trait FromAccounts<'a>: Sized {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError>;
}

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

impl<'a> FromAccounts<'a> for FusionAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let charge = unsafe {
            let info = *accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Charge>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let dst = unsafe {
            let info = *accounts.get(1).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Element>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let board = unsafe {
            let info = *accounts.get(2).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Board>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        Ok(Self { charge, dst, board })
    }
}

impl<'a> FromAccounts<'a> for FissionAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let charge = unsafe {
            let info = *accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Charge>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let src = unsafe {
            let info = *accounts.get(1).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Element>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let board = unsafe {
            let info = *accounts.get(2).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Board>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        Ok(Self { charge, src, board })
    }
}

impl<'a> FromAccounts<'a> for CompressionAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let charge = unsafe {
            let info = *accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Charge>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let src = unsafe {
            let info = *accounts.get(1).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Element>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let dst = unsafe {
            let info = *accounts.get(2).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Element>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        Ok(Self { charge, src, dst })
    }
}

impl<'a> FromAccounts<'a> for OverloadAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let charge = unsafe {
            let info = *accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Charge>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let src = unsafe {
            let info = *accounts.get(1).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Element>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let dst = unsafe {
            let info = *accounts.get(2).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Element>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let board = unsafe {
            let info = *accounts.get(3).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Board>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        Ok(Self {
            charge,
            src,
            dst,
            board,
        })
    }
}

impl<'a> FromAccounts<'a> for DriftAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let charge = unsafe {
            let info = *accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Charge>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let src = unsafe {
            let info = *accounts.get(1).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Element>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let dst = unsafe {
            let info = *accounts.get(2).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Element>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        Ok(Self { charge, src, dst })
    }
}

impl<'a> FromAccounts<'a> for VentAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let charge = unsafe {
            let info = *accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Charge>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let target = unsafe {
            let info = *accounts.get(1).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Element>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        Ok(Self { charge, target })
    }
}

impl<'a> FromAccounts<'a> for ClaimAccounts<'a> {
    fn parse(accounts: &'a [&AccountInfo]) -> Result<Self, ProgramError> {
        let charge = unsafe {
            let info = *accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Charge>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        let artefact = unsafe {
            let info = *accounts.get(1).ok_or(ProgramError::NotEnoughAccountKeys)?;
            info.is_writable()
                .then_some(())
                .ok_or(ProgramError::InvalidArgument)?;
            let s = core::slice::from_raw_parts_mut(info.data_ptr(), size_of::<Tombstone>());
            bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)?
        };
        Ok(Self { charge, artefact })
    }
}

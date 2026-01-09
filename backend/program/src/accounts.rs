//! Account structures and validation for TOKAMAK64 program instructions.

use core::slice;
use nucleus::{
    board::{Artefact, Board, Element},
    player::{Charge, Wallet},
    types::AddressBytes,
};
use pinocchio::{account::AccountView, error::ProgramError};

/// Trait alias for account iterators used in processor signatures.
pub trait AccountIter<'a>: Iterator<Item = &'a AccountView> {}
impl<'a, I: Iterator<Item = &'a AccountView>> AccountIter<'a> for I {}

// ============================================================================
// ENTRY & EXIT OPERATIONS
// ============================================================================

/// Inject: Bind charge to edge element. Validates: peripheral destination.
pub struct InjectionAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) dst: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

/// Eject: Unbind charge from edge element. Validates: peripheral source, bound charge.
pub struct EjectionAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

// ============================================================================
// MOVEMENT & VALUE TRANSFER
// ============================================================================

/// Compress: Move pot inward, rebind charge. Validates: adjacent elements, deeper destination.
pub struct CompressionAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) dst: &'a mut Element,
}

/// Rebind: Move charge to adjacent element. Validates: adjacent, charge bound.
pub struct RebindAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) dst: &'a mut Element,
}

/// Vent: Donate charge value to element pot. Validates: charge in element, sufficient balance.
pub struct VentAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) target: &'a mut Element,
}

// ============================================================================
// BREAKING & REWARDS
// ============================================================================

/// Overload: Trigger element reset. Validates: saturation exceeds threshold.
pub struct OverloadAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) target: &'a mut Element,
    pub(crate) artefact: &'a mut Artefact,
    pub(crate) board: &'a mut Board,
}

/// Claim: Collect reward share from overload event. Validates: generation matches.
pub struct ClaimAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) artefact: &'a mut Artefact,
}

// ============================================================================
// WALLET & BALANCE MANAGEMENT
// ============================================================================

/// Charge: Allocate Gluon from wallet to charge. Validates: sufficient balance.
pub struct ChargeAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) wallet: &'a mut Wallet,
}

/// Discharge: Merge charge back to wallet. Validates: charge unbound.
pub struct DischargeAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) wallet: &'a mut Wallet,
}

/// Infusion: Convert stable tokens to Gluon (1:1). Validates: token transfer.
pub struct InfusionAccounts<'a> {
    pub(crate) authority: &'a AccountView,
    pub(crate) wallet: &'a mut Wallet,
    pub(crate) src: &'a AccountView,
    pub(crate) mint: &'a AccountView,
    pub(crate) vault: &'a AccountView,
}

/// Extraction: Convert Gluon to stable tokens (1:1). Validates: sufficient balance.
pub struct ExtractionAccounts<'a> {
    pub(crate) wallet: &'a mut Wallet,
    pub(crate) vault: &'a AccountView,
    pub(crate) mint: &'a AccountView,
    pub(crate) dst: &'a AccountView,
    pub(crate) authority: &'a AccountView,
}

// ============================================================================
// ACCOUNT INITIALIZATION
// ============================================================================

/// InitCharge: Create new charge PDA. Validates: wallet authority.
pub struct InitChargeAccounts<'a> {
    pub(crate) signer: &'a AccountView,
    pub(crate) wallet: &'a mut Wallet,
    pub(crate) charge: &'a AccountView,
}

/// InitWallet: Create new wallet PDA. Validates: signer authority.
pub struct InitWalletAccounts<'a> {
    pub(crate) signer: &'a AccountView,
    pub(crate) wallet: &'a AccountView,
    pub(crate) mint: &'a AccountView,
}

// ============================================================================
// HELPERS & IMPLS
// ============================================================================

pub trait FromAccounts<'a>: Sized {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError>;
}

impl<'a> FromAccounts<'a> for InjectionAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let charge: &'a mut Charge = parse(it)?;
        authorize(signer, &charge.authority)?;
        Ok(Self {
            charge,
            dst: parse(it)?,
            board: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for EjectionAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let charge: &'a mut Charge = parse(it)?;
        authorize(signer, &charge.authority)?;
        Ok(Self {
            charge,
            src: parse(it)?,
            board: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for CompressionAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let charge: &'a mut Charge = parse(it)?;
        authorize(signer, &charge.authority)?;
        Ok(Self {
            charge,
            src: parse(it)?,
            dst: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for OverloadAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let charge: &'a mut Charge = parse(it)?;
        authorize(signer, &charge.authority)?;
        Ok(Self {
            charge,
            target: parse(it)?,
            artefact: parse(it)?,
            board: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for RebindAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let charge: &'a mut Charge = parse(it)?;
        authorize(signer, &charge.authority)?;
        Ok(Self {
            charge,
            src: parse(it)?,
            dst: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for VentAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let charge: &'a mut Charge = parse(it)?;
        authorize(signer, &charge.authority)?;
        Ok(Self {
            charge,
            target: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for ClaimAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let charge: &'a mut Charge = parse(it)?;
        authorize(signer, &charge.authority)?;
        Ok(Self {
            charge,
            artefact: parse(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for ChargeAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let charge: &'a mut Charge = parse(it)?;
        let wallet: &'a mut Wallet = parse(it)?;
        authorize(signer, &wallet.authority)?;
        Ok(Self { charge, wallet })
    }
}

impl<'a> FromAccounts<'a> for DischargeAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let charge: &'a mut Charge = parse(it)?;
        let wallet: &'a mut Wallet = parse(it)?;
        authorize(signer, &wallet.authority)?;
        Ok(Self { charge, wallet })
    }
}

impl<'a> FromAccounts<'a> for InfusionAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let authority = next(it)?;
        let wallet = parse::<Wallet, _>(it)?;
        authorize(authority, &wallet.authority)?;
        Ok(Self {
            authority,
            wallet,
            src: next(it)?,
            mint: next(it)?,
            vault: next(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for ExtractionAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let wallet = parse::<Wallet, _>(it)?;
        authorize(signer, &wallet.authority)?;
        Ok(Self {
            wallet,
            vault: next(it)?,
            mint: next(it)?,
            dst: next(it)?,
            authority: next(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for InitChargeAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        let wallet = parse::<Wallet, _>(it)?;
        authorize(signer, &wallet.authority)?;
        Ok(Self {
            signer,
            wallet,
            charge: next(it)?,
        })
    }
}

impl<'a> FromAccounts<'a> for InitWalletAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
        let signer = next(it)?;
        if !signer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature)?;
        }

        Ok(Self {
            signer,
            wallet: next(it)?,
            mint: next(it)?,
        })
    }
}

pub(crate) fn parse<'a, T, I>(it: &mut I) -> Result<&'a mut T, ProgramError>
where
    T: bytemuck::Pod,
    I: Iterator<Item = &'a AccountView>,
{
    let info = next(it)?;
    // Ensure account data is large enough before attempting cast
    if info.data_len() < size_of::<T>() {
        return Err(ProgramError::InvalidAccountData);
    }
    let s = unsafe { slice::from_raw_parts_mut(info.data_ptr(), size_of::<T>()) };
    bytemuck::try_from_bytes_mut(s).map_err(|_| ProgramError::InvalidAccountData)
}

fn next<'a, I>(it: &mut I) -> Result<&'a AccountView, ProgramError>
where
    I: Iterator<Item = &'a AccountView>,
{
    it.next().ok_or(ProgramError::NotEnoughAccountKeys)
}

pub(crate) fn authorize(
    signer: &AccountView,
    authority: &AddressBytes,
) -> Result<(), ProgramError> {
    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if signer.address().as_ref() == authority {
        Ok(())
    } else {
        Err(ProgramError::IncorrectAuthority)
    }
}

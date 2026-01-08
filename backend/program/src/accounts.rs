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

/// Accounts for Fuse: bind a charge to a destination edge Element.
pub struct FusionAccounts<'a> {
    /// Charge account to bind to board.
    pub(crate) charge: &'a mut Charge,
    /// Destination Element (must be on perimeter).
    pub(crate) dst: &'a mut Element,
    /// Board singleton for TVL tracking.
    pub(crate) board: &'a mut Board,
}

/// Accounts for Fiss: unbind a charge from its source Element and exit the board.
pub struct FissionAccounts<'a> {
    /// Charge account to unbind from board.
    pub(crate) charge: &'a mut Charge,
    /// Source Element (must be on perimeter).
    pub(crate) src: &'a mut Element,
    /// Board singleton for TVL tracking.
    pub(crate) board: &'a mut Board,
}

/// Accounts for Compress: move Element's pot inward and rebind charge to deeper destination.
pub struct CompressionAccounts<'a> {
    /// Charge account being rebind.
    pub(crate) charge: &'a mut Charge,
    /// Source Element (pot source).
    pub(crate) src: &'a mut Element,
    /// Destination Element (deeper; pot destination).
    pub(crate) dst: &'a mut Element,
}

/// Accounts for Overload: trigger Element reset and snapshot breaking event.
pub struct OverloadAccounts<'a> {
    /// Charge causing/triggering the overload.
    pub(crate) charge: &'a mut Charge,
    /// Element to reset (saturation must exceed threshold).
    pub(crate) target: &'a mut Element,
    /// Artefact account to snapshot breaking event and pot.
    pub(crate) artefact: &'a mut Artefact,
    /// Board singleton for TVL tracking.
    pub(crate) board: &'a mut Board,
}

/// Accounts for Rebind: move bound charge from source Element to adjacent destination.
pub struct RebindAccounts<'a> {
    /// Charge account being moved.
    pub(crate) charge: &'a mut Charge,
    /// Source Element (current location).
    pub(crate) src: &'a mut Element,
    /// Destination Element (must be adjacent).
    pub(crate) dst: &'a mut Element,
}

/// Accounts for Vent: donate charge value to its current Element's shared pot.
pub struct VentAccounts<'a> {
    /// Charge account being vented.
    pub(crate) charge: &'a mut Charge,
    /// Element receiving the vented value.
    pub(crate) target: &'a mut Element,
}

/// Accounts for Claim: collect reward share from Element's breaking event.
pub struct ClaimAccounts<'a> {
    /// Charge account claiming rewards.
    pub(crate) charge: &'a mut Charge,
    /// Artefact storing the breaking event snapshot.
    pub(crate) artefact: &'a mut Artefact,
}

/// Accounts for Charge: create new charge by allocating Gluon from wallet.
pub struct ChargeAccounts<'a> {
    /// New charge account to fund.
    pub(crate) charge: &'a mut Charge,
    /// Wallet account to withdraw from.
    pub(crate) wallet: &'a mut Wallet,
}

/// Accounts for Discharge: merge charge's Gluon back into wallet.
pub struct DischargeAccounts<'a> {
    /// Charge account to merge from.
    pub(crate) charge: &'a mut Charge,
    /// Wallet account to deposit to.
    pub(crate) wallet: &'a mut Wallet,
}

/// Accounts for TopUp: convert stable tokens (USDT/USDC) to Gluon in wallet.
pub struct TopUpAccounts<'a> {
    /// Wallet account to deposit Gluon into.
    pub(crate) wallet: &'a mut Wallet,
    /// Source ATA (player's token account).
    pub(crate) src: &'a AccountView,
    /// Token mint (USDT/USDC).
    pub(crate) mint: &'a AccountView,
    /// Program's token vault ATA.
    pub(crate) vault: &'a AccountView,
    /// Player's authority (signer).
    pub(crate) authority: &'a AccountView,
}

/// Accounts for Drain: convert wallet Gluon back to stable tokens in ATA.
pub struct DrainAccounts<'a> {
    /// Wallet account to withdraw Gluon from.
    pub(crate) wallet: &'a mut Wallet,
    /// Program's token vault ATA (source).
    pub(crate) vault: &'a AccountView,
    /// Token mint (USDT/USDC).
    pub(crate) mint: &'a AccountView,
    /// Destination ATA (player's token account).
    pub(crate) dst: &'a AccountView,
    /// Vault authority/PDA (signer).
    pub(crate) authority: &'a AccountView,
}

/// Accounts for InitCharge: initialize a new charge account.
pub struct InitChargeAccounts<'a> {
    /// Signer (becomes charge authority, pays rent).
    pub(crate) signer: &'a AccountView,
    /// Wallet account (signer's wallet).
    pub(crate) wallet: &'a mut Wallet,
    /// Charge account to initialize (PDA from signer + counter).
    pub(crate) charge: &'a AccountView,
}

/// Accounts for InitWallet: initialize a new wallet account.
pub struct InitWalletAccounts<'a> {
    /// Signer (becomes wallet authority, pays rent).
    pub(crate) signer: &'a AccountView,
    /// Wallet account to initialize (PDA from signer + mint).
    pub(crate) wallet: &'a AccountView,
    /// Mint account (stable token mint: USDT/USDC).
    pub(crate) mint: &'a AccountView,
}

pub trait FromAccounts<'a>: Sized {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError>;
}

impl<'a> FromAccounts<'a> for FusionAccounts<'a> {
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

impl<'a> FromAccounts<'a> for FissionAccounts<'a> {
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

impl<'a> FromAccounts<'a> for TopUpAccounts<'a> {
    fn extract<I: Iterator<Item = &'a AccountView>>(it: &mut I) -> Result<Self, ProgramError> {
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

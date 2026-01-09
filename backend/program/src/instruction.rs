//! Instruction definitions and discriminators for TOKAMAK64 game actions.

use core::mem;

use bytemuck::Pod;
use pinocchio::error::ProgramError;

const IX_COUNT: u64 = 13;

pub(crate) struct IxData<'a> {
    inner: &'a [u8],
    cursor: usize,
}

#[allow(dead_code)]
#[repr(u64)]
#[derive(Clone, Copy)]
pub enum TokamakInstruction {
    /// Initialize a new charge account (PDA) for a player.
    InitCharge,
    /// Initialize a new wallet account (PDA) for a player.
    InitWallet,
    /// Create a new charge by allocating Gluon from wallet to a charge account.
    Charge,
    /// Collect accumulated rewards from an Element after it breaks.
    Claim,
    /// Move Element's pot inward to a deeper adjacent Element while rebinding the charge.
    Compress,
    /// Convert Gluon from wallet back to stable tokens (USDT/USDC).
    Extract,
    /// Merge a charge's remaining Gluon back into the wallet account.
    Discharge,
    /// Move a bound charge from one Element to an adjacent Element.
    Rebind,
    /// Unbind a charge from its current Element and move it outside the board.
    Unbind,
    /// Bind a charge onto the board into an edge Element (perimeter only).
    Bind,
    /// Forcefully trigger an Element to overload and reset, distributing its accumulated pot.
    Overload,
    /// Convert stable tokens to Gluon and add to wallet (1:1 conversion).
    Infuse,
    /// Donate part of a bound charge's value to its current Element's shared pot.
    Vent = IX_COUNT - 1,
}

impl TokamakInstruction {
    pub(crate) fn parse(data: &mut IxData) -> Result<Self, ProgramError> {
        let discriminator: u64 = data.read()?;
        if let 0..IX_COUNT = discriminator {
            // # SAFETY
            // The bit pattern is valid for the enum due to range inclusion
            return Ok(unsafe { mem::transmute(discriminator) });
        }
        Err(ProgramError::InvalidInstructionData)
    }
}

impl<'a> IxData<'a> {
    pub(crate) fn new(inner: &'a [u8]) -> Self {
        let cursor = 0;
        Self { inner, cursor }
    }

    pub(crate) fn read<T: Pod>(&mut self) -> Result<T, ProgramError> {
        let end = self.cursor + size_of::<T>();
        let val = *self
            .inner
            .get(self.cursor..end)
            .and_then(|s| bytemuck::try_from_bytes(s).ok())
            .ok_or(ProgramError::InvalidInstructionData)?;
        self.cursor = end;
        Ok(val)
    }
}

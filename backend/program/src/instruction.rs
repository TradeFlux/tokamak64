use core::mem;

use bytemuck::Pod;
use pinocchio::error::ProgramError;

const IX_COUNT: u64 = 11;

pub(crate) struct IxData<'a> {
    inner: &'a [u8],
    cursor: usize,
}

// NOTE: we do construct all of the enum variants, via transmutation
#[allow(dead_code)]
#[repr(u64)]
#[derive(Clone, Copy)]
pub enum TokamakInstruction {
    /// Create a new charge by allocating Gluon from wallet to a charge account.
    /// Reads: amount (u64).
    Charge,
    /// Collect accumulated rewards from an Element after it breaks.
    /// Distributes share proportional to the player's value that was present during accumulation.
    Claim,
    /// Move Element's pot inward to a deeper adjacent Element while rebinding the charge.
    /// Consolidates value deeper into the board; fees added to the moving pot.
    Compress,
    /// Convert Gluon from wallet back to stable tokens (USDT/USDC).
    /// Reads: amount (u64). Completes withdrawal cycle.
    Drain,
    /// Merge a charge's remaining Gluon back into the wallet account.
    /// Reads: amount (u64). Unbinds funds from board play.
    Discharge,
    /// Move a bound charge from one Element to an adjacent Element.
    /// Incurs movement costs scaled by destination depth and speed tax.
    Rebind,
    /// Unbind a charge from its current Element and move it outside the board.
    /// Only possible from edge Elements; applies exit cost.
    Fiss,
    /// Bind a charge onto the board into an edge Element (perimeter only).
    /// Charge becomes bound and participates in Element pressure and breaking.
    Fuse,
    /// Forcefully trigger an Element to break and reset, distributing its accumulated pot.
    /// Creates an Artefact snapshot recording breaking event.
    Overload,
    /// Convert stable tokens to Gluon and add to wallet (1:1 conversion).
    /// Reads: amount (u64). Deposits Gluon into account for Charge/Fuse actions.
    TopUp,
    /// Donate part of a bound charge's value to its current Element's shared pot.
    /// Reduces player share; accelerates Element's breaking point.
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

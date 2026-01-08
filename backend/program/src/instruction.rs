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
    /// Deposit stable tokens into system, funding the wallet
    Charge,
    /// Claim the shares worth gluon from the pot of previously reset element
    Claim,
    /// Move the pot inward merging with others
    Compress,
    /// Convert gluon in wallet to correponding stable token
    Drain,
    /// Withdraw the funds from wallet, by converting GLUON to original stable token
    Discharge,
    /// Move the charge from one element to another
    Translate,
    /// Exit the board
    Fiss,
    /// Enter the board
    Fuse,
    /// Push the element beyond curve capacity, triggering a reset
    Overload,
    /// Convert some stable tokens to Gluon and put them in wallet
    TopUp,
    /// Donate some of the charge's balance to the pot of the current element
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

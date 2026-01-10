//! Test constants.

use super::prelude::*;
use nucleus::types::ElementIndex;

pub const PROGRAM_ID: Pubkey = tokamak_program::ID;

/// System program ID (all zeros, base58: 11111111111111111111111111111111)
pub const SYSTEM_PROGRAM_ID: Pubkey = Pubkey::new_from_array([0u8; 32]);

pub const EDGE_COORD: u64 = 0x01;
pub const INTERIOR_COORD: u64 = 0x0000_0400_0000_0000;

/// Zero element index for unbound charges
pub const ZERO_INDEX: ElementIndex = ElementIndex(0);

// ----------------------------------------------------------------------------
// Test Value Constants
// ----------------------------------------------------------------------------

/// Standard balance amounts
pub const BAL_MIN: u64 = 1_000_000;
pub const BAL_HIGH: u64 = 10_000_000;
pub const BAL_MAX: u64 = 100_000_000;

/// Common operation amounts
pub const AMT_HALF: u64 = 500_000;
pub const AMT_QUARTER: u64 = 250_000;

/// Share values (fixed-point 24.8 format)
pub const SHARE_ONE: u32 = 1 << 24;
pub const SHARE_TWO: u32 = 2 << 24;
pub const SHARE_FOUR: u32 = 4 << 24;

/// Account lamports (standard for test accounts)
pub const LAMPORTS: u64 = 1_000_000;
pub const SIGNER_LAMPORTS: u64 = 1_000_000_000;

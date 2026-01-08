use bytemuck::{Pod, Zeroable};

use crate::types::{AddressBytes, ElementIndex, Gluon, Q824};

/// Wallet: player's liquid (unallocated) balance and identity.
/// Entry point for deposits/withdrawals. Funds here incur no pressure.
/// Field order: 8+32+32 = 72 bytes (no padding).
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct Wallet {
    /// Liquid Gluon (not bound to any element).
    pub balance: Gluon,
    /// Authority pubkey (signer).
    pub authority: AddressBytes,
    /// Mint account for token transfers.
    pub mint: AddressBytes,
}

/// Charge: atomic allocation of a player's funds bound to one element.
/// Multiple charges per player allowed (across different elements).
/// Field order: 8+8+8+4+4+32 = 64 bytes (no padding).
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct Charge {
    /// Gluon allocated to this charge.
    pub balance: Gluon,
    /// Timestamp of last action (for speed tax calculations).
    pub timestamp: u64,
    /// Which element this charge is bound to (cleared on unbind).
    pub index: ElementIndex,
    /// Share of future pressure payouts (Q8.24, set on bind, cleared on unbind).
    pub share: Q824,
    pub _pad: u32,
    /// Authority pubkey (owner of this charge).
    pub authority: AddressBytes,
}

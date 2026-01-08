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
    /// Timestamp of last action (for speed tax and cost calculations).
    pub timestamp: u64,
    /// Which element this charge is bound to (0 if unbound/outside board).
    pub index: ElementIndex,
    /// Share of Element's pot as Q8.24 fixed-point (proportional reward at breaking).
    pub share: Q824,
    /// Padding for 64-byte alignment (Pod requirement).
    pub _pad: u32,
    /// Public key of the charge's owner.
    pub authority: AddressBytes,
}

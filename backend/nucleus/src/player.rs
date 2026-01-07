use bytemuck::{Pod, Zeroable};

use crate::types::{AddressBytes, ElementIndex, Gluon, Q824};

/// Wallet tracks a player's liquid (unallocated) balance and ownership.
///
/// The wallet is the entry point for deposits and withdrawals.
/// Every player has exactly one wallet. Funds in a wallet are not yet
/// bound to any element and incur no pressure.
///
/// Field order optimized to eliminate padding:
/// - Gluon (u64, 8 bytes, align 8): balance
/// - AddressBytes ([u8; 32], 32 bytes, align 1): authority
/// - AddressBytes ([u8; 32], 32 bytes, align 1): mint
/// Total: 72 bytes (8+32+32), perfectly aligned
///
/// Ref: TOKAMAK64 Part 3 (How Players Get In)
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct Wallet {
    /// Liquid balance in Gluon (not bound to any element).
    pub balance: Gluon,
    /// Player's authority (signer pubkey).
    pub authority: AddressBytes,
    /// Mint account associated with this wallet (for token transfers).
    pub mint: AddressBytes,
}

/// Charge is an atomic allocation of a player's funds bound to an element.
///
/// When a player enters an element, they create a Charge. The charge tracks:
/// - how much Gluon they have allocated to this element,
/// - their share of future pressure-driven payouts,
/// - and the timing of their actions (for speed tax calculations).
///
/// Multiple charges may exist for the same player across different elements.
/// Each charge is independent and can be managed separately.
///
/// Field order optimized to eliminate padding:
/// - u64 (8 bytes, align 8): balance
/// - u64 (8 bytes, align 8): timestamp
/// - ElementIndex (u64, 8 bytes, align 8): index
/// - Q824 (i32, 4 bytes, align 4): share
/// - u32 padding (4 bytes) to reach 32-byte boundary
/// - AddressBytes ([u8; 32], 32 bytes, align 1): authority
/// Total: 92 bytes (8+8+8+4+4+32), no padding gaps
///
/// Ref: TOKAMAK64 Part 3 (How Players Get In) and Part 5 (Pressure & Sharing)
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct Charge {
    /// Total Gluon allocated to this charge.
    pub balance: Gluon,
    /// Timestamp of the last action (blocks/slots). Used for speed tax.
    pub timestamp: u64,
    /// Which element this charge is bound to. Cleared (0) when unbind.
    pub index: ElementIndex,
    /// Share of future payouts (Q824 fixed-point). Set on entry, cleared on exit.
    pub share: Q824,
    _padding: u32,
    /// Public key of the owning wallet (for authority checks).
    pub authority: AddressBytes,
}

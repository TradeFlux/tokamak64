use bytemuck::AnyBitPattern;

use crate::types::{AddressBytes, ElementIndex, Gluon, Q1616};

/// Total player deposit which is not allocated to charges yet,
/// an entrypoint for deposits and withdrawals
#[repr(C)]
#[derive(AnyBitPattern, Clone, Copy)]
struct Wallet {
    pub balance: Gluon,
    pub authority: AddressBytes,
    pub mint: AddressBytes,
}

/// Atomic allocation of user funds which can be moved around the board, and also
/// contains information for redeeming the claims to the pot in case of element overload
#[repr(C)]
#[derive(AnyBitPattern, Clone, Copy)]
struct Charge {
    pub index: ElementIndex,
    /// Total volume of GLUON, the player has in charge
    pub balance: Gluon,
    /// Timestamp (in some discrete blockchain measure) when the player performed the last action
    pub timestamps: u64,
    /// Total share of the rewards the player is currently entitled to (determined by the curve)
    pub share: Q1616,
    /// Pubkey of the ownning wallet
    pub authority: AddressBytes,
}

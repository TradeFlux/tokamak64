//! Player accounts: wallets for liquid funds and charges for allocated funds bound to elements.

use bytemuck::{Pod, Zeroable};

use crate::types::{AddressBytes, ElementIndex, Gluon, Q824};

/// Liquid Gluon (outside board pressure). Entry/exit point for on-chain value via Infuse/Extract.
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct Wallet {
    /// Unallocated Gluon, ready to fund charges.
    pub balance: Gluon,
    /// Wallet authority (signer).
    pub authority: AddressBytes,
    /// Stable token mint (USDT/USDC).
    pub mint: AddressBytes,
    /// Count of charges created (for PDA derivation).
    pub charges: u32,
    _pad: u32,
}

/// Allocated Gluon bound to one element. Bound (index != 0) or unbound (index == 0).
/// Reward share calculated at entry, claimed after element breaks.
#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, Debug)]
pub struct Charge {
    /// Allocated Gluon (on board or awaiting move/exit).
    pub balance: Gluon,
    /// Last action timestamp (for speed tax calculation).
    pub timestamp: u64,
    /// Bound element: atomic number + generation (0 = unbound).
    pub index: ElementIndex,
    /// Proportional share of element pot (Q8.24 fixed-point).
    pub share: Q824,
    /// Charge authority (signer).
    pub authority: AddressBytes,
    /// Stable token mint.
    pub mint: AddressBytes,
    _pad: u32,
}

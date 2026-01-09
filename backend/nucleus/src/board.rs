//! Board state: elements, curves, pots, and global artefact snapshots.

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

use crate::types::{Coordinates, ElementIndex, Gluon, Q1648, Q824};

/// Curve: bonding curve state for an element.
/// Determines entry cost, player share value, and pressure mechanics.
/// Field order: 8+8+4+4 = 24 bytes (Pod-aligned).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
pub struct Curve {
    /// Maximum Gluon this curve can accumulate.
    pub capacity: Gluon,
    /// Total Gluon ever accumulated (TVL, net of deposits/withdrawals).
    pub tvl: Gluon,
    /// Accumulated pressure integral as Q16.48; path-independent checksum of history.
    pub pressure: Q1648,
    /// Current saturation as Q8.24 (0 to 6); higher = crowded, higher costs.
    pub saturation: Q824,
    /// Cumulative number of shares owned by bound charges (can differ from saturation)
    pub shares: Q824,
}

/// Element: single board group where players gather and accumulate pressure.
/// Resets (overloads) when pressure exceeds a threshold.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
pub struct Element {
    /// Shared pot (rewards, cost contributions).
    pub pot: Gluon,
    /// Static identity + generation counter.
    pub index: ElementIndex,
    /// Bonding curve state.
    pub curve: Curve,
    /// Bitboard: which squares this element occupies.
    pub coordinates: Coordinates,
}

/// Board: global singleton tracking game-wide state.
/// Field order: 8+8+4+1+3 = 24 bytes (Pod-aligned).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
pub struct Board {
    /// Total Gluon locked across all active charges.
    pub tvl: Gluon,
    /// Gluon in quantum pocket reserve (reserved for rare unlocks/events).
    pub quantum_pocket: Gluon,
    /// Count of active charges on board (used in global pressure calculations).
    pub charge_count: u32,
    /// Quantum unlock progression index (tracks fully-reset elements).
    pub quantum_index: u8,
    /// Padding for 24-byte alignment (Pod requirement).
    pub _pad: [u8; 3],
}

/// Artefact: snapshot of a reset element. Players can claim their share but cannot re-enter.
/// Created when an element resets (saturation exceeds threshold).
/// Field order: 8+8 = 16 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
pub struct Artefact {
    /// Remaining pot to distribute to shareholders.
    pub pot: Gluon,
    /// Which element this was (reference only).
    pub index: ElementIndex,
    /// Cumulative number of shares owned by bound charges at the moment of overload
    pub shares: Q824,
    _pad: u32,
}

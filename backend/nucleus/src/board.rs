use bytemuck::{Pod, Zeroable};

use crate::types::{Coordinates, ElementIndex, Gluon, Q1648, Q824};

/// Curve: bonding curve state for an element.
/// Determines entry cost, player share value, and pressure mechanics.
/// Field order: 8+8+4+4 = 24 bytes (Pod-aligned).
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Curve {
    /// Maximum Gluon this curve can accumulate.
    pub capacity: Gluon,
    /// Total Gluon ever accumulated (TVL, net of deposits/withdrawals).
    pub tvl: Gluon,
    /// Accumulated pressure integral (Q16.48). Path-independent checksum of pressure history.
    pub pressure: Q1648,
    /// Current saturation on curve (Q8.24). Higher = more crowded, costlier next entry.
    pub saturation: Q824,
    pub _pad: u32,
}

/// Element: single board group where players gather and accumulate pressure.
/// Resets (overloads) when pressure exceeds a threshold.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
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
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Board {
    /// Total value locked (TVL) across all charges.
    pub tvl: Gluon,
    /// Accumulated Gluon in quantum pocket (spent on rare unlocks).
    pub quantum_pocket: Gluon,
    /// Active charge count (used in global pressure calculations).
    pub charge_count: u32,
    /// Quantum unlock index (elements fully reset).
    pub quantum_index: u8,
    pub _pad: [u8; 3],
}

/// Artefact: snapshot of a reset element. Players can claim their share but cannot re-enter.
/// Created when an element resets (saturation exceeds threshold).
/// Field order: 8+8 = 16 bytes.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Artefact {
    /// Remaining pot to distribute to shareholders.
    pub pot: Gluon,
    /// Which element this was (reference only).
    pub index: ElementIndex,
}

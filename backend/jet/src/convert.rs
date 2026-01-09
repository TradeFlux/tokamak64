//! Bidirectional conversions between nucleus types and FlatBuffers.
//!
//! Provides conversions for deserializing FlatBuffers into nucleus types,
//! and helper functions for serializing nucleus types into FlatBuffers.
//!
//! ## Deserialization: FlatBuffers → Nucleus
//! Implemented via `From` traits for zero-copy reading.
//!
//! ## Serialization: Nucleus → FlatBuffers
//! Use the helper functions in this module with a `FlatBufferBuilder`.

use crate::player as fb_player;
use crate::tokamak::{board as fb_board, types as fb_types};

// Re-export player types module for disambiguation
use crate::player_generated::tokamak::types as fb_player_types;

// ============================================================================
// Types conversions
// ============================================================================

/// Convert FlatBuffers AddressBytes (game) to nucleus AddressBytes ([u8; 32])
impl From<&fb_types::AddressBytes> for nucleus::types::AddressBytes {
    fn from(fb: &fb_types::AddressBytes) -> Self {
        fb.0
    }
}

/// Convert FlatBuffers AddressBytes (player) to nucleus AddressBytes ([u8; 32])
impl From<&fb_player_types::AddressBytes> for nucleus::types::AddressBytes {
    fn from(fb: &fb_player_types::AddressBytes) -> Self {
        fb.0
    }
}

// Note: We can't implement From<u64> for ElementIndex/Coordinates due to orphan rules.
// Use helper functions instead.

/// Convert u64 from FlatBuffers to ElementIndex
#[inline]
pub fn to_element_index(val: u64) -> nucleus::types::ElementIndex {
    nucleus::types::ElementIndex(val)
}

/// Convert ElementIndex to u64 for FlatBuffers
#[inline]
pub fn from_element_index(idx: nucleus::types::ElementIndex) -> u64 {
    idx.0
}

/// Convert u64 from FlatBuffers to Coordinates
#[inline]
pub fn to_coordinates(val: u64) -> nucleus::types::Coordinates {
    nucleus::types::Coordinates(val)
}

/// Convert Coordinates to u64 for FlatBuffers
#[inline]
pub fn from_coordinates(coords: nucleus::types::Coordinates) -> u64 {
    coords.0
}

// ============================================================================
// Board conversions: FlatBuffers → Nucleus
// ============================================================================

/// Convert FlatBuffers Curve to nucleus Curve
impl From<fb_board::Curve<'_>> for nucleus::board::Curve {
    fn from(fb: fb_board::Curve<'_>) -> Self {
        nucleus::board::Curve {
            capacity: fb.capacity(),
            tvl: fb.tvl(),
            pressure: fb.pressure(),
            saturation: fb.saturation(),
            shares: fb.shares(),
        }
    }
}

/// Convert FlatBuffers Element to nucleus Element
impl From<fb_board::Element<'_>> for nucleus::board::Element {
    fn from(fb: fb_board::Element<'_>) -> Self {
        nucleus::board::Element {
            pot: fb.pot(),
            index: to_element_index(fb.index()),
            curve: fb.curve().unwrap().into(),
            coordinates: to_coordinates(fb.coordinates()),
        }
    }
}

/// Convert FlatBuffers Board to nucleus Board
impl From<fb_board::Board<'_>> for nucleus::board::Board {
    fn from(fb: fb_board::Board<'_>) -> Self {
        nucleus::board::Board {
            tvl: fb.tvl(),
            quantum_pocket: fb.quantum_pocket(),
            charge_count: fb.charge_count(),
            quantum_index: fb.quantum_index(),
            _pad: [0; 3],
        }
    }
}

/// Convert FlatBuffers Artefact to nucleus Artefact
impl From<fb_board::Artefact<'_>> for nucleus::board::Artefact {
    fn from(fb: fb_board::Artefact<'_>) -> Self {
        use bytemuck::Zeroable;
        let mut art = nucleus::board::Artefact::zeroed();
        art.pot = fb.pot();
        art.index = to_element_index(fb.index());
        art.shares = fb.shares();
        art
    }
}

// ============================================================================
// Player conversions: FlatBuffers → Nucleus
// ============================================================================

/// Convert FlatBuffers Wallet to nucleus Wallet
impl From<fb_player::Wallet<'_>> for nucleus::player::Wallet {
    fn from(fb: fb_player::Wallet<'_>) -> Self {
        use bytemuck::Zeroable;
        let mut wallet = nucleus::player::Wallet::zeroed();
        wallet.balance = fb.balance();
        wallet.authority = fb.authority().unwrap().into();
        wallet.mint = fb.mint().unwrap().into();
        wallet.charges = fb.charges();
        wallet
    }
}

/// Convert FlatBuffers Charge to nucleus Charge
impl From<fb_player::Charge<'_>> for nucleus::player::Charge {
    fn from(fb: fb_player::Charge<'_>) -> Self {
        use bytemuck::Zeroable;
        let mut charge = nucleus::player::Charge::zeroed();
        charge.balance = fb.balance();
        charge.timestamp = fb.timestamp();
        charge.index = to_element_index(fb.index());
        charge.share = fb.share();
        charge.authority = fb.authority().unwrap().into();
        charge.mint = fb.mint().unwrap().into();
        charge
    }
}

// ============================================================================
// Serialization helpers: Nucleus → FlatBuffers
// ============================================================================

/// Create FlatBuffers CurveArgs from nucleus Curve
#[inline]
pub fn curve_args(curve: &nucleus::board::Curve) -> fb_board::CurveArgs {
    fb_board::CurveArgs {
        capacity: curve.capacity,
        tvl: curve.tvl,
        pressure: curve.pressure,
        saturation: curve.saturation,
        shares: curve.shares,
    }
}

/// Create FlatBuffers BoardArgs from nucleus Board
#[inline]
pub fn board_args(board: &nucleus::board::Board) -> fb_board::BoardArgs {
    fb_board::BoardArgs {
        tvl: board.tvl,
        quantum_pocket: board.quantum_pocket,
        charge_count: board.charge_count,
        quantum_index: board.quantum_index,
    }
}

/// Create FlatBuffers ArtefactArgs from nucleus Artefact
#[inline]
pub fn artefact_args(art: &nucleus::board::Artefact) -> fb_board::ArtefactArgs {
    fb_board::ArtefactArgs {
        pot: art.pot,
        index: from_element_index(art.index),
        shares: art.shares,
    }
}

/// Create FlatBuffers AddressBytes from nucleus AddressBytes
#[inline]
pub fn address_bytes(addr: &nucleus::types::AddressBytes) -> fb_types::AddressBytes {
    fb_types::AddressBytes(*addr)
}

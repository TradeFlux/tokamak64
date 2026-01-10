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

use crate::api::{Action, BoardEvent, PlayerEvent, SnapshotResponse};
use crate::fb::tokamak as fb;
use crate::tokamak;
use std::fmt;

// ============================================================================
// Error types
// ============================================================================

/// Error type for event conversion failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventConvertError {
    InvalidPlayerEvent(fb::PlayerEvent),
    InvalidBoardEvent(fb::BoardEvent),
    MissingEventData,
}

impl fmt::Display for EventConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPlayerEvent(e) => write!(f, "Invalid PlayerEvent type: {:?}", e),
            Self::InvalidBoardEvent(e) => write!(f, "Invalid BoardEvent type: {:?}", e),
            Self::MissingEventData => write!(f, "Missing event data"),
        }
    }
}

impl std::error::Error for EventConvertError {}

// ============================================================================
// Types conversions
// ============================================================================

/// Convert FlatBuffers AddressBytes to nucleus AddressBytes ([u8; 32])
impl From<&fb::AddressBytes> for tokamak::AddressBytes {
    fn from(fb: &fb::AddressBytes) -> Self {
        fb.0
    }
}

/// Convert nucleus AddressBytes to FlatBuffers AddressBytes
impl From<&tokamak::AddressBytes> for fb::AddressBytes {
    fn from(addr: &tokamak::AddressBytes) -> Self {
        fb::AddressBytes(*addr)
    }
}

// ============================================================================
// Board conversions: FlatBuffers → Nucleus
// ============================================================================

/// Convert FlatBuffers Curve to nucleus Curve
impl From<fb::Curve<'_>> for tokamak::Curve {
    fn from(fb: fb::Curve<'_>) -> Self {
        tokamak::Curve {
            capacity: fb.capacity(),
            tvl: fb.tvl(),
            pressure: fb.pressure(),
            saturation: fb.saturation(),
            shares: fb.shares(),
        }
    }
}

/// Convert nucleus Curve to FlatBuffers Curve
impl From<&tokamak::Curve> for fb::CurveArgs {
    fn from(curve: &tokamak::Curve) -> Self {
        fb::CurveArgs {
            capacity: curve.capacity,
            tvl: curve.tvl,
            pressure: curve.pressure,
            saturation: curve.saturation,
            shares: curve.shares,
        }
    }
}

/// Convert FlatBuffers Element to nucleus Element
impl From<fb::Element<'_>> for tokamak::Element {
    fn from(fb: fb::Element<'_>) -> Self {
        tokamak::Element {
            pot: fb.pot(),
            index: fb.index().into(),
            curve: fb.curve().unwrap().into(),
            coordinates: fb.coordinates().into(),
        }
    }
}

/// Convert FlatBuffers Board to nucleus Board
impl From<fb::Board<'_>> for tokamak::Board {
    fn from(fb: fb::Board<'_>) -> Self {
        tokamak::Board {
            tvl: fb.tvl(),
            quantum_pocket: fb.quantum_pocket(),
            charge_count: fb.charge_count(),
            quantum_index: fb.quantum_index(),
            _pad: [0; 3],
        }
    }
}

/// Convert nucleus Board to FlatBuffers Board
impl From<&tokamak::Board> for fb::BoardArgs {
    fn from(board: &tokamak::Board) -> Self {
        fb::BoardArgs {
            tvl: board.tvl,
            quantum_pocket: board.quantum_pocket,
            charge_count: board.charge_count,
            quantum_index: board.quantum_index,
        }
    }
}

/// Convert FlatBuffers Artefact to nucleus Artefact
impl From<fb::Artefact<'_>> for tokamak::Artefact {
    fn from(fb: fb::Artefact<'_>) -> Self {
        use bytemuck::Zeroable;
        let mut art = tokamak::Artefact::zeroed();
        art.pot = fb.pot();
        art.index = fb.index().into();
        art.shares = fb.shares();
        art
    }
}

/// Convert nucleus Artefact to FlatBuffers Artefact
impl From<&tokamak::Artefact> for fb::ArtefactArgs {
    fn from(art: &tokamak::Artefact) -> Self {
        fb::ArtefactArgs {
            pot: art.pot,
            index: art.index.into(),
            shares: art.shares,
        }
    }
}

// ============================================================================
// Player conversions: FlatBuffers → Nucleus
// ============================================================================

/// Convert FlatBuffers Wallet to nucleus Wallet
impl From<fb::Wallet<'_>> for tokamak::Wallet {
    fn from(fb: fb::Wallet<'_>) -> Self {
        use bytemuck::Zeroable;
        let mut wallet = tokamak::Wallet::zeroed();
        wallet.balance = fb.balance();
        wallet.authority = fb.authority().into();
        wallet.mint = fb.mint().into();
        wallet.charges = fb.charges();
        wallet
    }
}

/// Convert FlatBuffers Charge to nucleus Charge
impl From<fb::Charge<'_>> for tokamak::Charge {
    fn from(fb: fb::Charge<'_>) -> Self {
        use bytemuck::Zeroable;
        let mut charge = tokamak::Charge::zeroed();
        charge.balance = fb.balance();
        charge.timestamp = fb.timestamp();
        charge.index = fb.index().into();
        charge.share = fb.share();
        charge.authority = fb.authority().into();
        charge.mint = fb.mint().into();
        charge
    }
}

// ============================================================================
// API Type Conversions (deserialization only)
// Serialization is handled by api.rs serialize methods
// ============================================================================

// SnapshotResponse: From FlatBuffers
impl From<&fb::SnapshotResponse<'_>> for SnapshotResponse {
    fn from(fb: &fb::SnapshotResponse<'_>) -> Self {
        let game = fb.game();
        let wallets = fb.wallets();
        let charges = fb.charges();
        Self {
            board: game.board().into(),
            elements: game.elements().iter().map(|e| e.into()).collect(),
            artefacts: game.artefacts().iter().map(|a| a.into()).collect(),
            snapshot_time: game.snapshot_time(),
            slot: game.slot(),
            wallets: (0..wallets.len()).map(|i| wallets.get(i).into()).collect(),
            charges: (0..charges.len()).map(|i| charges.get(i).into()).collect(),
        }
    }
}

// Action: From FlatBuffers
impl<'a> From<&fb::Action<'a>> for Action<'a> {
    fn from(fb: &fb::Action<'a>) -> Self {
        Self {
            transaction: fb.transaction().bytes(),
            player: fb.player().into(),
        }
    }
}

// PlayerEventMessage: TryFrom FlatBuffers → extract inner event
impl TryFrom<&fb::PlayerEventMessage<'_>> for PlayerEvent {
    type Error = EventConvertError;

    fn try_from(fb: &fb::PlayerEventMessage<'_>) -> Result<Self, Self::Error> {
        match fb.event_type() {
            fb::PlayerEvent::Wallet => {
                let wallet = fb
                    .event_as_wallet()
                    .ok_or(EventConvertError::MissingEventData)?
                    .into();
                Ok(PlayerEvent::Wallet(wallet))
            }
            fb::PlayerEvent::Charge => {
                let charge = fb
                    .event_as_charge()
                    .ok_or(EventConvertError::MissingEventData)?
                    .into();
                Ok(PlayerEvent::Charge(charge))
            }
            _ => Err(EventConvertError::InvalidPlayerEvent(fb.event_type())),
        }
    }
}

// BoardEventMessage: TryFrom FlatBuffers → extract inner event
impl TryFrom<&fb::BoardEventMessage<'_>> for BoardEvent {
    type Error = EventConvertError;

    fn try_from(fb: &fb::BoardEventMessage<'_>) -> Result<Self, Self::Error> {
        match fb.event_type() {
            fb::BoardEvent::Board => {
                let board = fb
                    .event_as_board()
                    .ok_or(EventConvertError::MissingEventData)?
                    .into();
                Ok(BoardEvent::Board(board))
            }
            fb::BoardEvent::Element => {
                let element = fb
                    .event_as_element()
                    .ok_or(EventConvertError::MissingEventData)?
                    .into();
                Ok(BoardEvent::Element(element))
            }
            fb::BoardEvent::Artefact => {
                let artefact = fb
                    .event_as_artefact()
                    .ok_or(EventConvertError::MissingEventData)?
                    .into();
                Ok(BoardEvent::Artefact(artefact))
            }
            _ => Err(EventConvertError::InvalidBoardEvent(fb.event_type())),
        }
    }
}

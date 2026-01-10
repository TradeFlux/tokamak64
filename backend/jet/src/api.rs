//! Rust wrapper types for FlatBuffers API message types.

use crate::tokamak;

// ============================================================================
// SnapshotResponse
// ============================================================================

#[derive(Debug, Clone)]
pub struct SnapshotResponse {
    pub board: tokamak::Board,
    pub elements: Vec<tokamak::Element>,
    pub artefacts: Vec<tokamak::Artefact>,
    pub snapshot_time: u64,
    pub slot: u64,
    pub wallets: Vec<tokamak::Wallet>,
    pub charges: Vec<tokamak::Charge>,
}

// ============================================================================
// PlayerEvent
// ============================================================================

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Wallet(tokamak::Wallet),
    Charge(tokamak::Charge),
}

// ============================================================================
// BoardEvent
// ============================================================================

#[derive(Debug, Clone)]
pub enum BoardEvent {
    Board(tokamak::Board),
    Element(tokamak::Element),
    Artefact(tokamak::Artefact),
}

// ============================================================================
// Action
// ============================================================================

#[derive(Debug, Clone)]
pub struct Action<'a> {
    pub transaction: &'a [u8],
    pub player: tokamak::AddressBytes,
}

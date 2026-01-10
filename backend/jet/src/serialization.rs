// Serialization implementations for API types.
//
// This module contains all serialization logic for converting Rust types
// into FlatBuffers format using a FlatBufferBuilder.

use crate::api::{Action, BoardEvent, PlayerEvent, SnapshotResponse};
use crate::fb::tokamak as fb;
use crate::tokamak;

// ============================================================================
// Helper functions: serialize individual types
// ============================================================================

pub(crate) fn serialize_element<'a>(
    element: &tokamak::Element,
    fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
) -> flatbuffers::WIPOffset<fb::Element<'a>> {
    let curve = fb::Curve::create(fbb, &fb::CurveArgs::from(&element.curve));
    let curve = Some(curve);
    let args = &fb::ElementArgs {
        pot: element.pot,
        index: element.index.into(),
        curve,
        coordinates: element.coordinates.into(),
    };
    fb::Element::create(fbb, args)
}

pub(crate) fn serialize_artefact<'a>(
    artefact: &tokamak::Artefact,
    fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
) -> flatbuffers::WIPOffset<fb::Artefact<'a>> {
    let args = &fb::ArtefactArgs::from(artefact);
    fb::Artefact::create(fbb, args)
}

pub(crate) fn serialize_wallet<'a>(
    wallet: &tokamak::Wallet,
    fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
) -> flatbuffers::WIPOffset<fb::Wallet<'a>> {
    let authority = Some(&fb::AddressBytes(wallet.authority));
    let mint = Some(&fb::AddressBytes(wallet.mint));
    let args = &fb::WalletArgs {
        balance: wallet.balance,
        authority,
        mint,
        charges: wallet.charges,
    };
    fb::Wallet::create(fbb, args)
}

pub(crate) fn serialize_charge<'a>(
    charge: &tokamak::Charge,
    fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
) -> flatbuffers::WIPOffset<fb::Charge<'a>> {
    let authority = Some(&fb::AddressBytes(charge.authority));
    let mint = Some(&fb::AddressBytes(charge.mint));
    let args = &fb::ChargeArgs {
        balance: charge.balance,
        timestamp: charge.timestamp,
        index: charge.index.into(),
        share: charge.share,
        authority,
        mint,
    };
    fb::Charge::create(fbb, args)
}

// ============================================================================
// SnapshotResponse
// ============================================================================

impl SnapshotResponse {
    pub fn serialize<'a>(
        &self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<fb::SnapshotResponse<'a>> {
        // Serialize board
        let board = fb::Board::create(fbb, &fb::BoardArgs::from(&self.board));

        // Serialize elements
        let element_offsets: Vec<_> = self
            .elements
            .iter()
            .map(|e| serialize_element(e, fbb))
            .collect();
        let elements = fbb.create_vector(&element_offsets);

        // Serialize artefacts
        let artefact_offsets: Vec<_> = self
            .artefacts
            .iter()
            .map(|a| serialize_artefact(a, fbb))
            .collect();
        let artefacts = fbb.create_vector(&artefact_offsets);

        // Serialize game
        let board = Some(board);
        let elements = Some(elements);
        let artefacts = Some(artefacts);
        let game_args = &fb::GameArgs {
            board,
            elements,
            artefacts,
            snapshot_time: self.snapshot_time,
            slot: self.slot,
        };
        let game = fb::Game::create(fbb, game_args);

        // Serialize wallets
        let wallet_offsets: Vec<_> = self
            .wallets
            .iter()
            .map(|w| serialize_wallet(w, fbb))
            .collect();
        let wallets = fbb.create_vector(&wallet_offsets);

        // Serialize charges
        let charge_offsets: Vec<_> = self
            .charges
            .iter()
            .map(|c| serialize_charge(c, fbb))
            .collect();
        let charges = fbb.create_vector(&charge_offsets);

        // Create response
        let game = Some(game);
        let wallets = Some(wallets);
        let charges = Some(charges);
        let args = &fb::SnapshotResponseArgs {
            game,
            wallets,
            charges,
        };
        fb::SnapshotResponse::create(fbb, args)
    }
}

// ============================================================================
// PlayerEvent
// ============================================================================

impl PlayerEvent {
    pub fn serialize<'a>(
        &self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> (
        flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>,
        fb::PlayerEvent,
    ) {
        match self {
            Self::Wallet(wallet) => {
                let offset = serialize_wallet(wallet, fbb);
                (offset.as_union_value(), fb::PlayerEvent::Wallet)
            }
            Self::Charge(charge) => {
                let offset = serialize_charge(charge, fbb);
                (offset.as_union_value(), fb::PlayerEvent::Charge)
            }
        }
    }
}

// ============================================================================
// BoardEvent
// ============================================================================

impl BoardEvent {
    pub fn serialize<'a>(
        &self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> (
        flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>,
        fb::BoardEvent,
    ) {
        match self {
            Self::Board(board) => {
                let args = &fb::BoardArgs::from(board);
                let offset = fb::Board::create(fbb, args);
                (offset.as_union_value(), fb::BoardEvent::Board)
            }
            Self::Element(element) => {
                let offset = serialize_element(element, fbb);
                (offset.as_union_value(), fb::BoardEvent::Element)
            }
            Self::Artefact(artefact) => {
                let offset = serialize_artefact(artefact, fbb);
                (offset.as_union_value(), fb::BoardEvent::Artefact)
            }
        }
    }
}

// ============================================================================
// Action
// ============================================================================

impl<'a> Action<'a> {
    pub fn serialize(
        &self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<fb::Action<'a>> {
        let player = Some(&fb::AddressBytes(self.player));
        let transaction = Some(fbb.create_vector(self.transaction));
        let args = &fb::ActionArgs {
            transaction,
            player,
        };
        fb::Action::create(fbb, args)
    }
}

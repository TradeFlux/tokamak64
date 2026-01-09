//! FlatBuffers-generated types for TOKAMAK64 game state serialization.
//!
//! This crate contains auto-generated code from FlatBuffers schemas.
//! The code is generated at build time via build.rs using the flatc compiler.
//!
//! ## Schema Modules
//! - `tokamak::board` - Board state (Curve, Element, Board, Artefact)
//! - `tokamak::player` - Player accounts (Wallet, Charge)
//! - `tokamak` - Root Game table
//!

pub mod convert;

// Include generated FlatBuffers code for game state
#[path = ""]
mod game_generated {
    include!(concat!(env!("OUT_DIR"), "/game.rs"));
}

// Include generated FlatBuffers code for player accounts
#[path = ""]
mod player_generated {
    include!(concat!(env!("OUT_DIR"), "/player.rs"));
}

// Re-export all generated types under the tokamak namespace
pub use game_generated::tokamak;
pub use player_generated::tokamak::player;

//! FlatBuffers-generated types for TOKAMAK64 game state serialization.
//!
//! This crate contains auto-generated code from FlatBuffers schemas.
//! The code is generated at build time via build.rs using the flatc compiler.

pub mod api;
pub mod convert;
pub mod serialization;

// FlatBuffers generated types
pub mod fb {
    // Use api.rs which includes the other schemas
    include!(concat!(env!("OUT_DIR"), "/api.rs"));
    pub use tokamak::*;
}

pub mod tokamak {
    pub use nucleus::board::*;
    pub use nucleus::player::*;
    pub use nucleus::types::*;
}

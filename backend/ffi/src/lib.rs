//! FFI bindings for TOKAMAK64 nucleus crate.
//! Exposes transparent types and core game mechanics for Dart/Flutter.
//!
//! All types are PODs (Plain Old Data) with repr(C) or repr(transparent).
//! Safe to pass across FFI boundary without serialization.

pub use nucleus::board::{Artefact, Board, Curve, Element};
pub use nucleus::player::{Charge, Wallet};
pub use nucleus::types::{AddressBytes, Coordinates, ElementIndex, Gluon, Q1648, Q824};

// Re-export core game functions
pub use nucleus::action::{claim, compress, rebind};
pub use nucleus::fees::{bind_fee, compression_fee, fee_multiplier, rebind_fee, unbind_fee};
pub use nucleus::round_divide;

// Re-export constants
pub use nucleus::consts::{
    DECIMALS, MAX_ATOMIC_NUMBER, MAX_DELTA_TIMESTAMP, MAX_SATURATION, MAX_SPEED_MULTIPLIER,
    MIN_FEE, SUM_ATOMIC_NUMBERS,
};

// FFI-specific re-exports (no-op wrappers for clarity in generated bindings)
#[no_mangle]
pub extern "C" fn tokamak_version() -> u32 {
    1
}

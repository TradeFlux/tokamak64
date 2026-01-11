//! Element account factories.

use super::accounts::program_account;
use super::constants::*;
use super::prelude::*;
use super::types::AccountWithPubkey;
use super::utils::elem_index;
use nucleus::{
    board::{Curve, Element},
    types::Coordinates,
};

/// Creates element account with shares
pub fn element_with_shares(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
    shares: u32,
) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let e = Element {
        pot,
        index: elem_index(atomic),
        curve: Curve {
            capacity: 1_000_000_000_000,
            tvl: 0,
            pressure: 0,
            saturation,
            shares,
        },
        coordinates: Coordinates(coords),
    };
    let data = bytes_of(&e).to_vec();
    AccountWithPubkey {
        pubkey: key,
        account: program_account(data),
    }
}

/// Creates element account
pub fn element(atomic: u64, coords: u64, saturation: u32, pot: u64) -> AccountWithPubkey {
    element_with_shares(atomic, coords, saturation, pot, 0)
}

/// Creates element at edge with default values (common default)
pub fn element_edge(atomic: u64) -> AccountWithPubkey {
    element(atomic, EDGE_COORD, 0, 0)
}

/// Creates element at edge with saturation (common default)
pub fn element_edge_sat(atomic: u64, saturation: u32) -> AccountWithPubkey {
    element(atomic, EDGE_COORD, saturation, 0)
}

/// Creates element at edge with shares (common default)
pub fn element_edge_shared(atomic: u64, shares: u32) -> AccountWithPubkey {
    element_with_shares(atomic, EDGE_COORD, 0, 0, shares)
}

/// Creates element at custom coordinates (no shares)
pub fn element_at(atomic: u64, coords: u64) -> AccountWithPubkey {
    element(atomic, coords, 0, 0)
}

/// Creates element at custom coordinates with pot and shares
pub fn element_with_shares_at(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
    shares: u32,
) -> AccountWithPubkey {
    element_with_shares(atomic, coords, saturation, pot, shares)
}

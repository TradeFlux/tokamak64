//! Artefact account factories.

use super::accounts::program_account;
use super::constants::*;
use super::prelude::*;
use super::types::AccountWithPubkey;
use nucleus::{board::Artefact, types::ElementIndex};

/// Creates artefact account with index and shares
pub fn artefact_full(pot: u64, index: ElementIndex, shares: u32) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let a = Artefact {
        pot,
        index,
        shares,
        _pad: 0,
    };
    let data = bytes_of(&a).to_vec();
    AccountWithPubkey {
        pubkey: key,
        account: program_account(data),
    }
}

/// Creates artefact account (index and shares default to zero)
pub fn artefact(pot: u64) -> AccountWithPubkey {
    artefact_full(pot, ZERO_INDEX, 0)
}

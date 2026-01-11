//! Board account factories.

use super::accounts::program_account;
use super::constants::*;
use super::prelude::*;
use super::types::AccountWithPubkey;
use nucleus::board::Board;

/// Creates board account
pub fn board(tvl: u64, charge_count: u32) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let b = Board {
        tvl,
        quantum_pocket: 0,
        charge_count,
        quantum_index: 0,
        _pad: [0u8; 3],
    };
    let data = bytes_of(&b).to_vec();
    AccountWithPubkey {
        pubkey: key,
        account: program_account(data),
    }
}

/// Creates board with 0 tvl and 0 charge_count (common default)
pub fn board_empty() -> AccountWithPubkey {
    board(0, 0)
}

/// Creates board with BAL_HIGH tvl and charge_count (common default)
pub fn board_with_count(charge_count: u32) -> AccountWithPubkey {
    board(BAL_HIGH, charge_count)
}

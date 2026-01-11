//! Account factories for wallets and charges.

use super::constants::*;
use super::prelude::*;
use super::types::AccountWithPubkey;
use nucleus::{player::Charge, player::Wallet, types::ElementIndex};

/// Base account config for program-owned accounts
pub fn program_account(data: Vec<u8>) -> Account {
    Account {
        lamports: LAMPORTS,
        data,
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    }
}

/// Creates signer account with high lamports
pub fn signer() -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let account = Account {
        lamports: SIGNER_LAMPORTS,
        data: vec![],
        owner: Pubkey::default(),
        executable: false,
        rent_epoch: 0,
    };
    (key, account).into()
}

/// Creates wallet account
pub fn wallet(authority: &Pubkey, balance: u64) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let w = Wallet {
        balance,
        authority: authority.to_bytes(),
        mint: [0u8; 32],
        charges: 0,
        _pad: 0,
    };
    let data = bytes_of(&w).to_vec();
    (key, program_account(data)).into()
}

/// Creates wallet with BAL_MIN (common default)
pub fn wallet_min(authority: &Pubkey) -> AccountWithPubkey {
    wallet(authority, BAL_MIN)
}

/// Creates charge account
pub fn charge_with_share(
    authority: &Pubkey,
    balance: u64,
    index: ElementIndex,
    share: u32,
) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let c = Charge {
        balance,
        timestamp: 0,
        index,
        share,
        authority: authority.to_bytes(),
        mint: [0u8; 32],
        _pad: 0,
    };
    let data = bytes_of(&c).to_vec();
    AccountWithPubkey {
        pubkey: key,
        account: program_account(data),
    }
}

/// Creates charge account (zero share)
pub fn charge(authority: &Pubkey, balance: u64, index: ElementIndex) -> AccountWithPubkey {
    charge_with_share(authority, balance, index, 0)
}

/// Creates charge account with BAL_MIN and ZERO_INDEX (common default)
pub fn charge_min(authority: &Pubkey) -> AccountWithPubkey {
    charge(authority, BAL_MIN, ZERO_INDEX)
}

/// Creates charge account with BAL_HIGH and ZERO_INDEX (common default)
pub fn charge_high(authority: &Pubkey) -> AccountWithPubkey {
    charge(authority, BAL_HIGH, ZERO_INDEX)
}

/// Creates charge account with BAL_HIGH and index (common default)
pub fn charge_high_with_index(authority: &Pubkey, index: ElementIndex) -> AccountWithPubkey {
    charge(authority, BAL_HIGH, index)
}

/// Creates charge account with BAL_HIGH, index, and SHARE_ONE (common default)
pub fn charge_shared(authority: &Pubkey, index: ElementIndex) -> AccountWithPubkey {
    charge_with_share(authority, BAL_HIGH, index, SHARE_ONE)
}

//! PDA derivation and helpers.

use super::constants::PROGRAM_ID;
use super::prelude::*;
use super::types::AccountWithPubkey;

/// Derive wallet PDA and bump from signer and mint
pub fn derive_wallet_pda(signer: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[signer.as_ref(), mint.as_ref()], &PROGRAM_ID)
}

/// Derive charge PDA and bump from signer, mint, and charge index
pub fn derive_charge_pda(signer: &Pubkey, mint: &[u8; 32], index: u32) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[signer.as_ref(), mint, &index.to_le_bytes()], &PROGRAM_ID)
}

/// Creates an uninitialized PDA account (no data, owned by system program)
pub fn pda_account(pda: Pubkey) -> AccountWithPubkey {
    AccountWithPubkey {
        pubkey: pda,
        account: Account {
            lamports: 0,
            data: vec![],
            // System program ID is all zeros
            owner: Pubkey::default(),
            executable: false,
            rent_epoch: 0,
        },
    }
}

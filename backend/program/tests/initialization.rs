//! Tests for InitWallet and InitCharge instructions (PDA account creation).
//!
//! Note: These tests focus on account validation and error paths.
//! Full CPI flow testing (CreateAccount) requires solana-program-test or similar.

mod common;
use common::*;

use nucleus::player::Wallet;

// ============================================================================
// INITWALLET INSTRUCTION TESTS
// ============================================================================

/// InitWallet fails when signer is not marked as signer
#[test]
fn init_wallet_fails_missing_signature() {
    let signer = signer();
    let signer_key = signer.pubkey;
    let mint_key = Pubkey::new_unique();
    let mint = AccountWithPubkey {
        pubkey: mint_key,
        account: Account::default(),
    };
    let (wallet_pda, bump) = derive_wallet_pda(&signer_key, &mint_key);
    let wallet = pda_account(wallet_pda);

    test_run!(
        ix!(
            TokamakInstruction::InitWallet,
            bump,
            vec![
                AccountMeta::new(signer_key, false), // Not a signer!
                AccountMeta::new(wallet_pda, false),
                AccountMeta::new_readonly(mint_key, false),
            ]
        ),
        &[signer.into(), wallet.into(), mint.into()],
        &[Check::err(ProgramError::MissingRequiredSignature)]
    );
}

// ============================================================================
// PDA DERIVATION TESTS
// ============================================================================

/// Wallet PDA is deterministic from signer and mint
#[test]
fn wallet_pda_is_deterministic() {
    let signer_key = Pubkey::new_unique();
    let mint_key = Pubkey::new_unique();

    let (pda1, bump1) = derive_wallet_pda(&signer_key, &mint_key);
    let (pda2, bump2) = derive_wallet_pda(&signer_key, &mint_key);

    assert_eq!(pda1, pda2, "PDA should be deterministic");
    assert_eq!(bump1, bump2, "Bump should be deterministic");
}

/// Charge PDA is deterministic from signer, mint, and index
#[test]
fn charge_pda_is_deterministic() {
    let signer_key = Pubkey::new_unique();
    let mint = [1u8; 32];

    let (pda1, bump1) = derive_charge_pda(&signer_key, &mint, 0);
    let (pda2, bump2) = derive_charge_pda(&signer_key, &mint, 0);

    assert_eq!(pda1, pda2, "PDA should be deterministic");
    assert_eq!(bump1, bump2, "Bump should be deterministic");
}

/// Charge PDA changes with different index
#[test]
fn charge_pda_differs_by_index() {
    let signer_key = Pubkey::new_unique();
    let mint = [1u8; 32];

    let (pda1, _) = derive_charge_pda(&signer_key, &mint, 0);
    let (pda2, _) = derive_charge_pda(&signer_key, &mint, 1);

    assert_ne!(pda1, pda2, "PDA should differ by index");
}

// ============================================================================
// ACCOUNT VALIDATION TESTS
// ============================================================================

/// InitCharge fails when wallet is not owned by the program
#[test]
fn init_charge_fails_invalid_wallet_data() {
    let signer = signer();
    let signer_key = signer.pubkey;
    let mint = Pubkey::new_unique();

    let (wallet_pda, _) = derive_wallet_pda(&signer_key, &mint);
    let wallet_data = Wallet {
        balance: 0,
        authority: signer_key.to_bytes(),
        mint: mint.to_bytes(),
        charges: 0,
        _pad: 0,
    };
    let wallet = AccountWithPubkey {
        pubkey: wallet_pda,
        account: program_account(bytes_of(&wallet_data).to_vec()),
    };

    let (charge_pda, bump) = derive_charge_pda(&signer_key, &mint.to_bytes(), 0);
    let charge = pda_account(charge_pda);

    test_run!(
        ix!(
            TokamakInstruction::InitCharge,
            bump,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::InvalidAccountData)]
    );
}

/// InitCharge fails when wallet has wrong owner
#[test]
fn init_charge_fails_invalid_wallet_owner() {
    let signer = signer();
    let signer_key = signer.pubkey;
    let mint = Pubkey::new_unique();

    let (wallet_pda, _) = derive_wallet_pda(&signer_key, &mint);
    let wallet_data = Wallet {
        balance: 0,
        authority: signer_key.to_bytes(),
        mint: mint.to_bytes(),
        charges: 0,
        _pad: 0,
    };
    // Create wallet with wrong owner (system program instead of our program)
    let mut wallet_account = program_account(bytes_of(&wallet_data).to_vec());
    wallet_account.owner = SYSTEM_PROGRAM_ID;

    let wallet = AccountWithPubkey {
        pubkey: wallet_pda,
        account: wallet_account,
    };

    let (charge_pda, bump) = derive_charge_pda(&signer_key, &mint.to_bytes(), 0);
    let charge = pda_account(charge_pda);

    test_run!(
        ix!(
            TokamakInstruction::InitCharge,
            bump,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::InvalidAccountData)]
    );
}

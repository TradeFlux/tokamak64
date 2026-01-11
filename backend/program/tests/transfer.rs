//! Tests for Charge (wallet→charge) and Discharge (charge→wallet) instructions.

mod common;
use common::*;

use nucleus::player::Wallet;

// ============================================================================
// CHARGE INSTRUCTION TESTS
// ============================================================================

/// Verify successful transfer from wallet to charge account
#[test]
fn charge_success_transfer_from_wallet_to_charge() {
    let signer = signer();
    let charge = charge(&signer.pubkey, 0, ZERO_INDEX);
    let wallet = wallet_min(&signer.pubkey);

    let result = test_run!(
        ix!(
            TokamakInstruction::Charge,
            AMT_HALF,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::success()]
    );

    assert_charge_bal(&result, 1, AMT_HALF);
    let w: Wallet = result.get(2);
    assert_eq!(w.balance, AMT_HALF);
}

/// Charge instruction must reject zero amount
#[test]
fn charge_fails_zero_amount() {
    let signer = signer();
    let charge = charge_min(&signer.pubkey);
    let wallet = wallet_min(&signer.pubkey);

    test_run!(
        ix!(
            TokamakInstruction::Charge,
            0u64,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::InvalidArgument)]
    );
}

/// Charge fails when wallet lacks balance
#[test]
fn charge_fails_insufficient_wallet_balance() {
    let signer = signer();
    let charge = charge_min(&signer.pubkey);
    let wallet = wallet(&signer.pubkey, 100);

    test_run!(
        ix!(
            TokamakInstruction::Charge,
            1_000u64,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::ArithmeticOverflow)]
    );
}

/// Charge fails when signer doesn't match wallet authority
#[test]
fn charge_fails_wrong_authority() {
    let signer = signer();
    let other = Pubkey::new_unique();
    let charge = charge_min(&signer.pubkey);
    let wallet = wallet_min(&other);

    test_run!(
        ix!(
            TokamakInstruction::Charge,
            AMT_HALF,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::IncorrectAuthority)]
    );
}

/// Charge adds to existing charge balance
#[test]
fn charge_accumulates_to_existing_balance() {
    let signer = signer();
    let charge = charge(&signer.pubkey, 100_000, ZERO_INDEX);
    let wallet = wallet_min(&signer.pubkey);

    let result = test_run!(
        ix!(
            TokamakInstruction::Charge,
            200_000u64,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::success()]
    );

    assert_charge_bal(&result, 1, 300_000);
}

// ============================================================================
// DISCHARGE INSTRUCTION TESTS
// ============================================================================

/// Verify successful transfer from charge to wallet account
#[test]
fn discharge_success_transfer_from_charge_to_wallet() {
    let signer = signer();
    let charge = charge(&signer.pubkey, BAL_MIN, ZERO_INDEX);
    let wallet = wallet(&signer.pubkey, 0);

    let result = test_run!(
        ix!(
            TokamakInstruction::Discharge,
            AMT_HALF,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::success()]
    );

    assert_charge_bal(&result, 1, AMT_HALF);
    let w: Wallet = result.get(2);
    assert_eq!(w.balance, AMT_HALF);
}

/// Discharge instruction must reject zero amount
#[test]
fn discharge_fails_zero_amount() {
    let signer = signer();
    let charge = charge(&signer.pubkey, BAL_MIN, ZERO_INDEX);
    let wallet = wallet(&signer.pubkey, 0);

    test_run!(
        ix!(
            TokamakInstruction::Discharge,
            0u64,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::InvalidArgument)]
    );
}

/// Discharge fails when charge lacks balance
#[test]
fn discharge_fails_insufficient_charge_balance() {
    let signer = signer();
    let charge = charge(&signer.pubkey, 100, ZERO_INDEX);
    let wallet = wallet(&signer.pubkey, 0);

    test_run!(
        ix!(
            TokamakInstruction::Discharge,
            1_000u64,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::InsufficientFunds)]
    );
}

/// Discharge fails when charge is bound to an element (Custom(50))
#[test]
fn discharge_fails_charge_is_bound() {
    let signer = signer();
    let index = elem_index(1);
    let charge = charge(&signer.pubkey, BAL_MIN, index);
    let wallet = wallet(&signer.pubkey, 0);

    test_run!(
        ix!(
            TokamakInstruction::Discharge,
            AMT_HALF,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::Custom(50))]
    );
}

/// Discharge fails when signer doesn't match charge authority
#[test]
fn discharge_fails_wrong_authority() {
    let signer = signer();
    let other = Pubkey::new_unique();
    let charge = charge(&signer.pubkey, BAL_MIN, ZERO_INDEX);
    let wallet = wallet(&other, 0);

    test_run!(
        ix!(
            TokamakInstruction::Discharge,
            AMT_HALF,
            metas!(signer, charge, wallet)
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::IncorrectAuthority)]
    );
}

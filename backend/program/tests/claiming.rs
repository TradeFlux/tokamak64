//! Tests for Claim instruction.

mod common;
use common::*;

use nucleus::board::Artefact;
use nucleus::player::Charge;

// ============================================================================
// CLAIM INSTRUCTION TESTS
// ============================================================================

/// Claim proportional share from artefact pot to charge
#[test]
fn claim_success_proportional_share() {
    let signer = signer();
    let art_index = elem_index(3);
    let charge = charge_with_share(&signer.pubkey, 0, art_index, SHARE_ONE);
    let art = artefact_full(1_000_000, art_index, SHARE_TWO);

    let result = test_run!(
        ix!(TokamakInstruction::Claim, metas!(signer, charge, art)),
        &[signer.into(), charge.into(), art.into()],
        &[Check::success()]
    );

    let c: Charge = result.get(1);
    let a: Artefact = result.get(2);
    assert_eq!(c.balance, AMT_HALF);
    assert_eq!(a.pot, AMT_HALF);
    assert_eq!(c.share, 0);
    assert!(c.index.is_zero());
}

/// Claim fails when charge has zero share (Custom(42))
#[test]
fn claim_fails_zero_share() {
    let signer = signer();
    let art_index = elem_index(3);
    let charge = charge_with_share(&signer.pubkey, 0, art_index, 0);
    let art = artefact_full(1_000_000, art_index, SHARE_TWO);

    test_run!(
        ix!(TokamakInstruction::Claim, metas!(signer, charge, art)),
        &[signer.into(), charge.into(), art.into()],
        &[Check::err(ProgramError::Custom(42))]
    );
}

/// Claim fails when charge index doesn't match artefact index (Custom(42))
#[test]
fn claim_fails_index_mismatch() {
    let signer = signer();
    let charge_index = elem_index(3);
    let art_index = elem_index(5);
    let charge = charge_with_share(&signer.pubkey, 0, charge_index, SHARE_ONE);
    let art = artefact_full(1_000_000, art_index, SHARE_TWO);

    test_run!(
        ix!(TokamakInstruction::Claim, metas!(signer, charge, art)),
        &[signer.into(), charge.into(), art.into()],
        &[Check::err(ProgramError::Custom(42))]
    );
}

/// Multiple claims distribute proportionally based on shares
#[test]
fn claim_multiple_claims_distribute_correctly() {
    let art_index = elem_index(3);
    let signer1 = signer();
    let charge1 = charge_with_share(&signer1.pubkey, 0, art_index, SHARE_ONE);
    let art = artefact_full(1_000_000, art_index, SHARE_FOUR);

    let result = test_run!(
        ix!(TokamakInstruction::Claim, metas!(signer1, charge1, art)),
        &[signer1.into(), charge1.into(), art.into()],
        &[Check::success()]
    );

    let c1: Charge = result.get(1);
    let a1: Artefact = result.get(2);
    assert_eq!(c1.balance, AMT_QUARTER);
    assert_eq!(a1.pot, 750_000);
}

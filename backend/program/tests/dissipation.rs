//! Tests for Vent instruction.

mod common;
use common::*;

use nucleus::board::Element;
use nucleus::player::Charge;

// ============================================================================
// VENT INSTRUCTION TESTS
// ============================================================================

/// Vent transfers charge balance to element pot
#[test]
fn vent_success_to_element_pot() {
    let signer = signer();
    let elem_index = elem_index(3);
    let initial = BAL_MIN;
    let vent_amount = 200_000;
    let charge = charge(&signer.pubkey, initial, elem_index);
    let elem = element_edge(3);

    let result = test_run!(
        ix!(TokamakInstruction::Vent, vent_amount, vec![
            AccountMeta::new(signer.pubkey, true),
            AccountMeta::new(charge.pubkey, false),
            AccountMeta::new(elem.pubkey, false),
        ]),
        &[signer.into(), charge.into(), elem.into()],
        &[Check::success()]
    );

    let c: Charge = result.get(1);
    let e: Element = result.get(2);
    assert_eq!(c.balance, initial - vent_amount);
    assert_eq!(e.pot, vent_amount);
}

/// Vent fails when amount is zero
#[test]
fn vent_fails_zero_amount() {
    let signer = signer();
    let elem_index = elem_index(3);
    let charge = charge(&signer.pubkey, BAL_MIN, elem_index);
    let elem = element_edge(3);

    test_run!(
        ix!(TokamakInstruction::Vent, 0, vec![
            AccountMeta::new(signer.pubkey, true),
            AccountMeta::new(charge.pubkey, false),
            AccountMeta::new(elem.pubkey, false),
        ]),
        &[signer.into(), charge.into(), elem.into()],
        &[Check::err(ProgramError::InvalidArgument)]
    );
}

/// Vent fails when charge has insufficient balance
#[test]
fn vent_fails_insufficient_balance() {
    let signer = signer();
    let elem_index = elem_index(3);
    let charge = charge(&signer.pubkey, 100, elem_index);
    let elem = element_edge(3);

    test_run!(
        ix!(TokamakInstruction::Vent, 1_000, vec![
            AccountMeta::new(signer.pubkey, true),
            AccountMeta::new(charge.pubkey, false),
            AccountMeta::new(elem.pubkey, false),
        ]),
        &[signer.into(), charge.into(), elem.into()],
        &[Check::err(ProgramError::ArithmeticOverflow)]
    );
}

/// Vent fails when element doesn't match charge index (Custom(32))
#[test]
fn vent_fails_wrong_target_element() {
    let signer = signer();
    let elem_index = elem_index(3);
    let charge = charge(&signer.pubkey, BAL_MIN, elem_index);
    let elem = element_edge(5);

    test_run!(
        ix!(TokamakInstruction::Vent, 200_000, vec![
            AccountMeta::new(signer.pubkey, true),
            AccountMeta::new(charge.pubkey, false),
            AccountMeta::new(elem.pubkey, false),
        ]),
        &[signer.into(), charge.into(), elem.into()],
        &[Check::err(ProgramError::Custom(32))]
    );
}

//! Cross-cutting tests for authority validation and speed tax.

mod common;
use common::*;

use nucleus::player::Charge;

// ============================================================================
// AUTHORITY VALIDATION TESTS
// ============================================================================

/// Charge instruction fails when signer is not marked as signer
#[test]
fn authority_fails_missing_signature() {
    let signer = signer();
    let charge = charge_min(&signer.pubkey);
    let wallet = wallet_min(&signer.pubkey);

    test_run!(
        ix!(
            TokamakInstruction::Charge,
            AMT_HALF,
            vec![
                AccountMeta::new(signer.pubkey, false),
                AccountMeta::new(charge.pubkey, false),
                AccountMeta::new(wallet.pubkey, false),
            ]
        ),
        &[signer.into(), charge.into(), wallet.into()],
        &[Check::err(ProgramError::MissingRequiredSignature)]
    );
}

// ============================================================================
// SPEED TAX TESTS
// ============================================================================

/// Rapid actions pay higher fees than patient actions (speed tax)
#[test]
fn speed_tax_higher_fee_for_rapid_actions() {
    let elem_index = elem_index(1);
    let charge_key = Pubkey::new_unique();
    let signer = signer();

    // Both charges bound at slot 0; speed tax depends on current slot (elapsed time)
    let charge_base = Charge {
        balance: BAL_MAX,
        timestamp: 0,
        index: elem_index,
        share: SHARE_ONE,
        authority: signer.pubkey.to_bytes(),
        mint: [0u8; 32],
        _pad: 0,
    };
    let charge_acc = Account {
        lamports: BAL_MIN,
        data: bytes_of(&charge_base).to_vec(),
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };

    let elem = element_with_shares_at(1, EDGE_COORD, SHARE_ONE, BAL_MIN, SHARE_ONE);
    let board = board_with_count(1);

    let ix = ix!(
        TokamakInstruction::Unbind,
        vec![
            AccountMeta::new(signer.pubkey, true),
            AccountMeta::new(charge_key, false),
            AccountMeta::new(elem.pubkey, false),
            AccountMeta::new(board.pubkey, false),
        ]
    );

    // Rapid: unbind immediately (0 slots elapsed, multiplier = 128)
    let mut mollusk_rapid = mollusk();
    mollusk_rapid.warp_to_slot(0);
    let result_rapid = mollusk_rapid.process_and_validate_instruction(
        &ix,
        &[
            signer.clone().into(),
            (charge_key, charge_acc.clone()),
            elem.clone().into(),
            board.clone().into(),
        ],
        &[Check::success()],
    );

    // Patient: unbind after waiting (1000 slots elapsed, multiplier = 1)
    let mut mollusk_patient = mollusk();
    mollusk_patient.warp_to_slot(1000);
    let result_patient = mollusk_patient.process_and_validate_instruction(
        &ix,
        &[
            signer.into(),
            (charge_key, charge_acc),
            elem.into(),
            board.into(),
        ],
        &[Check::success()],
    );

    let charge_patient_result: Charge = result_patient.get(1);
    let charge_rapid_result: Charge = result_rapid.get(1);

    assert!(
        charge_rapid_result.balance < charge_patient_result.balance,
        "Rapid balance {} should be less than patient balance {}",
        charge_rapid_result.balance,
        charge_patient_result.balance
    );
}

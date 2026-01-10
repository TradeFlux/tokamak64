//! Cross-cutting tests for authority validation and speed tax.

mod common;
use common::*;

use nucleus::player::Charge;

// ============================================================================
// AUTHORITY VALIDATION TESTS
// ============================================================================

mod authority_tests {
    use super::*;

    /// Charge instruction fails when signer is not marked as signer
    #[test]
    fn fails_missing_signature() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge_min(&signer.pubkey);
        let wallet = wallet_min(&signer.pubkey);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data_with_u64(TokamakInstruction::Charge, AMT_HALF),
            vec![
                AccountMeta::new(signer.pubkey, false),
                AccountMeta::new(charge.pubkey, false),
                AccountMeta::new(wallet.pubkey, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), wallet.into()],
            &[Check::err(ProgramError::MissingRequiredSignature)],
        );
    }
}

// ============================================================================
// SPEED TAX TESTS
// ============================================================================

mod speed_tax_tests {
    use super::*;

    /// Rapid actions pay higher fees than patient actions (speed tax)
    #[test]
    fn higher_fee_for_rapid_actions() {
        let mut mollusk_patient = mollusk();
        mollusk_patient.warp_to_slot(2000);

        let signer = signer();
        let elem_index = elem_index(1);
        let charge_key = Pubkey::new_unique();

        let charge_patient = Charge {
            balance: BAL_MAX,
            timestamp: 0,
            index: elem_index,
            share: SHARE_ONE,
            authority: signer.pubkey.to_bytes(),
            mint: [0u8; 32],
            _pad: 0,
        };
        let charge_acc_patient = Account {
            lamports: BAL_MIN,
            data: bytes_of(&charge_patient).to_vec(),
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        };

        let elem = element_with_shares_at(1, EDGE_COORD, SHARE_ONE, BAL_MIN, SHARE_ONE);
        let board = board_with_count(1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Unbind),
            vec![
                AccountMeta::new(signer.pubkey, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem.pubkey, false),
                AccountMeta::new(board.pubkey, false),
            ],
        );

        let result_patient = mollusk_patient.process_and_validate_instruction(
            &ix.clone(),
            &[
                signer.clone().into(),
                (charge_key, charge_acc_patient),
                elem.clone().into(),
                board.clone().into(),
            ],
            &[Check::success()],
        );

        let mut mollusk_rapid = mollusk();
        mollusk_rapid.warp_to_slot(1100);

        let charge_rapid = Charge {
            balance: BAL_MAX,
            timestamp: 1000,
            index: elem_index,
            share: SHARE_ONE,
            authority: signer.pubkey.to_bytes(),
            mint: [0u8; 32],
            _pad: 0,
        };
        let charge_acc_rapid = Account {
            lamports: BAL_MIN,
            data: bytes_of(&charge_rapid).to_vec(),
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        };

        let result_rapid = mollusk_rapid.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                (charge_key, charge_acc_rapid),
                elem.into(),
                board.into(),
            ],
            &[Check::success()],
        );

        let charge_patient_result: Charge = read(&result_patient.resulting_accounts[1].1);
        let charge_rapid_result: Charge = read(&result_rapid.resulting_accounts[1].1);

        assert!(
            charge_rapid_result.balance < charge_patient_result.balance,
            "Rapid balance {} should be less than patient balance {}",
            charge_rapid_result.balance,
            charge_patient_result.balance
        );
    }
}

//! Cross-cutting tests for authority validation and speed tax.

mod common;
use common::*;

use bytemuck::bytes_of;
use mollusk_svm::result::Check;
use nucleus::player::Charge;
use nucleus::types::ElementIndex;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

// ============================================================================
// AUTHORITY VALIDATION TESTS
// ============================================================================

mod authority_tests {
    use super::*;

    #[test]
    fn fails_missing_signature() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 0, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 1_000_000);

        let mut data = Vec::with_capacity(16);
        data.extend_from_slice(&IX_CHARGE.to_le_bytes());
        data.extend_from_slice(&500_000u64.to_le_bytes());

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(signer_key, false),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::err(ProgramError::MissingRequiredSignature)],
        );
    }
}

// ============================================================================
// SPEED TAX TESTS
// ============================================================================

mod speed_tax_tests {
    use super::*;

    #[test]
    fn higher_fee_for_rapid_actions() {
        let mut mollusk_patient = mollusk();
        mollusk_patient.warp_to_slot(2000);

        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((1u64 << 56) | 1);
        let charge_key = Pubkey::new_unique();

        let charge_patient = Charge {
            balance: 100_000_000,
            timestamp: 0,
            index: elem_index,
            share: 1 << 24,
            authority: signer_key.to_bytes(),
            mint: [0u8; 32],
            _pad: 0,
        };
        let charge_acc_patient = solana_sdk::account::Account {
            lamports: 1_000_000,
            data: bytes_of(&charge_patient).to_vec(),
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        };

        let (elem_key, elem_acc, _) = make_element_with_shares(1, EDGE_COORD, 1 << 24, 100_000, 1 << 24);
        let (board_key, board_acc, _) = make_board(100_000_000, 1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            IX_UNBIND.to_le_bytes().as_ref(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result_patient = mollusk_patient.process_and_validate_instruction(
            &ix.clone(),
            &[
                (signer_key, signer.clone()),
                (charge_key, charge_acc_patient),
                (elem_key, elem_acc.clone()),
                (board_key, board_acc.clone()),
            ],
            &[Check::success()],
        );

        let mut mollusk_rapid = mollusk();
        mollusk_rapid.warp_to_slot(1100);

        let charge_rapid = Charge {
            balance: 100_000_000,
            timestamp: 1000,
            index: elem_index,
            share: 1 << 24,
            authority: signer_key.to_bytes(),
            mint: [0u8; 32],
            _pad: 0,
        };
        let charge_acc_rapid = solana_sdk::account::Account {
            lamports: 1_000_000,
            data: bytes_of(&charge_rapid).to_vec(),
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        };

        let result_rapid = mollusk_rapid.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc_rapid),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge_patient_result: Charge = read_account(&result_patient.resulting_accounts[1].1);
        let charge_rapid_result: Charge = read_account(&result_rapid.resulting_accounts[1].1);

        assert!(
            charge_rapid_result.balance < charge_patient_result.balance,
            "Rapid balance {} should be less than patient balance {}",
            charge_rapid_result.balance,
            charge_patient_result.balance
        );
    }
}

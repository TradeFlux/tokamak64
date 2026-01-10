//! Tests for Vent instruction.

mod common;
use common::*;

use mollusk_svm::result::Check;
use nucleus::board::Element;
use nucleus::player::Charge;
use nucleus::types::ElementIndex;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};

mod vent_tests {
    use super::*;

    fn ix_data(amount: u64) -> Vec<u8> {
        let mut data = Vec::with_capacity(16);
        data.extend_from_slice(&IX_VENT.to_le_bytes());
        data.extend_from_slice(&amount.to_le_bytes());
        data
    }

    #[test]
    fn success_vent_to_element_pot() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, elem_index);
        let (elem_key, elem_acc, _) = make_element(3, EDGE_COORD, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(200_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (elem_key, elem_acc)],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let element: Element = read_account(&result.resulting_accounts[2].1);

        assert_eq!(charge.balance, 800_000);
        assert_eq!(element.pot, 200_000);
    }

    #[test]
    fn fails_zero_amount() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, elem_index);
        let (elem_key, elem_acc, _) = make_element(3, EDGE_COORD, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(0),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (elem_key, elem_acc)],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn fails_insufficient_balance() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 100, elem_index);
        let (elem_key, elem_acc, _) = make_element(3, EDGE_COORD, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(1_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (elem_key, elem_acc)],
            &[Check::err(ProgramError::ArithmeticOverflow)],
        );
    }

    #[test]
    fn fails_wrong_target_element() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, elem_index);
        let (elem_key, elem_acc, _) = make_element(5, EDGE_COORD, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(200_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (elem_key, elem_acc)],
            &[Check::err(ProgramError::Custom(32))],
        );
    }
}

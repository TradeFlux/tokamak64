//! Tests for Overload instruction.

mod common;
use common::*;

use mollusk_svm::result::Check;
use nucleus::consts::MAX_SATURATION;
use nucleus::types::ElementIndex;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};

mod overload_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_OVERLOAD.to_le_bytes().to_vec()
    }

    #[test]
    fn fails_below_max_saturation() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, elem_index);
        let (elem_key, elem_acc, _) = make_element(3, EDGE_COORD, MAX_SATURATION - 1, 500_000);
        let (art_key, art_acc, _) = make_artefact(0, ElementIndex(0), 0);
        let (board_key, board_acc, _) = make_board(10_000_000, 5);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(art_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (art_key, art_acc),
                (board_key, board_acc),
            ],
            &[Check::err(ProgramError::Custom(1))],
        );
    }
}

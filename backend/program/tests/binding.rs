//! Tests for Bind, Unbind, and Rebind instructions.

mod common;
use common::*;

use mollusk_svm::result::Check;
use nucleus::board::{Board, Element};
use nucleus::player::Charge;
use nucleus::types::ElementIndex;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};

// ============================================================================
// BIND INSTRUCTION TESTS
// ============================================================================

mod bind_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_BIND.to_le_bytes().to_vec()
    }

    #[test]
    fn success_bind_to_edge_element() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, ElementIndex(0));
        let (elem_key, elem_acc, _) = make_element(1, EDGE_COORD, 0, 0);
        let (board_key, board_acc, _) = make_board(0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let board: Board = read_account(&result.resulting_accounts[3].1);

        assert!(!charge.index.is_zero());
        assert!(board.tvl > 0);
        assert_eq!(board.charge_count, 1);
    }

    #[test]
    fn fails_not_on_edge() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, ElementIndex(0));
        let (elem_key, elem_acc, _) = make_element(1, INTERIOR_COORD, 0, 0);
        let (board_key, board_acc, _) = make_board(0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn fails_charge_already_bound() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let index = ElementIndex((2u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, index);
        let (elem_key, elem_acc, _) = make_element(1, EDGE_COORD, 0, 0);
        let (board_key, board_acc, _) = make_board(0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::err(ProgramError::Custom(43))],
        );
    }

    #[test]
    fn deducts_bind_fee() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let initial_balance = 10_000_000u64;
        let (charge_key, charge_acc, _) =
            make_charge(&signer_key, initial_balance, ElementIndex(0));
        let (elem_key, elem_acc, _) = make_element(5, EDGE_COORD, 1 << 24, 0);
        let (board_key, board_acc, _) = make_board(0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let element: Element = read_account(&result.resulting_accounts[2].1);

        assert!(charge.balance < initial_balance);
        assert!(element.pot > 0);
    }
}

// ============================================================================
// UNBIND INSTRUCTION TESTS
// ============================================================================

mod unbind_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_UNBIND.to_le_bytes().to_vec()
    }

    #[test]
    fn success_unbind_from_edge_element() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((1u64 << 56) | 1);
        let (charge_key, charge_acc, _) =
            make_charge_with_share(&signer_key, 10_000_000, elem_index, 1 << 24);
        let (elem_key, elem_acc, _) =
            make_element_with_shares(1, EDGE_COORD, 1 << 24, 100_000, 1 << 24);
        let (board_key, board_acc, _) = make_board(10_000_000, 1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let board: Board = read_account(&result.resulting_accounts[3].1);

        assert!(charge.index.is_zero());
        assert_eq!(board.charge_count, 0);
    }

    #[test]
    fn fails_not_on_edge() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((5u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, elem_index);
        let (elem_key, elem_acc, _) = make_element(5, INTERIOR_COORD, 0, 0);
        let (board_key, board_acc, _) = make_board(10_000_000, 1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn deducts_unbind_fee() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let initial_balance = 10_000_000u64;
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) =
            make_charge_with_share(&signer_key, initial_balance, elem_index, 1 << 24);
        let (elem_key, elem_acc, _) = make_element_with_shares(3, EDGE_COORD, 2 << 24, 0, 1 << 24);
        let (board_key, board_acc, _) = make_board(initial_balance, 1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let element: Element = read_account(&result.resulting_accounts[2].1);

        assert!(charge.balance < initial_balance);
        assert!(element.pot > 0);
    }
}

// ============================================================================
// REBIND INSTRUCTION TESTS
// ============================================================================

mod rebind_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_REBIND.to_le_bytes().to_vec()
    }

    #[test]
    fn success_rebind_to_adjacent_element() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let src_index = ElementIndex((1u64 << 56) | 1);
        let (charge_key, charge_acc, _) =
            make_charge_with_share(&signer_key, 10_000_000, src_index, 1 << 24);
        let (src_key, src_acc, _) =
            make_element_with_shares(1, src_coord, 1 << 24, 100_000, 1 << 24);
        let (dst_key, dst_acc, _) = make_element(2, dst_coord, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(src_key, false),
                AccountMeta::new(dst_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (src_key, src_acc),
                (dst_key, dst_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let dst: Element = read_account(&result.resulting_accounts[3].1);

        assert_eq!(charge.index, dst.index);
    }

    #[test]
    fn fails_not_adjacent() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = non_adjacent_coords();
        let src_index = ElementIndex((1u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, src_index);
        let (src_key, src_acc, _) = make_element(1, src_coord, 0, 0);
        let (dst_key, dst_acc, _) = make_element(3, dst_coord, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(src_key, false),
                AccountMeta::new(dst_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (src_key, src_acc),
                (dst_key, dst_acc),
            ],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn fails_charge_not_in_source() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let other_index = ElementIndex((5u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, other_index);
        let (src_key, src_acc, _) = make_element(1, src_coord, 0, 0);
        let (dst_key, dst_acc, _) = make_element(2, dst_coord, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(src_key, false),
                AccountMeta::new(dst_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (src_key, src_acc),
                (dst_key, dst_acc),
            ],
            &[Check::err(ProgramError::Custom(1))],
        );
    }

    #[test]
    fn fee_routing_outward_to_src() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let src_index = ElementIndex((5u64 << 56) | 1);
        let (charge_key, charge_acc, _) =
            make_charge_with_share(&signer_key, 10_000_000, src_index, 1 << 24);
        let (src_key, src_acc, _) = make_element_with_shares(5, src_coord, 1 << 24, 0, 1 << 24);
        let (dst_key, dst_acc, _) = make_element(2, dst_coord, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(src_key, false),
                AccountMeta::new(dst_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (src_key, src_acc),
                (dst_key, dst_acc),
            ],
            &[Check::success()],
        );

        let src: Element = read_account(&result.resulting_accounts[2].1);
        let dst: Element = read_account(&result.resulting_accounts[3].1);

        assert!(src.pot > 0);
        assert_eq!(dst.pot, 0);
    }

    #[test]
    fn fee_routing_inward_to_dst() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let src_index = ElementIndex((2u64 << 56) | 1);
        let (charge_key, charge_acc, _) =
            make_charge_with_share(&signer_key, 10_000_000, src_index, 1 << 24);
        let (src_key, src_acc, _) = make_element_with_shares(2, src_coord, 0, 0, 1 << 24);
        let (dst_key, dst_acc, _) = make_element(5, dst_coord, 1 << 24, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(src_key, false),
                AccountMeta::new(dst_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (src_key, src_acc),
                (dst_key, dst_acc),
            ],
            &[Check::success()],
        );

        let src: Element = read_account(&result.resulting_accounts[2].1);
        let dst: Element = read_account(&result.resulting_accounts[3].1);

        assert_eq!(src.pot, 0);
        assert!(dst.pot > 0);
    }
}

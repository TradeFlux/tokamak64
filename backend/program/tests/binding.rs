//! Tests for Bind, Unbind, and Rebind instructions.

mod common;
use common::*;

use nucleus::board::{Board, Element};
use nucleus::player::Charge;

// ============================================================================
// BIND INSTRUCTION TESTS
// ============================================================================

mod bind_tests {
    use super::*;

    /// Bind charge to edge element, validate board state
    #[test]
    fn success_bind_to_edge_element() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge_high(&signer.pubkey);
        let elem = element_edge(1);
        let board = board_empty();

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Bind),
            metas_charge_elem(signer.pubkey, charge.pubkey, elem.pubkey, board.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into(), board.into()],
            &[Check::success()],
        );

        let c: Charge = read(&result.resulting_accounts[1].1);
        let b: Board = read(&result.resulting_accounts[3].1);
        assert!(!c.index.is_zero());
        assert!(b.tvl > 0);
        assert_eq!(b.charge_count, 1);
    }

    /// Bind fails when element is not on board edge
    #[test]
    fn fails_not_on_edge() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge_high(&signer.pubkey);
        let elem = element_at(1, INTERIOR_COORD);
        let board = board_empty();

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Bind),
            metas_charge_elem(signer.pubkey, charge.pubkey, elem.pubkey, board.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into(), board.into()],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    /// Bind fails when charge is already bound (Custom(43))
    #[test]
    fn fails_charge_already_bound() {
        let mollusk = mollusk();
        let signer = signer();
        let index = elem_index(2);
        let charge = charge(&signer.pubkey, BAL_HIGH, index);
        let elem = element_edge(1);
        let board = board_empty();

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Bind),
            metas_charge_elem(signer.pubkey, charge.pubkey, elem.pubkey, board.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into(), board.into()],
            &[Check::err(ProgramError::Custom(43))],
        );
    }

    /// Bind deducts fee from charge balance to element pot
    #[test]
    fn deducts_bind_fee() {
        let mollusk = mollusk();
        let signer = signer();
        let initial = BAL_HIGH;
        let charge = charge(&signer.pubkey, initial, ZERO_INDEX);
        let elem = element_edge_sat(5, SHARE_ONE);
        let board = board_empty();

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Bind),
            metas_charge_elem(signer.pubkey, charge.pubkey, elem.pubkey, board.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into(), board.into()],
            &[Check::success()],
        );

        let c: Charge = read(&result.resulting_accounts[1].1);
        let e: Element = read(&result.resulting_accounts[2].1);

        assert!(c.balance < initial);
        assert!(e.pot > 0);
    }
}

// ============================================================================
// UNBIND INSTRUCTION TESTS
// ============================================================================

mod unbind_tests {
    use super::*;

    /// Unbind charge from edge element, reset index
    #[test]
    fn success_unbind_from_edge_element() {
        let mollusk = mollusk();
        let signer = signer();
        let elem_index = elem_index(1);
        let charge = charge_shared(&signer.pubkey, elem_index);
        let elem = element_with_shares_at(1, EDGE_COORD, SHARE_ONE, BAL_MIN, SHARE_ONE);
        let board = board_with_count(1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Unbind),
            metas_charge_elem(signer.pubkey, charge.pubkey, elem.pubkey, board.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into(), board.into()],
            &[Check::success()],
        );

        let c: Charge = read(&result.resulting_accounts[1].1);
        let b: Board = read(&result.resulting_accounts[3].1);
        assert!(c.index.is_zero());
        assert_eq!(b.charge_count, 0);
    }

    /// Unbind fails when element is not on board edge
    #[test]
    fn fails_not_on_edge() {
        let mollusk = mollusk();
        let signer = signer();
        let elem_index = elem_index(5);
        let charge = charge_high_with_index(&signer.pubkey, elem_index);
        let elem = element_at(5, INTERIOR_COORD);
        let board = board_with_count(1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Unbind),
            metas_charge_elem(signer.pubkey, charge.pubkey, elem.pubkey, board.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into(), board.into()],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    /// Unbind deducts fee from charge to element pot
    #[test]
    fn deducts_unbind_fee() {
        let mollusk = mollusk();
        let signer = signer();
        let initial = BAL_HIGH;
        let elem_index = elem_index(3);
        let charge = charge_with_share(&signer.pubkey, initial, elem_index, SHARE_ONE);
        let elem = element_with_shares_at(3, EDGE_COORD, SHARE_TWO, 0, SHARE_ONE);
        let board = board_with_count(1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Unbind),
            metas_charge_elem(signer.pubkey, charge.pubkey, elem.pubkey, board.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into(), board.into()],
            &[Check::success()],
        );

        let c: Charge = read(&result.resulting_accounts[1].1);
        let e: Element = read(&result.resulting_accounts[2].1);

        assert!(c.balance < initial);
        assert!(e.pot > 0);
    }
}

// ============================================================================
// REBIND INSTRUCTION TESTS
// ============================================================================

mod rebind_tests {
    use super::*;

    /// Rebind charge to adjacent element
    #[test]
    fn success_rebind_to_adjacent_element() {
        let mollusk = mollusk();
        let signer = signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let src_index = elem_index(1);
        let charge = charge_shared(&signer.pubkey, src_index);
        let src = element_with_shares_at(1, src_coord, SHARE_ONE, BAL_MIN, SHARE_ONE);
        let dst = element_at(2, dst_coord);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Rebind),
            metas_charge_src_dst(signer.pubkey, charge.pubkey, src.pubkey, dst.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), src.into(), dst.into()],
            &[Check::success()],
        );

        let c: Charge = read(&result.resulting_accounts[1].1);
        let dst_elem: Element = read(&result.resulting_accounts[3].1);

        assert_eq!(c.index, dst_elem.index);
    }

    /// Rebind fails when elements are not adjacent
    #[test]
    fn fails_not_adjacent() {
        let mollusk = mollusk();
        let signer = signer();
        let (_src_coord, dst_coord) = non_adjacent_coords();
        let src_index = elem_index(1);
        let charge = charge_high_with_index(&signer.pubkey, src_index);
        let src = element_edge(1);
        let dst = element_at(3, dst_coord);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Rebind),
            metas_charge_src_dst(signer.pubkey, charge.pubkey, src.pubkey, dst.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), src.into(), dst.into()],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    /// Rebind fails when charge is not in source element (Custom(1))
    #[test]
    fn fails_charge_not_in_source() {
        let mollusk = mollusk();
        let signer = signer();
        let (_src_coord, dst_coord) = adjacent_coords();
        let other_index = elem_index(5);
        let charge = charge_high_with_index(&signer.pubkey, other_index);
        let src = element_edge(1);
        let dst = element_at(2, dst_coord);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Rebind),
            metas_charge_src_dst(signer.pubkey, charge.pubkey, src.pubkey, dst.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), src.into(), dst.into()],
            &[Check::err(ProgramError::Custom(1))],
        );
    }

    /// Fee routes outward to source when source has higher saturation
    #[test]
    fn fee_routing_outward_to_src() {
        let mollusk = mollusk();
        let signer = signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let src_index = elem_index(5);
        let charge = charge_shared(&signer.pubkey, src_index);
        let src = element_with_shares_at(5, src_coord, SHARE_ONE, 0, SHARE_ONE);
        let dst = element_at(2, dst_coord);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Rebind),
            metas_charge_src_dst(signer.pubkey, charge.pubkey, src.pubkey, dst.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), src.into(), dst.into()],
            &[Check::success()],
        );

        let src_elem: Element = read(&result.resulting_accounts[2].1);
        let dst_elem: Element = read(&result.resulting_accounts[3].1);

        assert!(src_elem.pot > 0);
        assert_eq!(dst_elem.pot, 0);
    }

    /// Fee routes inward to destination when destination has higher saturation
    #[test]
    fn fee_routing_inward_to_dst() {
        let mollusk = mollusk();
        let signer = signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let src_index = elem_index(2);
        let charge = charge_shared(&signer.pubkey, src_index);
        let src = element_with_shares_at(2, src_coord, 0, 0, SHARE_ONE);
        let dst = element_at(5, dst_coord);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Rebind),
            metas_charge_src_dst(signer.pubkey, charge.pubkey, src.pubkey, dst.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), src.into(), dst.into()],
            &[Check::success()],
        );

        let src_elem: Element = read(&result.resulting_accounts[2].1);
        let dst_elem: Element = read(&result.resulting_accounts[3].1);

        assert_eq!(src_elem.pot, 0);
        assert!(dst_elem.pot > 0);
    }
}

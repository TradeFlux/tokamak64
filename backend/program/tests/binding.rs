//! Tests for Bind, Unbind, and Rebind instructions.

mod common;
use common::*;

use nucleus::board::{Board, Element};
use nucleus::player::Charge;

// ============================================================================
// BIND INSTRUCTION TESTS
// ============================================================================

/// Bind charge to edge element, validate board state
#[test]
fn bind_success_edge_element() {
    let signer = signer();
    let charge = charge_high(&signer.pubkey);
    let elem = element_edge(1);
    let board = board_empty();

    let result = test_run!(
        ix!(
            TokamakInstruction::Bind,
            metas!(signer, charge, elem, board)
        ),
        &[signer.into(), charge.into(), elem.into(), board.into()],
        &[Check::success()]
    );

    let c: Charge = result.get(1);
    let b: Board = result.get(3);
    assert!(!c.index.is_zero());
    assert!(b.tvl > 0);
    assert_eq!(b.charge_count, 1);
}

/// Bind fails when element is not on board edge
#[test]
fn bind_fails_not_on_edge() {
    let signer = signer();
    let charge = charge_high(&signer.pubkey);
    let elem = element_at(1, INTERIOR_COORD);
    let board = board_empty();

    test_run!(
        ix!(
            TokamakInstruction::Bind,
            metas!(signer, charge, elem, board)
        ),
        &[signer.into(), charge.into(), elem.into(), board.into()],
        &[Check::err(ProgramError::InvalidArgument)]
    );
}

/// Bind fails when charge is already bound (Custom(43))
#[test]
fn bind_fails_already_bound() {
    let signer = signer();
    let index = elem_index(2);
    let charge = charge(&signer.pubkey, BAL_HIGH, index);
    let elem = element_edge(1);
    let board = board_empty();

    test_run!(
        ix!(
            TokamakInstruction::Bind,
            metas!(signer, charge, elem, board)
        ),
        &[signer.into(), charge.into(), elem.into(), board.into()],
        &[Check::err(ProgramError::Custom(43))]
    );
}

/// Bind deducts fee from charge balance to element pot
#[test]
fn bind_deducts_fee() {
    let signer = signer();
    let initial = BAL_HIGH;
    let charge = charge(&signer.pubkey, initial, ZERO_INDEX);
    let elem = element_edge_sat(5, SHARE_ONE);
    let board = board_empty();

    let result = test_run!(
        ix!(
            TokamakInstruction::Bind,
            metas!(signer, charge, elem, board)
        ),
        &[signer.into(), charge.into(), elem.into(), board.into()],
        &[Check::success()]
    );

    let c: Charge = result.get(1);
    let e: Element = result.get(2);
    assert!(c.balance < initial);
    assert!(e.pot > 0);
}

// ============================================================================
// UNBIND INSTRUCTION TESTS
// ============================================================================

/// Unbind charge from edge element, reset index
#[test]
fn unbind_success_edge_element() {
    let signer = signer();
    let elem_index = elem_index(1);
    let charge = charge_shared(&signer.pubkey, elem_index);
    let elem = element_with_shares_at(1, EDGE_COORD, SHARE_ONE, BAL_MIN, SHARE_ONE);
    let board = board_with_count(1);

    let result = test_run!(
        ix!(
            TokamakInstruction::Unbind,
            metas!(signer, charge, elem, board)
        ),
        &[signer.into(), charge.into(), elem.into(), board.into()],
        &[Check::success()]
    );

    let c: Charge = result.get(1);
    let b: Board = result.get(3);
    assert!(c.index.is_zero());
    assert_eq!(b.charge_count, 0);
}

/// Unbind fails when element is not on board edge
#[test]
fn unbind_fails_not_on_edge() {
    let signer = signer();
    let elem_index = elem_index(5);
    let charge = charge_high_with_index(&signer.pubkey, elem_index);
    let elem = element_at(5, INTERIOR_COORD);
    let board = board_with_count(1);

    test_run!(
        ix!(
            TokamakInstruction::Unbind,
            metas!(signer, charge, elem, board)
        ),
        &[signer.into(), charge.into(), elem.into(), board.into()],
        &[Check::err(ProgramError::InvalidArgument)]
    );
}

/// Unbind deducts fee from charge to element pot
#[test]
fn unbind_deducts_fee() {
    let signer = signer();
    let initial = BAL_HIGH;
    let elem_index = elem_index(3);
    let charge = charge_with_share(&signer.pubkey, initial, elem_index, SHARE_ONE);
    let elem = element_with_shares_at(3, EDGE_COORD, SHARE_TWO, 0, SHARE_ONE);
    let board = board_with_count(1);

    let result = test_run!(
        ix!(
            TokamakInstruction::Unbind,
            metas!(signer, charge, elem, board)
        ),
        &[signer.into(), charge.into(), elem.into(), board.into()],
        &[Check::success()]
    );

    let c: Charge = result.get(1);
    let e: Element = result.get(2);
    assert!(c.balance < initial);
    assert!(e.pot > 0);
}

// ============================================================================
// REBIND INSTRUCTION TESTS
// ============================================================================

/// Rebind charge to adjacent element
#[test]
fn rebind_success_adjacent_element() {
    let signer = signer();
    let (src_coord, dst_coord) = adjacent_coords();
    let src_index = elem_index(1);
    let charge = charge_shared(&signer.pubkey, src_index);
    let src = element_with_shares_at(1, src_coord, SHARE_ONE, BAL_MIN, SHARE_ONE);
    let dst = element_at(2, dst_coord);

    let result = test_run!(
        ix!(TokamakInstruction::Rebind, metas!(signer, charge, src, dst)),
        &[signer.into(), charge.into(), src.into(), dst.into()],
        &[Check::success()]
    );

    let c: Charge = result.get(1);
    let dst_elem: Element = result.get(3);
    assert_eq!(c.index, dst_elem.index);
}

/// Rebind fails when elements are not adjacent
#[test]
fn rebind_fails_not_adjacent() {
    let signer = signer();
    let (_src_coord, dst_coord) = non_adjacent_coords();
    let src_index = elem_index(1);
    let charge = charge_high_with_index(&signer.pubkey, src_index);
    let src = element_edge(1);
    let dst = element_at(3, dst_coord);

    test_run!(
        ix!(TokamakInstruction::Rebind, metas!(signer, charge, src, dst)),
        &[signer.into(), charge.into(), src.into(), dst.into()],
        &[Check::err(ProgramError::InvalidArgument)]
    );
}

/// Rebind fails when charge is not in source element (Custom(1))
#[test]
fn rebind_fails_charge_not_in_source() {
    let signer = signer();
    let (_src_coord, dst_coord) = adjacent_coords();
    let other_index = elem_index(5);
    let charge = charge_high_with_index(&signer.pubkey, other_index);
    let src = element_edge(1);
    let dst = element_at(2, dst_coord);

    test_run!(
        ix!(TokamakInstruction::Rebind, metas!(signer, charge, src, dst)),
        &[signer.into(), charge.into(), src.into(), dst.into()],
        &[Check::err(ProgramError::Custom(1))]
    );
}

/// Fee routes outward to source when source has higher saturation
#[test]
fn rebind_fee_routing_outward() {
    let signer = signer();
    let (src_coord, dst_coord) = adjacent_coords();
    let src_index = elem_index(5);
    let charge = charge_shared(&signer.pubkey, src_index);
    let src = element_with_shares_at(5, src_coord, SHARE_ONE, 0, SHARE_ONE);
    let dst = element_at(2, dst_coord);

    let result = test_run!(
        ix!(TokamakInstruction::Rebind, metas!(signer, charge, src, dst)),
        &[signer.into(), charge.into(), src.into(), dst.into()],
        &[Check::success()]
    );

    let src_elem: Element = result.get(2);
    let dst_elem: Element = result.get(3);
    assert!(src_elem.pot > 0);
    assert_eq!(dst_elem.pot, 0);
}

/// Fee routes inward to destination when destination has higher saturation
#[test]
fn rebind_fee_routing_inward() {
    let signer = signer();
    let (src_coord, dst_coord) = adjacent_coords();
    let src_index = elem_index(2);
    let charge = charge_shared(&signer.pubkey, src_index);
    let src = element_with_shares_at(2, src_coord, 0, 0, SHARE_ONE);
    let dst = element_at(5, dst_coord);

    let result = test_run!(
        ix!(TokamakInstruction::Rebind, metas!(signer, charge, src, dst)),
        &[signer.into(), charge.into(), src.into(), dst.into()],
        &[Check::success()]
    );

    let src_elem: Element = result.get(2);
    let dst_elem: Element = result.get(3);
    assert_eq!(src_elem.pot, 0);
    assert!(dst_elem.pot > 0);
}

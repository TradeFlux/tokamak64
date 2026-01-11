//! Tests for Compress instruction.

mod common;
use common::*;

use nucleus::board::Element;
use nucleus::player::Charge;

// ============================================================================
// COMPRESS INSTRUCTION TESTS
// ============================================================================

/// Compress source element into destination element (inward)
#[test]
fn compress_success_inward() {
    let signer = signer();
    let (src_coord, _dst_coord) = adjacent_coords();
    let src_pot = AMT_HALF;
    let src_index = elem_index(2);
    let charge = charge_shared(&signer.pubkey, src_index);
    let src = element_with_shares_at(2, src_coord, SHARE_ONE, src_pot, SHARE_ONE);
    let dst = element_edge(5);

    let result = test_run!(
        ix!(
            TokamakInstruction::Compress,
            metas!(signer, charge, src, dst)
        ),
        &[signer.into(), charge.into(), src.into(), dst.into()],
        &[Check::success()]
    );

    let c: Charge = result.get(1);
    let src_elem: Element = result.get(2);
    let dst_elem: Element = result.get(3);
    assert_eq!(c.index, dst_elem.index);
    assert_eq!(src_elem.pot, 0);
    assert!(dst_elem.pot >= src_pot);
}

/// Compress fails when compressing outward (Custom(42))
#[test]
fn compress_fails_outward() {
    let signer = signer();
    let (src_coord, _dst_coord) = adjacent_coords();
    let src_index = elem_index(5);
    let charge = charge_high_with_index(&signer.pubkey, src_index);
    let src = element_at(5, src_coord);
    let dst = element_edge(2);

    test_run!(
        ix!(
            TokamakInstruction::Compress,
            metas!(signer, charge, src, dst)
        ),
        &[signer.into(), charge.into(), src.into(), dst.into()],
        &[Check::err(ProgramError::Custom(42))]
    );
}

/// Compress fails when charge is not bound to source element (Custom(1))
#[test]
fn compress_fails_charge_not_in_source() {
    let signer = signer();
    let (src_coord, _dst_coord) = adjacent_coords();
    let other_index = elem_index(10);
    let charge = charge_high_with_index(&signer.pubkey, other_index);
    let src = element_at(2, src_coord);
    let dst = element_edge(5);

    test_run!(
        ix!(
            TokamakInstruction::Compress,
            metas!(signer, charge, src, dst)
        ),
        &[signer.into(), charge.into(), src.into(), dst.into()],
        &[Check::err(ProgramError::Custom(1))]
    );
}

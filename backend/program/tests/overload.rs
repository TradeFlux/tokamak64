//! Tests for Overload instruction.

mod common;
use common::*;

// ============================================================================
// OVERLOAD INSTRUCTION TESTS
// ============================================================================

/// Overload fails when element is not at max saturation (Custom(1))
#[test]
fn overload_fails_below_max_saturation() {
    let signer = signer();
    let elem_index = elem_index(3);
    let charge = charge(&signer.pubkey, BAL_MIN, elem_index);
    let elem = element_with_shares_at(3, EDGE_COORD, MAX_SATURATION - 1, AMT_HALF, 0);
    let art = artefact(0);
    let board = board_with_count(5);

    test_run!(
        ix!(
            TokamakInstruction::Overload,
            metas!(signer, charge, elem, art, board)
        ),
        &[
            signer.into(),
            charge.into(),
            elem.into(),
            art.into(),
            board.into()
        ],
        &[Check::err(ProgramError::Custom(1))]
    );
}

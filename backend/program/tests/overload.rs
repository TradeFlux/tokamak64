//! Tests for Overload instruction.

mod common;
use common::*;

// ============================================================================
// OVERLOAD INSTRUCTION TESTS
// ============================================================================

mod overload_tests {
    use super::*;

    /// Overload fails when element is not at max saturation (Custom(1))
    #[test]
    fn fails_below_max_saturation() {
        let mollusk = mollusk();
        let signer = signer();
        let elem_index = elem_index(3);
        let charge = charge(&signer.pubkey, BAL_MIN, elem_index);
        let elem = element_with_shares_at(3, EDGE_COORD, MAX_SATURATION - 1, AMT_HALF, 0);
        let art = artefact(0);
        let board = board_with_count(5);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(TokamakInstruction::Overload),
            vec![
                AccountMeta::new(signer.pubkey, true),
                AccountMeta::new(charge.pubkey, false),
                AccountMeta::new(elem.pubkey, false),
                AccountMeta::new(art.pubkey, false),
                AccountMeta::new(board.pubkey, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                charge.into(),
                elem.into(),
                art.into(),
                board.into(),
            ],
            &[Check::err(ProgramError::Custom(1))],
        );
    }
}

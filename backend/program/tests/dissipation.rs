//! Tests for Vent instruction.

mod common;
use common::*;

use nucleus::board::Element;
use nucleus::player::Charge;

// ============================================================================
// VENT INSTRUCTION TESTS
// ============================================================================

mod vent_tests {
    use super::*;

    fn ix_data(amount: u64) -> Vec<u8> {
        ix_data_with_u64(TokamakInstruction::Vent, amount)
    }

    /// Vent transfers charge balance to element pot
    #[test]
    fn success_vent_to_element_pot() {
        let mollusk = mollusk();
        let signer = signer();
        let elem_index = elem_index(3);
        let initial = BAL_MIN;
        let vent_amount = 200_000;
        let charge = charge(&signer.pubkey, initial, elem_index);
        let elem = element_edge(3);

        let ix = Instruction::new_with_bytes(PROGRAM_ID, &ix_data(vent_amount), vec![
            AccountMeta::new(signer.pubkey, true),
            AccountMeta::new(charge.pubkey, false),
            AccountMeta::new(elem.pubkey, false),
        ]);

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into()],
            &[Check::success()],
        );

        let c: Charge = read(&result.resulting_accounts[1].1);
        let e: Element = read(&result.resulting_accounts[2].1);

        assert_eq!(c.balance, initial - vent_amount);
        assert_eq!(e.pot, vent_amount);
    }

    /// Vent fails when amount is zero
    #[test]
    fn fails_zero_amount() {
        let mollusk = mollusk();
        let signer = signer();
        let elem_index = elem_index(3);
        let charge = charge(&signer.pubkey, BAL_MIN, elem_index);
        let elem = element_edge(3);

        let ix = Instruction::new_with_bytes(PROGRAM_ID, &ix_data(0), vec![
            AccountMeta::new(signer.pubkey, true),
            AccountMeta::new(charge.pubkey, false),
            AccountMeta::new(elem.pubkey, false),
        ]);

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into()],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    /// Vent fails when charge has insufficient balance
    #[test]
    fn fails_insufficient_balance() {
        let mollusk = mollusk();
        let signer = signer();
        let elem_index = elem_index(3);
        let charge = charge(&signer.pubkey, 100, elem_index);
        let elem = element_edge(3);

        let ix = Instruction::new_with_bytes(PROGRAM_ID, &ix_data(1_000), vec![
            AccountMeta::new(signer.pubkey, true),
            AccountMeta::new(charge.pubkey, false),
            AccountMeta::new(elem.pubkey, false),
        ]);

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into()],
            &[Check::err(ProgramError::ArithmeticOverflow)],
        );
    }

    /// Vent fails when element doesn't match charge index (Custom(32))
    #[test]
    fn fails_wrong_target_element() {
        let mollusk = mollusk();
        let signer = signer();
        let elem_index = elem_index(3);
        let charge = charge(&signer.pubkey, BAL_MIN, elem_index);
        let elem = element_edge(5);

        let ix = Instruction::new_with_bytes(PROGRAM_ID, &ix_data(200_000), vec![
            AccountMeta::new(signer.pubkey, true),
            AccountMeta::new(charge.pubkey, false),
            AccountMeta::new(elem.pubkey, false),
        ]);

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), elem.into()],
            &[Check::err(ProgramError::Custom(32))],
        );
    }
}

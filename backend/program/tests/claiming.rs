//! Tests for Claim instruction.

mod common;
use common::*;

use nucleus::board::Artefact;
use nucleus::player::Charge;

// ============================================================================
// CLAIM INSTRUCTION TESTS
// ============================================================================

mod claim_tests {
    use super::*;

    /// Claim proportional share from artefact pot to charge
    #[test]
    fn success_claim_proportional_share() {
        let mollusk = mollusk();
        let signer = signer();
        let art_index = elem_index(3);
        let charge = charge_with_share(&signer.pubkey, 0, art_index, SHARE_ONE);
        let art = artefact_full(1_000_000, art_index, SHARE_TWO);

        let ix = Instruction::new_with_bytes(PROGRAM_ID, &ix_data(TokamakInstruction::Claim), metas_charge_art(signer.pubkey, charge.pubkey, art.pubkey));

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                charge.into(),
                art.into(),
            ],
            &[Check::success()],
        );

        let c: Charge = read(&result.resulting_accounts[1].1);
        let a: Artefact = read(&result.resulting_accounts[2].1);

        assert_eq!(c.balance, AMT_HALF);
        assert_eq!(a.pot, AMT_HALF);
        assert_eq!(c.share, 0);
        assert!(c.index.is_zero());
    }

    /// Claim fails when charge has zero share (Custom(42))
    #[test]
    fn fails_zero_share() {
        let mollusk = mollusk();
        let signer = signer();
        let art_index = elem_index(3);
        let charge = charge_with_share(&signer.pubkey, 0, art_index, 0);
        let art = artefact_full(1_000_000, art_index, SHARE_TWO);

        let ix = Instruction::new_with_bytes(PROGRAM_ID, &ix_data(TokamakInstruction::Claim), metas_charge_art(signer.pubkey, charge.pubkey, art.pubkey));

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                charge.into(),
                art.into(),
            ],
            &[Check::err(ProgramError::Custom(42))],
        );
    }

    /// Claim fails when charge index doesn't match artefact index (Custom(42))
    #[test]
    fn fails_index_mismatch() {
        let mollusk = mollusk();
        let signer = signer();
        let charge_index = elem_index(3);
        let art_index = elem_index(5);
        let charge = charge_with_share(&signer.pubkey, 0, charge_index, SHARE_ONE);
        let art = artefact_full(1_000_000, art_index, SHARE_TWO);

        let ix = Instruction::new_with_bytes(PROGRAM_ID, &ix_data(TokamakInstruction::Claim), metas_charge_art(signer.pubkey, charge.pubkey, art.pubkey));

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                charge.into(),
                art.into(),
            ],
            &[Check::err(ProgramError::Custom(42))],
        );
    }

    /// Multiple claims distribute proportionally based on shares
    #[test]
    fn multiple_claims_distribute_correctly() {
        let mollusk = mollusk();
        let art_index = elem_index(3);

        let signer1 = signer();
        let charge1 = charge_with_share(&signer1.pubkey, 0, art_index, SHARE_ONE);
        let art = artefact_full(1_000_000, art_index, SHARE_FOUR);

        let ix1 = Instruction::new_with_bytes(PROGRAM_ID, &ix_data(TokamakInstruction::Claim), metas_charge_art(signer1.pubkey, charge1.pubkey, art.pubkey));

        let result1 = mollusk.process_and_validate_instruction(
            &ix1,
            &[
                signer1.into(),
                charge1.into(),
                art.into(),
            ],
            &[Check::success()],
        );

        let c1: Charge = read(&result1.resulting_accounts[1].1);
        let a1: Artefact = read(&result1.resulting_accounts[2].1);

        assert_eq!(c1.balance, AMT_QUARTER);
        assert_eq!(a1.pot, 750_000);
    }
}

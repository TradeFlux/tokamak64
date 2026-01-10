//! Tests for Claim instruction.

mod common;
use common::*;

use mollusk_svm::result::Check;
use nucleus::board::Artefact;
use nucleus::player::Charge;
use nucleus::types::ElementIndex;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};

mod claim_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_CLAIM.to_le_bytes().to_vec()
    }

    #[test]
    fn success_claim_proportional_share() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let art_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) =
            make_charge_with_share(&signer_key, 0, art_index, 1 << 24);
        let (art_key, art_acc, _) = make_artefact(1_000_000, art_index, 2 << 24);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(art_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (art_key, art_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let artefact: Artefact = read_account(&result.resulting_accounts[2].1);

        assert_eq!(charge.balance, 500_000);
        assert_eq!(artefact.pot, 500_000);
        assert_eq!(charge.share, 0);
        assert!(charge.index.is_zero());
    }

    #[test]
    fn fails_zero_share() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let art_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge_with_share(&signer_key, 0, art_index, 0);
        let (art_key, art_acc, _) = make_artefact(1_000_000, art_index, 2 << 24);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(art_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (art_key, art_acc),
            ],
            &[Check::err(ProgramError::Custom(42))],
        );
    }

    #[test]
    fn fails_index_mismatch() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let charge_index = ElementIndex((3u64 << 56) | 1);
        let art_index = ElementIndex((5u64 << 56) | 1);
        let (charge_key, charge_acc, _) =
            make_charge_with_share(&signer_key, 0, charge_index, 1 << 24);
        let (art_key, art_acc, _) = make_artefact(1_000_000, art_index, 2 << 24);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(art_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (art_key, art_acc),
            ],
            &[Check::err(ProgramError::Custom(42))],
        );
    }

    #[test]
    fn multiple_claims_distribute_correctly() {
        let mollusk = mollusk();
        let art_index = ElementIndex((3u64 << 56) | 1);

        let (signer1_key, signer1) = make_signer();
        let (charge1_key, charge1_acc, _) =
            make_charge_with_share(&signer1_key, 0, art_index, 1 << 24);
        let (art_key, art_acc, _) = make_artefact(1_000_000, art_index, 4 << 24);

        let ix1 = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer1_key, true),
                AccountMeta::new(charge1_key, false),
                AccountMeta::new(art_key, false),
            ],
        );

        let result1 = mollusk.process_and_validate_instruction(
            &ix1,
            &[
                (signer1_key, signer1),
                (charge1_key, charge1_acc),
                (art_key, art_acc),
            ],
            &[Check::success()],
        );

        let charge1: Charge = read_account(&result1.resulting_accounts[1].1);
        let artefact1: Artefact = read_account(&result1.resulting_accounts[2].1);

        assert_eq!(charge1.balance, 250_000);
        assert_eq!(artefact1.pot, 750_000);
    }
}

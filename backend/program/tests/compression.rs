//! Tests for Compress instruction.

mod common;
use common::*;

use mollusk_svm::result::Check;
use nucleus::board::Element;
use nucleus::player::Charge;
use nucleus::types::ElementIndex;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};

mod compress_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_COMPRESS.to_le_bytes().to_vec()
    }

    #[test]
    fn success_compress_inward() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let src_pot = 500_000u64;
        let src_index = ElementIndex((2u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge_with_share(&signer_key, 10_000_000, src_index, 1 << 24);
        let (src_key, src_acc, _) = make_element_with_shares(2, src_coord, 1 << 24, src_pot, 1 << 24);
        let (dst_key, dst_acc, _) = make_element(5, dst_coord, 0, 0);

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
        let src: Element = read_account(&result.resulting_accounts[2].1);
        let dst: Element = read_account(&result.resulting_accounts[3].1);

        assert_eq!(charge.index, dst.index);
        assert_eq!(src.pot, 0);
        assert!(dst.pot >= src_pot);
    }

    #[test]
    fn fails_compress_outward() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let src_index = ElementIndex((5u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, src_index);
        let (src_key, src_acc, _) = make_element(5, src_coord, 0, 100_000);
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
            &[Check::err(ProgramError::Custom(42))],
        );
    }

    #[test]
    fn fails_charge_not_in_source() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let other_index = ElementIndex((10u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, other_index);
        let (src_key, src_acc, _) = make_element(2, src_coord, 0, 100_000);
        let (dst_key, dst_acc, _) = make_element(5, dst_coord, 0, 0);

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
}

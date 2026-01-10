//! Tests for Charge and Discharge instructions.

mod common;
use common::*;

use mollusk_svm::result::Check;
use nucleus::player::{Charge, Wallet};
use nucleus::types::ElementIndex;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

// ============================================================================
// CHARGE INSTRUCTION TESTS
// ============================================================================

mod charge_tests {
    use super::*;

    fn ix_data(amount: u64) -> Vec<u8> {
        let mut data = Vec::with_capacity(16);
        data.extend_from_slice(&IX_CHARGE.to_le_bytes());
        data.extend_from_slice(&amount.to_le_bytes());
        data
    }

    #[test]
    fn success_transfer_from_wallet_to_charge() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 0, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 1_000_000);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(500_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::success()],
        );

        let wallet: Wallet = read_account(&result.resulting_accounts[2].1);
        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        assert_eq!(wallet.balance, 500_000);
        assert_eq!(charge.balance, 500_000);
    }

    #[test]
    fn fails_zero_amount() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 0, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 1_000_000);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(0),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn fails_insufficient_wallet_balance() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 0, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 100);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(1_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::err(ProgramError::ArithmeticOverflow)],
        );
    }

    #[test]
    fn fails_wrong_authority() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let other = Pubkey::new_unique();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 0, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&other, 1_000_000);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(500_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::err(ProgramError::IncorrectAuthority)],
        );
    }

    #[test]
    fn accumulates_to_existing_balance() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 100_000, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 1_000_000);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(200_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        assert_eq!(charge.balance, 300_000);
    }
}

// ============================================================================
// DISCHARGE INSTRUCTION TESTS
// ============================================================================

mod discharge_tests {
    use super::*;

    fn ix_data(amount: u64) -> Vec<u8> {
        let mut data = Vec::with_capacity(16);
        data.extend_from_slice(&IX_DISCHARGE.to_le_bytes());
        data.extend_from_slice(&amount.to_le_bytes());
        data
    }

    #[test]
    fn success_transfer_from_charge_to_wallet() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(500_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::success()],
        );

        let wallet: Wallet = read_account(&result.resulting_accounts[2].1);
        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        assert_eq!(wallet.balance, 500_000);
        assert_eq!(charge.balance, 500_000);
    }

    #[test]
    fn fails_zero_amount() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(0),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn fails_insufficient_charge_balance() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 100, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(1_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::err(ProgramError::InsufficientFunds)],
        );
    }

    #[test]
    fn fails_charge_is_bound() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let index = ElementIndex((1u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, index);
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(500_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::err(ProgramError::Custom(50))],
        );
    }

    #[test]
    fn fails_wrong_authority() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let other = Pubkey::new_unique();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&other, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(500_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::err(ProgramError::IncorrectAuthority)],
        );
    }
}

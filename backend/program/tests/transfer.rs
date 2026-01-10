//! Tests for Charge (wallet→charge) and Discharge (charge→wallet) instructions.

mod common;
use common::*;

use nucleus::player::Wallet;

// ============================================================================
// CHARGE INSTRUCTION TESTS
// ============================================================================

mod charge_tests {
    use super::*;

    fn make_ix_data(amount: u64) -> Vec<u8> {
        ix_data_with_u64(TokamakInstruction::Charge, amount)
    }

    fn metas(
        signer: Pubkey,
        charge: Pubkey,
        wallet: Pubkey,
    ) -> Vec<solana_sdk::instruction::AccountMeta> {
        vec![
            solana_sdk::instruction::AccountMeta::new(signer, true),
            solana_sdk::instruction::AccountMeta::new(charge, false),
            solana_sdk::instruction::AccountMeta::new(wallet, false),
        ]
    }

    /// Verify successful transfer from wallet to charge account
    #[test]
    fn success_transfer_from_wallet_to_charge() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge(&signer.pubkey, 0, ZERO_INDEX);
        let wallet = wallet_min(&signer.pubkey);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(AMT_HALF),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), wallet.into()],
            &[Check::success()],
        );

        assert_charge_bal(&result, 1, AMT_HALF);
        let w: Wallet = read(&result.resulting_accounts[2].1);
        assert_eq!(w.balance, AMT_HALF);
    }

    /// Charge instruction must reject zero amount
    #[test]
    fn fails_zero_amount() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge_min(&signer.pubkey);
        let wallet = wallet_min(&signer.pubkey);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(0),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), wallet.into()],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    /// Charge fails when wallet lacks balance
    #[test]
    fn fails_insufficient_wallet_balance() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge_min(&signer.pubkey);
        let wallet = wallet(&signer.pubkey, 100);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(1_000),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), wallet.into()],
            &[Check::err(ProgramError::ArithmeticOverflow)],
        );
    }

    /// Charge fails when signer doesn't match wallet authority
    #[test]
    fn fails_wrong_authority() {
        let mollusk = mollusk();
        let signer = signer();
        let other = Pubkey::new_unique();
        let charge = charge_min(&signer.pubkey);
        let wallet = wallet_min(&other);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(AMT_HALF),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), wallet.into()],
            &[Check::err(ProgramError::IncorrectAuthority)],
        );
    }

    /// Charge adds to existing charge balance
    #[test]
    fn accumulates_to_existing_balance() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge(&signer.pubkey, 100_000, ZERO_INDEX);
        let wallet = wallet_min(&signer.pubkey);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(200_000),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[signer.into(), charge.into(), wallet.into()],
            &[Check::success()],
        );

        assert_charge_bal(&result, 1, 300_000);
    }
}

// ============================================================================
// DISCHARGE INSTRUCTION TESTS
// ============================================================================

mod discharge_tests {
    use super::*;

    fn make_ix_data(amount: u64) -> Vec<u8> {
        ix_data_with_u64(TokamakInstruction::Discharge, amount)
    }

    fn metas(
        signer: Pubkey,
        charge: Pubkey,
        wallet: Pubkey,
    ) -> Vec<solana_sdk::instruction::AccountMeta> {
        vec![
            solana_sdk::instruction::AccountMeta::new(signer, true),
            solana_sdk::instruction::AccountMeta::new(charge, false),
            solana_sdk::instruction::AccountMeta::new(wallet, false),
        ]
    }

    /// Verify successful transfer from charge to wallet account
    #[test]
    fn success_transfer_from_charge_to_wallet_min() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge(&signer.pubkey, BAL_MIN, ZERO_INDEX);
        let wallet = wallet(&signer.pubkey, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(AMT_HALF),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                charge.into(),
                wallet.into(),
            ],
            &[Check::success()],
        );

        assert_charge_bal(&result, 1, AMT_HALF);
        let w: Wallet = read(&result.resulting_accounts[2].1);
        assert_eq!(w.balance, AMT_HALF);
    }

    /// Discharge instruction must reject zero amount
    #[test]
    fn fails_zero_amount() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge(&signer.pubkey, BAL_MIN, ZERO_INDEX);
        let wallet = wallet(&signer.pubkey, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(0),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                charge.into(),
                wallet.into(),
            ],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    /// Discharge fails when charge lacks balance
    #[test]
    fn fails_insufficient_charge_balance() {
        let mollusk = mollusk();
        let signer = signer();
        let charge = charge(&signer.pubkey, 100, ZERO_INDEX);
        let wallet = wallet(&signer.pubkey, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(1_000),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                charge.into(),
                wallet.into(),
            ],
            &[Check::err(ProgramError::InsufficientFunds)],
        );
    }

    /// Discharge fails when charge is bound to an element (Custom(50))
    #[test]
    fn fails_charge_is_bound() {
        let mollusk = mollusk();
        let signer = signer();
        let index = elem_index(1);
        let charge = charge(&signer.pubkey, BAL_MIN, index);
        let wallet = wallet(&signer.pubkey, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(AMT_HALF),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                charge.into(),
                wallet.into(),
            ],
            &[Check::err(ProgramError::Custom(50))],
        );
    }

    /// Discharge fails when signer doesn't match charge authority
    #[test]
    fn fails_wrong_authority() {
        let mollusk = mollusk();
        let signer = signer();
        let other = Pubkey::new_unique();
        let charge = charge(&signer.pubkey, BAL_MIN, ZERO_INDEX);
        let wallet = wallet(&other, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &make_ix_data(AMT_HALF),
            metas(signer.pubkey, charge.pubkey, wallet.pubkey),
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                signer.into(),
                charge.into(),
                wallet.into(),
            ],
            &[Check::err(ProgramError::IncorrectAuthority)],
        );
    }
}

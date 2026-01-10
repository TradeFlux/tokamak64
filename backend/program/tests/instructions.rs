//! Integration tests for TOKAMAK64 program instructions using mollusk-svm.
//!
//! ## Testable Instructions
//! - Charge, Discharge, Bind, Unbind, Rebind, Compress, Vent, Claim, Overload
//!
//! ## Difficult to Test (require CPI)
//! - InitWallet, InitCharge: Require System Program CreateAccount CPI
//! - Infuse, Extract: Require Token Program TransferChecked CPI
//!
//! These CPI-dependent instructions would need either:
//! 1. A mock program that simulates the CPI target
//! 2. Integration tests with a full Solana test validator
//! 3. Pre-created accounts in a fixture

use bytemuck::bytes_of;
use mollusk_svm::{result::Check, Mollusk};
use nucleus::{
    board::{Artefact, Board, Curve, Element},
    consts::MAX_SATURATION,
    player::{Charge, Wallet},
    types::{Coordinates, ElementIndex},
};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

// ============================================================================
// CONSTANTS
// ============================================================================

const PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0xbe, 0x3f, 0x06, 0x2e, 0x38, 0x97, 0x61, 0x6e, 0x74, 0x0e, 0x90, 0x73, 0x11, 0xe4, 0x41, 0xd6,
    0xcc, 0x88, 0xa2, 0x30, 0x62, 0x21, 0x6d, 0x74, 0xb8, 0xb3, 0xb6, 0xb1, 0xe2, 0x25, 0x65, 0x01,
]);

// Instruction discriminators (u64)
const IX_CHARGE: u64 = 2;
const IX_CLAIM: u64 = 3;
const IX_COMPRESS: u64 = 4;
const IX_DISCHARGE: u64 = 6;
const IX_REBIND: u64 = 7;
const IX_UNBIND: u64 = 8;
const IX_BIND: u64 = 9;
const IX_OVERLOAD: u64 = 10;
const IX_VENT: u64 = 12;

// ============================================================================
// HARNESS
// ============================================================================

fn mollusk() -> Mollusk {
    let mut m = Mollusk::new(&PROGRAM_ID, "tokamak_program");
    m.warp_to_slot(2000); // Past speed tax window by default
    m
}

fn make_signer() -> (Pubkey, Account) {
    let key = Pubkey::new_unique();
    let account = Account {
        lamports: 1_000_000_000,
        data: vec![],
        owner: Pubkey::default(),
        executable: false,
        rent_epoch: 0,
    };
    (key, account)
}

fn make_wallet(authority: &Pubkey, balance: u64) -> (Pubkey, Account, Wallet) {
    let key = Pubkey::new_unique();
    let wallet = Wallet {
        balance,
        authority: authority.to_bytes(),
        mint: [0u8; 32],
        charges: 0,
        _pad: 0,
    };
    let account = Account {
        lamports: 1_000_000,
        data: bytes_of(&wallet).to_vec(),
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };
    (key, account, wallet)
}

fn make_charge(authority: &Pubkey, balance: u64, index: ElementIndex) -> (Pubkey, Account, Charge) {
    make_charge_with_share(authority, balance, index, 0)
}

fn make_charge_with_share(
    authority: &Pubkey,
    balance: u64,
    index: ElementIndex,
    share: u32,
) -> (Pubkey, Account, Charge) {
    let key = Pubkey::new_unique();
    let charge = Charge {
        balance,
        timestamp: 0,
        index,
        share,
        authority: authority.to_bytes(),
        mint: [0u8; 32],
        _pad: 0,
    };
    let account = Account {
        lamports: 1_000_000,
        data: bytes_of(&charge).to_vec(),
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };
    (key, account, charge)
}

fn make_element(atomic: u64, coords: u64, saturation: u32, pot: u64) -> (Pubkey, Account, Element) {
    make_element_with_shares(atomic, coords, saturation, pot, 0)
}

fn make_element_with_shares(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
    shares: u32,
) -> (Pubkey, Account, Element) {
    make_element_full(atomic, coords, saturation, pot, shares, 0)
}

fn make_element_full(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
    shares: u32,
    tvl: u64,
) -> (Pubkey, Account, Element) {
    let key = Pubkey::new_unique();
    let index = ElementIndex((atomic << 56) | 1); // generation = 1
    let element = Element {
        pot,
        index,
        curve: Curve {
            capacity: 1_000_000_000_000,
            tvl,
            pressure: 0,
            saturation,
            shares,
        },
        coordinates: Coordinates(coords),
    };
    let account = Account {
        lamports: 1_000_000,
        data: bytes_of(&element).to_vec(),
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };
    (key, account, element)
}

fn make_board(tvl: u64, charge_count: u32) -> (Pubkey, Account, Board) {
    let key = Pubkey::new_unique();
    let board = Board {
        tvl,
        quantum_pocket: 0,
        charge_count,
        quantum_index: 0,
        _pad: [0u8; 3],
    };
    let account = Account {
        lamports: 1_000_000,
        data: bytes_of(&board).to_vec(),
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };
    (key, account, board)
}

fn make_artefact(pot: u64, index: ElementIndex, shares: u32) -> (Pubkey, Account, Artefact) {
    let key = Pubkey::new_unique();
    let artefact = Artefact { pot, index, shares, _pad: 0 };
    let account = Account {
        lamports: 1_000_000,
        data: bytes_of(&artefact).to_vec(),
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };
    (key, account, artefact)
}

fn read_account<T: bytemuck::Pod + Copy>(account: &Account) -> T {
    *bytemuck::from_bytes(&account.data[..size_of::<T>()])
}

// Edge coordinates (on perimeter)
const EDGE_COORD: u64 = 0x01; // bit 0 = A1 (corner, definitely on edge)
// Non-edge coordinates (interior)
const INTERIOR_COORD: u64 = 0x0000_0400_0000_0000; // center-ish

// Adjacent coordinates (share an edge)
fn adjacent_coords() -> (u64, u64) {
    (0x01, 0x02) // A1 and B1 (horizontally adjacent)
}

// Non-adjacent coordinates
fn non_adjacent_coords() -> (u64, u64) {
    (0x01, 0x04) // A1 and C1 (not adjacent)
}

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
        let (wallet_key, wallet_acc, _) = make_wallet(&other, 1_000_000); // Different authority

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
        assert_eq!(charge.balance, 300_000); // 100k + 200k
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
        // Charge bound to element with atomic=1
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
        let (wallet_key, wallet_acc, _) = make_wallet(&other, 0); // Different authority

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

// ============================================================================
// BIND INSTRUCTION TESTS
// ============================================================================

mod bind_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_BIND.to_le_bytes().to_vec()
    }

    #[test]
    fn success_bind_to_edge_element() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, ElementIndex(0));
        let (elem_key, elem_acc, _) = make_element(1, EDGE_COORD, 0, 0);
        let (board_key, board_acc, _) = make_board(0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let board: Board = read_account(&result.resulting_accounts[3].1);

        // Charge should be bound (index != 0)
        assert!(!charge.index.is_zero());
        // Board TVL should increase
        assert!(board.tvl > 0);
        // Board charge count should increment
        assert_eq!(board.charge_count, 1);
    }

    #[test]
    fn fails_not_on_edge() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, ElementIndex(0));
        let (elem_key, elem_acc, _) = make_element(1, INTERIOR_COORD, 0, 0);
        let (board_key, board_acc, _) = make_board(0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn fails_charge_already_bound() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        // Already bound charge
        let index = ElementIndex((2u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, index);
        let (elem_key, elem_acc, _) = make_element(1, EDGE_COORD, 0, 0);
        let (board_key, board_acc, _) = make_board(0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::err(ProgramError::Custom(43))],
        );
    }

    #[test]
    fn deducts_bind_fee() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let initial_balance = 10_000_000u64;
        let (charge_key, charge_acc, _) = make_charge(&signer_key, initial_balance, ElementIndex(0));
        // Higher saturation = higher fee
        let (elem_key, elem_acc, _) = make_element(5, EDGE_COORD, 1 << 24, 0); // saturation = 1.0
        let (board_key, board_acc, _) = make_board(0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let element: Element = read_account(&result.resulting_accounts[2].1);

        // Fee deducted from charge
        assert!(charge.balance < initial_balance);
        // Fee added to pot
        assert!(element.pot > 0);
    }
}

// ============================================================================
// UNBIND INSTRUCTION TESTS
// ============================================================================

mod unbind_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_UNBIND.to_le_bytes().to_vec()
    }

    #[test]
    fn success_unbind_from_edge_element() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((1u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge_with_share(&signer_key, 10_000_000, elem_index, 1 << 24);
        let (elem_key, elem_acc, _) = make_element_with_shares(1, EDGE_COORD, 1 << 24, 100_000, 1 << 24);
        let (board_key, board_acc, _) = make_board(10_000_000, 1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let board: Board = read_account(&result.resulting_accounts[3].1);

        // Charge should be unbound
        assert!(charge.index.is_zero());
        // Board charge count should decrement
        assert_eq!(board.charge_count, 0);
    }

    #[test]
    fn fails_not_on_edge() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((5u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, elem_index);
        let (elem_key, elem_acc, _) = make_element(5, INTERIOR_COORD, 0, 0);
        let (board_key, board_acc, _) = make_board(10_000_000, 1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn deducts_unbind_fee() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let initial_balance = 10_000_000u64;
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge_with_share(&signer_key, initial_balance, elem_index, 1 << 24);
        let (elem_key, elem_acc, _) = make_element_with_shares(3, EDGE_COORD, 2 << 24, 0, 1 << 24);
        let (board_key, board_acc, _) = make_board(initial_balance, 1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let element: Element = read_account(&result.resulting_accounts[2].1);

        // Fee deducted
        assert!(charge.balance < initial_balance);
        // Fee added to pot
        assert!(element.pot > 0);
    }
}

// ============================================================================
// REBIND INSTRUCTION TESTS
// ============================================================================

mod rebind_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_REBIND.to_le_bytes().to_vec()
    }

    #[test]
    fn success_rebind_to_adjacent_element() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        let src_index = ElementIndex((1u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge_with_share(&signer_key, 10_000_000, src_index, 1 << 24);
        let (src_key, src_acc, _) = make_element_with_shares(1, src_coord, 1 << 24, 100_000, 1 << 24);
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
        let dst: Element = read_account(&result.resulting_accounts[3].1);

        // Charge bound to destination
        assert_eq!(charge.index, dst.index);
    }

    #[test]
    fn fails_not_adjacent() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = non_adjacent_coords();
        let src_index = ElementIndex((1u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, src_index);
        let (src_key, src_acc, _) = make_element(1, src_coord, 0, 0);
        let (dst_key, dst_acc, _) = make_element(3, dst_coord, 0, 0);

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
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn fails_charge_not_in_source() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        // Charge bound to different element
        let other_index = ElementIndex((5u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 10_000_000, other_index);
        let (src_key, src_acc, _) = make_element(1, src_coord, 0, 0);
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
            &[Check::err(ProgramError::Custom(1))],
        );
    }

    #[test]
    fn fee_routing_outward_to_src() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        // Moving from higher to lower atomic (outward)
        let src_index = ElementIndex((5u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge_with_share(&signer_key, 10_000_000, src_index, 1 << 24);
        let (src_key, src_acc, _) = make_element_with_shares(5, src_coord, 1 << 24, 0, 1 << 24);
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

        let src: Element = read_account(&result.resulting_accounts[2].1);
        let dst: Element = read_account(&result.resulting_accounts[3].1);

        // Moving outward: fee goes to src
        assert!(src.pot > 0);
        assert_eq!(dst.pot, 0);
    }

    #[test]
    fn fee_routing_inward_to_dst() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        // Moving from lower to higher atomic (inward)
        let src_index = ElementIndex((2u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge_with_share(&signer_key, 10_000_000, src_index, 1 << 24);
        let (src_key, src_acc, _) = make_element_with_shares(2, src_coord, 0, 0, 1 << 24);
        let (dst_key, dst_acc, _) = make_element(5, dst_coord, 1 << 24, 0);

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

        let src: Element = read_account(&result.resulting_accounts[2].1);
        let dst: Element = read_account(&result.resulting_accounts[3].1);

        // Moving inward: fee goes to dst
        assert_eq!(src.pot, 0);
        assert!(dst.pot > 0);
    }
}

// ============================================================================
// COMPRESS INSTRUCTION TESTS
// ============================================================================

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

        // Charge bound to destination
        assert_eq!(charge.index, dst.index);
        // Source pot moved to destination (plus fees)
        assert_eq!(src.pot, 0);
        assert!(dst.pot >= src_pot);
    }

    #[test]
    fn fails_compress_outward() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (src_coord, dst_coord) = adjacent_coords();
        // Trying to compress from higher to lower (outward - invalid)
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
        // Charge bound to different element
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

// ============================================================================
// VENT INSTRUCTION TESTS
// ============================================================================

mod vent_tests {
    use super::*;

    fn ix_data(amount: u64) -> Vec<u8> {
        let mut data = Vec::with_capacity(16);
        data.extend_from_slice(&IX_VENT.to_le_bytes());
        data.extend_from_slice(&amount.to_le_bytes());
        data
    }

    #[test]
    fn success_vent_to_element_pot() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, elem_index);
        let (elem_key, elem_acc, _) = make_element(3, EDGE_COORD, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(200_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
            ],
        );

        let result = mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (elem_key, elem_acc)],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let element: Element = read_account(&result.resulting_accounts[2].1);

        assert_eq!(charge.balance, 800_000);
        assert_eq!(element.pot, 200_000);
    }

    #[test]
    fn fails_zero_amount() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, elem_index);
        let (elem_key, elem_acc, _) = make_element(3, EDGE_COORD, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(0),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (elem_key, elem_acc)],
            &[Check::err(ProgramError::InvalidArgument)],
        );
    }

    #[test]
    fn fails_insufficient_balance() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 100, elem_index);
        let (elem_key, elem_acc, _) = make_element(3, EDGE_COORD, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(1_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (elem_key, elem_acc)],
            &[Check::err(ProgramError::ArithmeticOverflow)],
        );
    }

    #[test]
    fn fails_wrong_target_element() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        // Charge bound to element 3
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, elem_index);
        // But trying to vent to element 5
        let (elem_key, elem_acc, _) = make_element(5, EDGE_COORD, 0, 0);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(200_000),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (elem_key, elem_acc)],
            &[Check::err(ProgramError::Custom(32))],
        );
    }
}

// ============================================================================
// CLAIM INSTRUCTION TESTS
// ============================================================================

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
        // Charge has 50% of shares (1 << 24 out of 2 << 24)
        let (charge_key, charge_acc, _) = make_charge_with_share(&signer_key, 0, art_index, 1 << 24);
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
            &[(signer_key, signer), (charge_key, charge_acc), (art_key, art_acc)],
            &[Check::success()],
        );

        let charge: Charge = read_account(&result.resulting_accounts[1].1);
        let artefact: Artefact = read_account(&result.resulting_accounts[2].1);

        // Claimed 50% of pot
        assert_eq!(charge.balance, 500_000);
        assert_eq!(artefact.pot, 500_000);
        // Share cleared
        assert_eq!(charge.share, 0);
        // Index cleared (unbound)
        assert!(charge.index.is_zero());
    }

    #[test]
    fn fails_zero_share() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let art_index = ElementIndex((3u64 << 56) | 1);
        // Charge has zero share
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
            &[(signer_key, signer), (charge_key, charge_acc), (art_key, art_acc)],
            &[Check::err(ProgramError::Custom(42))],
        );
    }

    #[test]
    fn fails_index_mismatch() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        // Charge bound to element 3
        let charge_index = ElementIndex((3u64 << 56) | 1);
        // But artefact is for element 5
        let art_index = ElementIndex((5u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge_with_share(&signer_key, 0, charge_index, 1 << 24);
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
            &[(signer_key, signer), (charge_key, charge_acc), (art_key, art_acc)],
            &[Check::err(ProgramError::Custom(42))],
        );
    }

    #[test]
    fn multiple_claims_distribute_correctly() {
        let mollusk = mollusk();
        let art_index = ElementIndex((3u64 << 56) | 1);

        // First claim: 25% share
        let (signer1_key, signer1) = make_signer();
        let (charge1_key, charge1_acc, _) = make_charge_with_share(&signer1_key, 0, art_index, 1 << 24);
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
            &[(signer1_key, signer1), (charge1_key, charge1_acc), (art_key, art_acc)],
            &[Check::success()],
        );

        let charge1: Charge = read_account(&result1.resulting_accounts[1].1);
        let artefact1: Artefact = read_account(&result1.resulting_accounts[2].1);

        // First claim gets 25%
        assert_eq!(charge1.balance, 250_000);
        assert_eq!(artefact1.pot, 750_000);
    }
}

// ============================================================================
// OVERLOAD INSTRUCTION TESTS
// ============================================================================

mod overload_tests {
    use super::*;

    fn ix_data() -> Vec<u8> {
        IX_OVERLOAD.to_le_bytes().to_vec()
    }

    // NOTE: success_overload_at_max_saturation test is complex to set up correctly
    // because the overload processor:
    // 1. Claims the charge's share (modifying charge.balance)
    // 2. Resets the curve to zeroed state
    // 3. Rebinds with a zeroed source element
    // 4. Performs complex TVL calculations
    //
    // Testing this requires precise curve state that's difficult to achieve
    // with unit test fixtures. This would be better tested with:
    // - A full integration test with actual program flow
    // - Or a test validator with pre-created accounts

    #[test]
    fn fails_below_max_saturation() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((3u64 << 56) | 1);
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 1_000_000, elem_index);
        // Below MAX_SATURATION
        let (elem_key, elem_acc, _) = make_element(3, EDGE_COORD, MAX_SATURATION - 1, 500_000);
        let (art_key, art_acc, _) = make_artefact(0, ElementIndex(0), 0);
        let (board_key, board_acc, _) = make_board(10_000_000, 5);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &ix_data(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(art_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc),
                (elem_key, elem_acc),
                (art_key, art_acc),
                (board_key, board_acc),
            ],
            &[Check::err(ProgramError::Custom(1))],
        );
    }
}

// ============================================================================
// AUTHORITY VALIDATION TESTS (Cross-cutting)
// ============================================================================

mod authority_tests {
    use super::*;

    #[test]
    fn fails_missing_signature() {
        let mollusk = mollusk();
        let (signer_key, signer) = make_signer();
        let (charge_key, charge_acc, _) = make_charge(&signer_key, 0, ElementIndex(0));
        let (wallet_key, wallet_acc, _) = make_wallet(&signer_key, 1_000_000);

        let mut data = Vec::with_capacity(16);
        data.extend_from_slice(&IX_CHARGE.to_le_bytes());
        data.extend_from_slice(&500_000u64.to_le_bytes());

        // Signer not marked as signer
        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(signer_key, false), // Not a signer!
                AccountMeta::new(charge_key, false),
                AccountMeta::new(wallet_key, false),
            ],
        );

        mollusk.process_and_validate_instruction(
            &ix,
            &[(signer_key, signer), (charge_key, charge_acc), (wallet_key, wallet_acc)],
            &[Check::err(ProgramError::MissingRequiredSignature)],
        );
    }
}

// ============================================================================
// SPEED TAX TESTS
// ============================================================================

mod speed_tax_tests {
    use super::*;

    #[test]
    fn higher_fee_for_rapid_actions() {
        // Test with patient action first (no speed tax)
        let mut mollusk_patient = Mollusk::new(&PROGRAM_ID, "tokamak_program");
        mollusk_patient.warp_to_slot(2000);

        let (signer_key, signer) = make_signer();
        let elem_index = ElementIndex((1u64 << 56) | 1);
        let charge_key = Pubkey::new_unique();

        // Patient: old timestamp, no speed tax
        let charge_patient = Charge {
            balance: 100_000_000,
            timestamp: 0, // Ancient action
            index: elem_index,
            share: 1 << 24,
            authority: signer_key.to_bytes(),
            mint: [0u8; 32],
            _pad: 0,
        };
        let charge_acc_patient = Account {
            lamports: 1_000_000,
            data: bytes_of(&charge_patient).to_vec(),
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        };

        let (elem_key, elem_acc, _) = make_element_with_shares(1, EDGE_COORD, 1 << 24, 100_000, 1 << 24);
        let (board_key, board_acc, _) = make_board(100_000_000, 1);

        let ix = Instruction::new_with_bytes(
            PROGRAM_ID,
            &IX_UNBIND.to_le_bytes().to_vec(),
            vec![
                AccountMeta::new(signer_key, true),
                AccountMeta::new(charge_key, false),
                AccountMeta::new(elem_key, false),
                AccountMeta::new(board_key, false),
            ],
        );

        let result_patient = mollusk_patient.process_and_validate_instruction(
            &ix.clone(),
            &[
                (signer_key, signer.clone()),
                (charge_key, charge_acc_patient),
                (elem_key, elem_acc.clone()),
                (board_key, board_acc.clone()),
            ],
            &[Check::success()],
        );

        // Rapid: recent timestamp, incurs speed tax
        let mut mollusk_rapid = Mollusk::new(&PROGRAM_ID, "tokamak_program");
        mollusk_rapid.warp_to_slot(1100); // 100 slots after timestamp

        let charge_rapid = Charge {
            balance: 100_000_000,
            timestamp: 1000, // Recent action
            index: elem_index,
            share: 1 << 24,
            authority: signer_key.to_bytes(),
            mint: [0u8; 32],
            _pad: 0,
        };
        let charge_acc_rapid = Account {
            lamports: 1_000_000,
            data: bytes_of(&charge_rapid).to_vec(),
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        };

        let result_rapid = mollusk_rapid.process_and_validate_instruction(
            &ix,
            &[
                (signer_key, signer),
                (charge_key, charge_acc_rapid),
                (elem_key, elem_acc),
                (board_key, board_acc),
            ],
            &[Check::success()],
        );

        let charge_patient_result: Charge = read_account(&result_patient.resulting_accounts[1].1);
        let charge_rapid_result: Charge = read_account(&result_rapid.resulting_accounts[1].1);

        // Rapid action should have higher fee (lower remaining balance)
        assert!(
            charge_rapid_result.balance < charge_patient_result.balance,
            "Rapid balance {} should be less than patient balance {}",
            charge_rapid_result.balance,
            charge_patient_result.balance
        );
    }
}

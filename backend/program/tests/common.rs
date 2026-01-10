//! Common test utilities for TOKAMAK64 program integration tests.
#![allow(dead_code, unused_imports)]

use mollusk_svm::result::InstructionResult;
use nucleus::{
    board::{Artefact, Board, Curve, Element},
    player::{Charge, Wallet},
    types::{Coordinates, ElementIndex},
};

// Re-exports for test files
pub use bytemuck::bytes_of;
pub use mollusk_svm::{result::Check, Mollusk};
pub use nucleus::consts::MAX_SATURATION;
pub use solana_sdk::instruction::{AccountMeta, Instruction};
pub use solana_sdk::program_error::ProgramError;
pub use solana_sdk::{account::Account, pubkey::Pubkey};
pub use tokamak_program::instruction::TokamakInstruction;

// ============================================================================
// CONSTANTS
// ============================================================================

pub const PROGRAM_ID: Pubkey = tokamak_program::ID;

pub const EDGE_COORD: u64 = 0x01;
pub const INTERIOR_COORD: u64 = 0x0000_0400_0000_0000;

/// Zero element index for unbound charges
pub const ZERO_INDEX: ElementIndex = ElementIndex(0);

// ----------------------------------------------------------------------------
// Test Value Constants
// ----------------------------------------------------------------------------

/// Standard balance amounts
pub const BAL_MIN: u64 = 1_000_000;
pub const BAL_HIGH: u64 = 10_000_000;
pub const BAL_MAX: u64 = 100_000_000;

/// Common operation amounts
pub const AMT_HALF: u64 = 500_000;
pub const AMT_QUARTER: u64 = 250_000;

/// Share values (fixed-point 24.8 format)
pub const SHARE_ONE: u32 = 1 << 24;
pub const SHARE_TWO: u32 = 2 << 24;
pub const SHARE_FOUR: u32 = 4 << 24;

/// Account lamports (standard for test accounts)
pub const LAMPORTS: u64 = 1_000_000;
pub const SIGNER_LAMPORTS: u64 = 1_000_000_000;

// ============================================================================
// TYPES
// ============================================================================

/// Pair of Pubkey and Account for test setup
#[derive(Clone)]
pub struct AccountWithPubkey {
    pub pubkey: Pubkey,
    pub account: Account,
}

impl From<(Pubkey, Account)> for AccountWithPubkey {
    fn from((pubkey, account): (Pubkey, Account)) -> Self {
        Self { pubkey, account }
    }
}

impl From<AccountWithPubkey> for (Pubkey, Account) {
    fn from(value: AccountWithPubkey) -> Self {
        (value.pubkey, value.account)
    }
}

/// Creates 3-value tuple (key, account, Wallet) and wraps key+account
impl<T> From<(Pubkey, Account, T)> for AccountWithPubkey {
    fn from((pubkey, account, _): (Pubkey, Account, T)) -> Self {
        Self { pubkey, account }
    }
}

// ============================================================================
// TEST HARNESS
// ============================================================================

pub fn mollusk() -> Mollusk {
    let mut m = Mollusk::default();
    let elf_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../target/deploy/tokamak_program.so");
    let elf = std::fs::read(&elf_path).expect("Run `cargo build-sbf` first.");
    m.add_program_with_loader_and_elf(&PROGRAM_ID, &solana_sdk::bpf_loader::id(), &elf);
    m.warp_to_slot(2000);
    m
}

// ============================================================================
// ACCOUNT FACTORIES
// ============================================================================

/// Base account config for program-owned accounts
fn program_account(data: Vec<u8>) -> Account {
    Account {
        lamports: LAMPORTS,
        data,
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    }
}

/// Creates signer account with high lamports
pub fn signer() -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let account = Account {
        lamports: SIGNER_LAMPORTS,
        data: vec![],
        owner: Pubkey::default(),
        executable: false,
        rent_epoch: 0,
    };
    (key, account).into()
}

/// Creates wallet account
pub fn wallet(authority: &Pubkey, balance: u64) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let w = Wallet {
        balance,
        authority: authority.to_bytes(),
        mint: [0u8; 32],
        charges: 0,
        _pad: 0,
    };
    let data = bytes_of(&w).to_vec();
    (key, program_account(data)).into()
}

/// Creates wallet with BAL_MIN (common default)
pub fn wallet_min(authority: &Pubkey) -> AccountWithPubkey {
    wallet(authority, BAL_MIN)
}

/// Creates charge account (zero share) - internal
fn charge_full(authority: &Pubkey, balance: u64, index: ElementIndex) -> AccountWithPubkey {
    charge_full_with_share(authority, balance, index, 0)
}

/// Creates charge account with share - internal
fn charge_full_with_share(
    authority: &Pubkey,
    balance: u64,
    index: ElementIndex,
    share: u32,
) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let c = Charge {
        balance,
        timestamp: 0,
        index,
        share,
        authority: authority.to_bytes(),
        mint: [0u8; 32],
        _pad: 0,
    };
    let data = bytes_of(&c).to_vec();
    AccountWithPubkey {
        pubkey: key,
        account: program_account(data),
    }
}

/// Creates charge account (zero share)
pub fn charge(authority: &Pubkey, balance: u64, index: ElementIndex) -> AccountWithPubkey {
    charge_full(authority, balance, index)
}

/// Creates charge account with share
pub fn charge_with_share(
    authority: &Pubkey,
    balance: u64,
    index: ElementIndex,
    share: u32,
) -> AccountWithPubkey {
    charge_full_with_share(authority, balance, index, share)
}

/// Creates charge account with BAL_MIN and ZERO_INDEX (common default)
pub fn charge_min(authority: &Pubkey) -> AccountWithPubkey {
    charge(authority, BAL_MIN, ZERO_INDEX)
}

/// Creates charge account with BAL_HIGH and ZERO_INDEX (common default)
pub fn charge_high(authority: &Pubkey) -> AccountWithPubkey {
    charge(authority, BAL_HIGH, ZERO_INDEX)
}

/// Creates charge account with BAL_HIGH and index (common default)
pub fn charge_high_with_index(authority: &Pubkey, index: ElementIndex) -> AccountWithPubkey {
    charge(authority, BAL_HIGH, index)
}

/// Creates charge account with BAL_HIGH, index, and SHARE_ONE (common default)
pub fn charge_shared(authority: &Pubkey, index: ElementIndex) -> AccountWithPubkey {
    charge_with_share(authority, BAL_HIGH, index, SHARE_ONE)
}

// ============================================================================
// ELEMENT FACTORIES
// ============================================================================

/// Creates element account with full config (internal)
fn element_full(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
    shares: u32,
    tvl: u64,
) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let e = Element {
        pot,
        index: ElementIndex((atomic << 56) | 1),
        curve: Curve {
            capacity: 1_000_000_000_000,
            tvl,
            pressure: 0,
            saturation,
            shares,
        },
        coordinates: Coordinates(coords),
    };
    let data = bytes_of(&e).to_vec();
    AccountWithPubkey {
        pubkey: key,
        account: program_account(data),
    }
}

/// Creates element account with shares
pub fn element_with_shares(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
    shares: u32,
) -> AccountWithPubkey {
    element_full(atomic, coords, saturation, pot, shares, 0)
}

/// Creates element account
pub fn element(atomic: u64, coords: u64, saturation: u32, pot: u64) -> AccountWithPubkey {
    element_with_shares(atomic, coords, saturation, pot, 0)
}

/// Creates element at edge with default values (common default)
pub fn element_edge(atomic: u64) -> AccountWithPubkey {
    element(atomic, EDGE_COORD, 0, 0)
}

/// Creates element at edge with saturation (common default)
pub fn element_edge_sat(atomic: u64, saturation: u32) -> AccountWithPubkey {
    element(atomic, EDGE_COORD, saturation, 0)
}

/// Creates element at edge with shares (common default)
pub fn element_edge_shared(atomic: u64, shares: u32) -> AccountWithPubkey {
    element_with_shares(atomic, EDGE_COORD, 0, 0, shares)
}

/// Creates element at custom coordinates (no shares)
pub fn element_at(atomic: u64, coords: u64) -> AccountWithPubkey {
    element(atomic, coords, 0, 0)
}

/// Creates element at custom coordinates with pot and shares
pub fn element_with_shares_at(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
    shares: u32,
) -> AccountWithPubkey {
    element_with_shares(atomic, coords, saturation, pot, shares)
}

// ============================================================================
// BOARD FACTORIES
// ============================================================================

/// Creates board account - internal
fn board_tuple(tvl: u64, charge_count: u32) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let b = Board {
        tvl,
        quantum_pocket: 0,
        charge_count,
        quantum_index: 0,
        _pad: [0u8; 3],
    };
    let data = bytes_of(&b).to_vec();
    AccountWithPubkey {
        pubkey: key,
        account: program_account(data),
    }
}

/// Creates board account
pub fn board(tvl: u64, charge_count: u32) -> AccountWithPubkey {
    board_tuple(tvl, charge_count)
}

/// Creates board with 0 tvl and 0 charge_count (common default)
pub fn board_empty() -> AccountWithPubkey {
    board(0, 0)
}

/// Creates board with BAL_HIGH tvl and charge_count (common default)
pub fn board_with_count(charge_count: u32) -> AccountWithPubkey {
    board(BAL_HIGH, charge_count)
}

// ============================================================================
// ARTEFACT FACTORIES
// ============================================================================

/// Creates artefact account with index and shares - internal
fn artefact_full_tuple(pot: u64, index: ElementIndex, shares: u32) -> AccountWithPubkey {
    let key = Pubkey::new_unique();
    let a = Artefact {
        pot,
        index,
        shares,
        _pad: 0,
    };
    let data = bytes_of(&a).to_vec();
    AccountWithPubkey {
        pubkey: key,
        account: program_account(data),
    }
}

/// Creates artefact account - internal
fn artefact_tuple(pot: u64) -> AccountWithPubkey {
    artefact_full_tuple(pot, ZERO_INDEX, 0)
}

/// Creates artefact account with index and shares
pub fn artefact_full(pot: u64, index: ElementIndex, shares: u32) -> AccountWithPubkey {
    artefact_full_tuple(pot, index, shares)
}

/// Creates artefact account (index and shares default to zero)
pub fn artefact(pot: u64) -> AccountWithPubkey {
    artefact_full(pot, ZERO_INDEX, 0)
}

// ============================================================================
// UTILITIES
// ============================================================================

/// Reads account data into type T
pub fn read<T: bytemuck::Pod + Copy>(account: &Account) -> T {
    *bytemuck::from_bytes(&account.data[..size_of::<T>()])
}

/// Adjacent element coordinates for testing
pub fn adjacent_coords() -> (u64, u64) {
    (0x01, 0x02)
}

/// Non-adjacent element coordinates for testing
pub fn non_adjacent_coords() -> (u64, u64) {
    (0x01, 0x04)
}

/// Creates element index from atomic number
pub fn elem_index(atomic: u64) -> ElementIndex {
    ElementIndex((atomic << 56) | 1)
}

// ============================================================================
// INSTRUCTION BUILDERS
// ============================================================================

/// Creates instruction data from discriminator
pub fn ix_data(ix: TokamakInstruction) -> Vec<u8> {
    (ix as u64).to_le_bytes().to_vec()
}

/// Creates instruction data with u64 payload
pub fn ix_data_with_u64(ix: TokamakInstruction, value: u64) -> Vec<u8> {
    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&(ix as u64).to_le_bytes());
    data.extend_from_slice(&value.to_le_bytes());
    data
}

/// Account metas: signer + charge + element + board
pub fn metas_charge_elem(
    signer: Pubkey,
    charge: Pubkey,
    element: Pubkey,
    board: Pubkey,
) -> Vec<AccountMeta> {
    vec![
        AccountMeta::new(signer, true),
        AccountMeta::new(charge, false),
        AccountMeta::new(element, false),
        AccountMeta::new(board, false),
    ]
}

/// Account metas: signer + charge + source + destination
pub fn metas_charge_src_dst(
    signer: Pubkey,
    charge: Pubkey,
    src: Pubkey,
    dst: Pubkey,
) -> Vec<AccountMeta> {
    vec![
        AccountMeta::new(signer, true),
        AccountMeta::new(charge, false),
        AccountMeta::new(src, false),
        AccountMeta::new(dst, false),
    ]
}

/// Account metas: signer + charge + artefact
pub fn metas_charge_art(signer: Pubkey, charge: Pubkey, artefact: Pubkey) -> Vec<AccountMeta> {
    vec![
        AccountMeta::new(signer, true),
        AccountMeta::new(charge, false),
        AccountMeta::new(artefact, false),
    ]
}

// ============================================================================
// ASSERTIONS
// ============================================================================

/// Asserts charge balance at result index
pub fn assert_charge_bal(result: &InstructionResult, idx: usize, expected: u64) {
    let c: Charge = read(&result.resulting_accounts[idx].1);
    assert_eq!(
        c.balance, expected,
        "Expected charge balance {}, got {}",
        expected, c.balance
    );
}

/// Asserts element pot at result index
pub fn assert_pot(result: &InstructionResult, idx: usize, expected: u64) {
    let e: Element = read(&result.resulting_accounts[idx].1);
    assert_eq!(
        e.pot, expected,
        "Expected element pot {}, got {}",
        expected, e.pot
    );
}

/// Asserts board charge count at result index
pub fn assert_count(result: &InstructionResult, idx: usize, expected: u32) {
    let b: Board = read(&result.resulting_accounts[idx].1);
    assert_eq!(
        b.charge_count, expected,
        "Expected charge_count {}, got {}",
        expected, b.charge_count
    );
}

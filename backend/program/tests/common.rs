//! Common test utilities for TOKAMAK64 program integration tests.
#![allow(dead_code)]

use bytemuck::bytes_of;
use mollusk_svm::Mollusk;
use nucleus::{
    board::{Artefact, Board, Curve, Element},
    player::{Charge, Wallet},
    types::{Coordinates, ElementIndex},
};
use solana_sdk::{account::Account, pubkey::Pubkey};

// ============================================================================
// CONSTANTS
// ============================================================================

pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0xbe, 0x3f, 0x06, 0x2e, 0x38, 0x97, 0x61, 0x6e, 0x74, 0x0e, 0x90, 0x73, 0x11, 0xe4, 0x41, 0xd6,
    0xcc, 0x88, 0xa2, 0x30, 0x62, 0x21, 0x6d, 0x74, 0xb8, 0xb3, 0xb6, 0xb1, 0xe2, 0x25, 0x65, 0x01,
]);

pub const IX_CHARGE: u64 = 2;
pub const IX_CLAIM: u64 = 3;
pub const IX_COMPRESS: u64 = 4;
pub const IX_DISCHARGE: u64 = 6;
pub const IX_REBIND: u64 = 7;
pub const IX_UNBIND: u64 = 8;
pub const IX_BIND: u64 = 9;
pub const IX_OVERLOAD: u64 = 10;
pub const IX_VENT: u64 = 12;

pub const EDGE_COORD: u64 = 0x01;
pub const INTERIOR_COORD: u64 = 0x0000_0400_0000_0000;

// ============================================================================
// TEST HARNESS
// ============================================================================

pub fn mollusk() -> Mollusk {
    let mut m = Mollusk::default();
    // Load program from the build output
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let elf_path = manifest_dir.join("../../target/sbpf-solana-solana/release/tokamak_program.so");
    let elf = std::fs::read(&elf_path)
        .unwrap_or_else(|_| std::fs::read(manifest_dir.join("../target/sbpf-solana-solana/release/tokamak_program.so"))
        .unwrap_or_else(|_| std::fs::read("/Users/babur/Code/dev/tokamak/backend/target/sbpf-solana-solana/release/tokamak_program.so")
        .expect("Failed to read program file. Run `cargo build-sbf` first.")));
    m.add_program_with_loader_and_elf(&PROGRAM_ID, &solana_sdk::bpf_loader::id(), &elf);
    m.warp_to_slot(2000);
    m
}

pub fn make_signer() -> (Pubkey, Account) {
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

pub fn make_wallet(authority: &Pubkey, balance: u64) -> (Pubkey, Account, Wallet) {
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

pub fn make_charge(
    authority: &Pubkey,
    balance: u64,
    index: ElementIndex,
) -> (Pubkey, Account, Charge) {
    make_charge_with_share(authority, balance, index, 0)
}

pub fn make_charge_with_share(
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

pub fn make_element(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
) -> (Pubkey, Account, Element) {
    make_element_with_shares(atomic, coords, saturation, pot, 0)
}

pub fn make_element_with_shares(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
    shares: u32,
) -> (Pubkey, Account, Element) {
    make_element_full(atomic, coords, saturation, pot, shares, 0)
}

pub fn make_element_full(
    atomic: u64,
    coords: u64,
    saturation: u32,
    pot: u64,
    shares: u32,
    tvl: u64,
) -> (Pubkey, Account, Element) {
    let key = Pubkey::new_unique();
    let index = ElementIndex((atomic << 56) | 1);
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

pub fn make_board(tvl: u64, charge_count: u32) -> (Pubkey, Account, Board) {
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

pub fn make_artefact(pot: u64, index: ElementIndex, shares: u32) -> (Pubkey, Account, Artefact) {
    let key = Pubkey::new_unique();
    let artefact = Artefact {
        pot,
        index,
        shares,
        _pad: 0,
    };
    let account = Account {
        lamports: 1_000_000,
        data: bytes_of(&artefact).to_vec(),
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    };
    (key, account, artefact)
}

pub fn read_account<T: bytemuck::Pod + Copy>(account: &Account) -> T {
    *bytemuck::from_bytes(&account.data[..size_of::<T>()])
}

pub fn adjacent_coords() -> (u64, u64) {
    (0x01, 0x02)
}

pub fn non_adjacent_coords() -> (u64, u64) {
    (0x01, 0x04)
}

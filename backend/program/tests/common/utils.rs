//! Utility functions: deserialization, coordinates, assertions, result accessors.

use super::prelude::*;
use nucleus::{
    board::{Board, Element},
    player::Charge,
    types::ElementIndex,
};

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

/// Asserts charge balance at result index
pub fn assert_charge_bal(
    result: &mollusk_svm::result::InstructionResult,
    idx: usize,
    expected: u64,
) {
    let c: Charge = read(&result.resulting_accounts[idx].1);
    assert_eq!(
        c.balance, expected,
        "Expected charge balance {}, got {}",
        expected, c.balance
    );
}

/// Asserts element pot at result index
pub fn assert_pot(result: &mollusk_svm::result::InstructionResult, idx: usize, expected: u64) {
    let e: Element = read(&result.resulting_accounts[idx].1);
    assert_eq!(
        e.pot, expected,
        "Expected element pot {}, got {}",
        expected, e.pot
    );
}

/// Asserts board charge count at result index
pub fn assert_count(result: &mollusk_svm::result::InstructionResult, idx: usize, expected: u32) {
    let b: Board = read(&result.resulting_accounts[idx].1);
    assert_eq!(
        b.charge_count, expected,
        "Expected charge_count {}, got {}",
        expected, b.charge_count
    );
}

/// Extension for easy result account access
pub trait ResultExt {
    fn get<T: bytemuck::Pod + Copy>(&self, idx: usize) -> T;
}

impl ResultExt for mollusk_svm::result::InstructionResult {
    fn get<T: bytemuck::Pod + Copy>(&self, idx: usize) -> T {
        read(&self.resulting_accounts[idx].1)
    }
}

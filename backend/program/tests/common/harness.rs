//! Test harness - mollusk setup and test runner macro.

use super::prelude::*;
use super::constants::PROGRAM_ID;

/// Creates the test mollusk instance
pub fn mollusk() -> Mollusk {
    let mut m = Mollusk::default();
    let elf_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../target/deploy/tokamak_program.so");
    let elf = std::fs::read(&elf_path).expect("Run `cargo build-sbf` first.");
    m.add_program_with_loader_and_elf(&PROGRAM_ID, &solana_sdk::bpf_loader::id(), &elf);
    m.warp_to_slot(2000);
    m
}

/// Execute instruction with accounts and checks
#[macro_export]
macro_rules! test_run {
    ($ix:expr, $accounts:expr, $checks:expr) => {
        $crate::common::mollusk().process_and_validate_instruction(&$ix, $accounts, $checks)
    };
}

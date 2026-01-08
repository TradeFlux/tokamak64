//! # Program: Solana on-chain entrypoint and instruction processing for TOKAMAK64.
//!
//! This crate defines the on-chain program logic, instruction dispatch, and account interaction handlers.

use instruction::TokamakInstruction;
use pinocchio::{account::AccountView, address::declare_id, program_entrypoint, ProgramResult};

use crate::instruction::IxData;

mod accounts;
mod addresses;
mod instruction;
mod processors;

program_entrypoint!(process_instruction);
declare_id!("DuJrE9ZB4TqcMByw9g4CiDQdNQosPQCQw2ECWGfLiyi");

fn process_instruction(
    id: &pinocchio::Address,
    accounts: &[AccountView],
    data: &[u8],
) -> ProgramResult {
    use processors::*;
    use TokamakInstruction::*;

    let it = &mut accounts.iter();
    let mut data = IxData::new(data);
    let ix = TokamakInstruction::parse(&mut data)?;
    match ix {
        InitCharge => init::charge(it, data),
        InitWallet => init::wallet(it, data),
        Charge => charge::charge(it, data),
        Claim => claim::claim(it),
        Compress => compress::compress(it),
        Extract => extract::extract(it, data),
        Discharge => discharge::discharge(it, data),
        Rebind => rebind::rebind(it),
        Eject => eject::eject(it),
        Inject => inject::inject(it),
        Overload => overload::overload(it),
        Infuse => infuse::infuse(it, data),
        Vent => vent::vent(it, data),
    }
}

use instruction::TokamakInstruction;
use pinocchio::{account::AccountView, entrypoint, error::ProgramError, ProgramResult};

mod accounts;
mod instruction;
mod processors;

fn process_instruction(
    _program_id: &pinocchio::Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let ix = match instruction_data[0] {
        0 => TokamakInstruction::Charge,
        1 => TokamakInstruction::Claim,
        2 => TokamakInstruction::Compress,
        3 => TokamakInstruction::Discharge,
        4 => TokamakInstruction::Translate,
        5 => TokamakInstruction::Fiss,
        6 => TokamakInstruction::Fuse,
        7 => TokamakInstruction::Overload,
        8 => TokamakInstruction::Vent,
        _ => return Err(ProgramError::InvalidInstructionData),
    };

    let mut it = accounts.iter();
    match ix {
        TokamakInstruction::Charge => processors::charge::process_charge(&mut it),
        TokamakInstruction::Claim => processors::claim::process_claim(&mut it),
        TokamakInstruction::Compress => processors::compress::process_compress(&mut it),
        TokamakInstruction::Discharge => processors::discharge::process_discharge(&mut it),
        TokamakInstruction::Translate => processors::translate::process_translation(&mut it),
        TokamakInstruction::Fiss => processors::fission::process_fission(&mut it),
        TokamakInstruction::Fuse => processors::fuse::process_fuse(&mut it),
        TokamakInstruction::Overload => processors::overload::process_overload(&mut it),
        TokamakInstruction::Vent => processors::vent::process_vent(&mut it),
    }
}

entrypoint!(process_instruction);

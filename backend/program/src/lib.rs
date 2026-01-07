mod accounts;
mod instruction;
mod processors;
mod state;

use instruction::TokamakInstruction;
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, ProgramResult,
};

fn process_instruction(
    _program_id: &[u8; 32],
    accounts: &[AccountInfo],
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
        4 => TokamakInstruction::Drift,
        5 => TokamakInstruction::Fission,
        6 => TokamakInstruction::Fuse,
        7 => TokamakInstruction::Overload,
        8 => TokamakInstruction::Vent,
        _ => return Err(ProgramError::InvalidInstructionData),
    };

    let mut it = accounts.iter();
    match ix {
        TokamakInstruction::Fuse => processors::fuse::process_fuse(&mut it),
        TokamakInstruction::Fission => processors::fission::process_fission(&mut it),
        TokamakInstruction::Drift => processors::drift::process_drift(&mut it),
        TokamakInstruction::Compress => processors::compress::process_compress(&mut it),
        TokamakInstruction::Claim => processors::claim::process_claim(&mut it),
        TokamakInstruction::Vent => processors::vent::process_vent(&mut it),
        TokamakInstruction::Charge => Err(ProgramError::InvalidInstructionData),
        TokamakInstruction::Discharge => Err(ProgramError::InvalidInstructionData),
        TokamakInstruction::Overload => Err(ProgramError::InvalidInstructionData),
    }
}

entrypoint!(process_instruction);

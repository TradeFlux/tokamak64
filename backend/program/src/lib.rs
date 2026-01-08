use instruction::TokamakInstruction;
use pinocchio::{account::AccountView, program_entrypoint, ProgramResult};

use crate::instruction::IxData;

mod accounts;
mod instruction;
mod processors;

program_entrypoint!(process_instruction);

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
        Charge => charge::process_charge(it, data),
        Claim => claim::process_claim(it),
        Compress => compress::process_compress(it),
        Drain => drain::process_drain(it, data),
        Discharge => discharge::process_discharge(it, data),
        Translate => translate::process_translation(it),
        Fiss => fission::process_fission(it),
        Fuse => fuse::process_fuse(it),
        Overload => overload::process_overload(it),
        TopUp => topup::process_topup(it, data),
        Vent => vent::process_vent(it, data),
    }
}

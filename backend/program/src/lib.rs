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
        Charge => charge::charge(it, data),
        Claim => claim::claim(it),
        Compress => compress::compress(it),
        Drain => drain::drain(it, data),
        Discharge => discharge::discharge(it, data),
        Rebind => rebind::rebind(it),
        Fiss => fission::fission(it),
        Fuse => fuse::fuse(it),
        Overload => overload::overload(it),
        TopUp => topup::topup(it, data),
        Vent => vent::vent(it, data),
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TokamakInstruction {
    Deposit,
    Withdraw,
    Redeem,

    Enter,
    Shift,
    Compress,
    Donate,
    Exit,
}

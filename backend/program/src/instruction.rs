#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TokamakInstruction {
    Compress,
    Deposit,
    Donate,
    Fission,
    Fuse,
    Overload,
    Redeem,
    Shift,
    Withdraw,
}

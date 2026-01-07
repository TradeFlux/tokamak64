#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TokamakInstruction {
    /// Deposit stable tokens into system, funding the wallet
    Charge,
    /// Claim the shares worth gluon from the pot of previously reset element
    Claim,
    /// Move the pot inward merging with others
    Compress,
    /// Withdraw the funds from wallet, by converting GLUON to original stable token
    Discharge,
    /// Move the charge from one element to another
    Translate,
    /// Exit the board
    Fiss,
    /// Enter the board
    Fuse,
    /// Push the element beyond curve capacity, triggering a reset
    Overload,
    /// Donate some of the charge's balance to the pot of the current element
    Vent,
}

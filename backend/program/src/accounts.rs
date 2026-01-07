use nucleus::{
    board::{Board, Element, Tombstone},
    player::Charge,
};

pub struct EnterAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) dst: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

pub struct ExitAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

pub struct ShiftAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) dst: &'a mut Element,
}

pub struct OverloadAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) dst: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

pub struct DonateAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) target: &'a mut Element,
}

pub struct RedeemAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) tombstone: &'a mut Tombstone,
}

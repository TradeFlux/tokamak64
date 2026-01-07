use nucleus::{
    board::{Board, Element, Tombstone},
    player::Charge,
};

pub struct FusionAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) dst: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

pub struct FissionAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) board: &'a mut Board,
}

pub struct ShiftAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) src: &'a mut Element,
    pub(crate) dst: &'a mut Element,
}

pub struct CompressionAccounts<'a> {
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

pub struct LeakAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) target: &'a mut Element,
}

pub struct RedeemAccounts<'a> {
    pub(crate) charge: &'a mut Charge,
    pub(crate) artefact: &'a mut Tombstone,
}

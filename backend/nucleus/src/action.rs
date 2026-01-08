//! Core game actions: rebinding, claiming rewards, compressing pots, and venting value.

use curve::math::dx_for_dc;

use crate::{
    board::{Artefact, Curve, Element},
    consts::MAX_SATURATION,
    player::Charge,
    round_divide,
    types::Q824,
};

/// Rebind a charge from one element to another: unbind from src, bind to dst.
/// Updates saturation and pressure on both elements.
pub fn rebind(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    // Unbind: negate balance to get withdrawal delta.
    let withdrawal = charge.balance.wrapping_neg(); // Two's complement negation
    update_curve(withdrawal, src);
    // Bind: positive delta to destination, capture new share.
    charge.share = update_curve(charge.balance, dst);
    charge.index = dst.index;
}

/// Apply a charge delta to an element's curve. Updates saturation and pressure.
/// Delta is signed (two's complement u64): positive = deposit, negative = withdrawal.
fn update_curve(charge_delta: u64, elem: &mut Element) -> Q824 {
    if elem.index.is_zero() {
        return 0;
    }
    let Curve {
        capacity,
        saturation,
        pressure,
        ..
    } = elem.curve;
    let (share, pressure_delta) = dx_for_dc(saturation, pressure, charge_delta, capacity as u64);
    elem.curve.saturation += share;
    elem.curve.pressure += pressure_delta;
    elem.curve.tvl += charge_delta;
    share
}

/// Claim a shareholder's proportional share from a reset element's pot.
/// Distributes reward based on share value and updates charge state.
pub fn claim(charge: &mut Charge, artefact: &mut Artefact) {
    let reward = round_divide(artefact.pot, charge.share as u64, MAX_SATURATION as u64);
    charge.balance += reward;
    artefact.pot -= reward;
    charge.share = 0;
    charge.index.clear();
}

/// Compress an element inward: rebind charge and consolidate pot to deeper element.
/// Transfers accumulated pot from src to dst.
pub fn compress(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    rebind(charge, src, dst);
    dst.pot += src.pot;
    src.pot = 0;
}

use curve::math::dx_for_dc;

use crate::{
    board::{Curve, Element, Tombstone},
    consts::MAX_X,
    mul_div_round_nearest,
    player::Charge,
    types::Q824,
};

/// Move a charge from one element to another (unbind then bind).
pub fn translate(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    let balance = charge.balance as i64;
    transition(-balance, src);
    charge.share = transition(balance, dst);
    charge.index = dst.index;
}

/// Bind a charge to an element (enter the board).
fn transition(balance: i64, elem: &mut Element) -> Q824 {
    if elem.index.zero() {
        return 0;
    }
    let Curve {
        capacity,
        position,
        state,
        ..
    } = elem.curve;
    // Convert signed balance to u64 two's complement
    let dc = balance as u64;
    let (share, ds) = dx_for_dc(position, state, dc, capacity as u64);
    elem.curve.position = elem.curve.position.wrapping_add(share);
    elem.curve.state = elem.curve.state.wrapping_add(ds);
    elem.curve.volume += balance;
    share
}

/// Claim a share of a reset element's pot.
pub fn claim(charge: &mut Charge, artefact: &mut Tombstone) {
    let reward = mul_div_round_nearest(artefact.pot, charge.share as i64, MAX_X as i64);
    charge.balance += reward;
    artefact.pot -= reward;
    charge.share = 0;
    charge.index.clear();
}

/// Move a pot inward (to a deeper element) while rebinding the charge.
pub fn compress(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    translate(charge, src, dst);
    dst.pot += src.pot;
    src.pot = 0;
}

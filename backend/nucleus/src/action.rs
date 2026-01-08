use curve::math::dx_for_dc;

use crate::{
    board::{Curve, Element, Tombstone},
    consts::MAX_X,
    player::Charge,
    round_div,
    types::Q824,
};

/// Move a charge from one element to another (unbind then bind).
pub fn translate(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    // Unbind from src: negate balance to get withdrawal delta
    transition(charge.balance.wrapping_neg(), src);
    // Bind to dst: use positive balance delta
    charge.share = transition(charge.balance, dst);
    charge.index = dst.index;
}

/// Bind a charge to an element (enter the board).
///
/// The `dc` parameter is already in unsigned two's complement representation,
/// so positive values increase the curve position and negative values decrease it.
fn transition(dc: u64, elem: &mut Element) -> Q824 {
    if elem.index.zero() {
        return 0;
    }
    let Curve {
        capacity,
        position,
        state,
        ..
    } = elem.curve;
    let (share, ds) = dx_for_dc(position, state, dc, capacity as u64);
    elem.curve.position = elem.curve.position.wrapping_add(share);
    elem.curve.state = elem.curve.state.wrapping_add(ds);
    elem.curve.volume = elem.curve.volume.wrapping_add(dc);
    share
}

/// Claim a share of a reset element's pot.
pub fn claim(charge: &mut Charge, artefact: &mut Tombstone) {
    let reward = round_div(artefact.pot, charge.share as u64, MAX_X as u64);
    charge.balance = charge.balance.wrapping_add(reward);
    artefact.pot = artefact.pot.wrapping_sub(reward);
    charge.share = 0;
    charge.index.clear();
}

/// Move a pot inward (to a deeper element) while rebinding the charge.
pub fn compress(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    translate(charge, src, dst);
    dst.pot += src.pot;
    src.pot = 0;
}

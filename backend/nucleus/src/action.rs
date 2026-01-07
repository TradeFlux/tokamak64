use curve::math::dx_for_dc;

use crate::{
    board::{Curve, Element, Tombstone},
    consts::MAX_X,
    mul_div_round_nearest,
    player::Charge,
};

/// Move a charge from one element to another (unbind then bind).
pub fn drift(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    fission(charge, src);
    fuse(charge, dst);
}

/// Bind a charge to an element (enter the board).
pub fn fuse(charge: &mut Charge, dst: &mut Element) {
    let Curve {
        capacity,
        position,
        state,
        ..
    } = dst.curve;
    let (share, state_delta) = dx_for_dc(position, state, charge.balance as i64, capacity);
    charge.share = share;
    dst.curve.position += share;
    dst.curve.state += state_delta;
    charge.index = dst.index;
}

/// Unbind a charge from an element (leave the board).
pub fn fission(charge: &mut Charge, src: &mut Element) {
    let Curve {
        capacity,
        position,
        state,
        ..
    } = src.curve;
    let (share, state_delta) = dx_for_dc(position, state, -(charge.balance as i64), capacity);
    src.curve.position += share;
    src.curve.state += state_delta;
    charge.index.clear();
}

/// Claim a share of a reset element's pot.
pub fn claim(charge: &mut Charge, tomb: &mut Tombstone) {
    let reward = mul_div_round_nearest(tomb.pot, charge.share as u64, MAX_X);
    charge.balance += reward;
    tomb.pot -= reward;
    charge.share = 0;
    charge.index.clear();
}

/// Move a pot inward (to a deeper element) while rebinding the charge.
pub fn compress(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    drift(charge, src, dst);
    dst.pot += src.pot;
    src.pot = 0;
}

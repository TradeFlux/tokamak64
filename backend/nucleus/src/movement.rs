use curve::math::dx_for_dc;

use crate::{
    board::{Curve, Element},
    player::Charge,
};

pub fn shift(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    fuse(charge, dst);
    fission(charge, src);
}

pub fn fuse(charge: &mut Charge, dst: &mut Element) {
    let Curve { x, s, capacity } = dst.curve;
    let (share, ds) = dx_for_dc(x, s, charge.balance as i64, capacity);
    charge.share = share;
    dst.curve.x += share;
    dst.curve.s += ds;
}

pub fn fission(charge: &mut Charge, src: &mut Element) {
    let Curve { x, s, capacity } = src.curve;
    let (share, ds) = dx_for_dc(x, s, -(charge.balance as i64), capacity);
    src.curve.x += share;
    src.curve.s += ds;
}

use curve::math::dx_for_dc;

use crate::{
    board::{Curve, Element},
    player::Charge,
};

pub fn shift(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    fission(charge, src);
    fuse(charge, dst);
}

pub fn fuse(charge: &mut Charge, dst: &mut Element) {
    let Curve { x, s, capacity } = dst.curve;
    let (share, ds) = dx_for_dc(x, s, charge.balance as i64, capacity);
    charge.share = share;
    dst.curve.x += share;
    dst.curve.s += ds;
    charge.index = dst.index;
}

pub fn fission(charge: &mut Charge, src: &mut Element) {
    let Curve { x, s, capacity } = src.curve;
    let (share, ds) = dx_for_dc(x, s, -(charge.balance as i64), capacity);
    src.curve.x += share;
    src.curve.s += ds;
    charge.index.clear();
}

pub fn compress(charge: &mut Charge, src: &mut Element, dst: &mut Element) {
    shift(charge, src, dst);
    dst.pot += src.pot;
    src.pot = 0;
}

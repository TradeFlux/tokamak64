use crate::{
    board::{Curve, Element},
    consts::{MAX_DELTA_TS, MAX_SPEED_MULTIPLIER, MAX_X, MAX_Z, MIN_FEE},
    mul_div_round_nearest,
    player::Charge,
    types::Gluon,
};

pub fn shift_fee(charge: &Charge, src: &Element, dst: &Element) -> Gluon {
    let src_z = src.index.atomic_number();
    let dst_z = dst.index.atomic_number();
    let delta_z = src_z.abs_diff(dst_z);
    if src.index > dst.index {
        fee(charge, &src.curve, delta_z)
    } else {
        fee(charge, &dst.curve, delta_z)
    }
}

pub fn fusion_fee(charge: &Charge, dst: &Element) -> Gluon {
    fee(charge, &dst.curve, dst.index.atomic_number())
}

pub fn fission_fee(charge: &Charge, src: &Element) -> Gluon {
    fee(charge, &src.curve, src.index.atomic_number())
}

fn fee(charge: &Charge, curve: &Curve, delta_z: u64) -> Gluon {
    const DIV: u64 = MAX_Z * MAX_X;

    let mul = delta_z * curve.position as u64;
    mul_div_round_nearest(charge.balance, mul, DIV).max(MIN_FEE)
}

pub fn compression_fee(src: &Element) -> Gluon {
    const DIV: u64 = MAX_X * 100;
    let mul = src.curve.position as u64 * 5;
    mul_div_round_nearest(src.pot, mul, DIV).max(MIN_FEE)
}

pub fn speed_multiplier(charge: &Charge, timestamp: u64) -> u64 {
    const DIV: u64 = MAX_DELTA_TS.pow(2);

    let elapsed = timestamp.saturating_sub(charge.timestamp);
    let mul = elapsed.min(MAX_DELTA_TS).pow(2);
    1 + mul_div_round_nearest(MAX_SPEED_MULTIPLIER, mul, DIV)
}

use crate::{
    board::{Curve, Element},
    consts::{MAX_DELTA_TS, MAX_SPEED_MULTIPLIER, MAX_X, MAX_Z, MIN_FEE},
    player::Charge,
    round_div,
    types::Gluon,
};

pub fn translation_fee(charge: &Charge, src: &Element, dst: &Element) -> Gluon {
    let src_z = src.index.atomic_number();
    let dst_z = dst.index.atomic_number();
    let delta_z = dst_z.wrapping_sub(src_z);
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
    const DIV: u64 = MAX_Z * MAX_X as u64;
    let mul = (delta_z as u64) * (curve.position as u64);
    let result = round_div(charge.balance as u64, mul, DIV);
    result.max(MIN_FEE)
}

pub fn compression_fee(src: &Element) -> Gluon {
    let div = (MAX_X as u64) * 100u64;
    let mul = (src.curve.position as u64) * 5u64;
    let result = round_div(src.pot as u64, mul, div);
    result.max(MIN_FEE)
}

pub fn speed_multiplier(charge: &Charge, timestamp: u64) -> u64 {
    const DIV: u64 = (MAX_DELTA_TS).pow(2);
    let elapsed = timestamp.saturating_sub(charge.timestamp);
    let mul = elapsed.min(MAX_DELTA_TS).pow(2);
    1 + round_div(MAX_SPEED_MULTIPLIER as u64, mul, DIV)
}

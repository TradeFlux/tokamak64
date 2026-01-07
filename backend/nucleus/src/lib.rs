pub mod action;
pub mod board;
pub mod consts;
pub mod fees;
pub mod player;
pub mod types;

#[inline]
fn mul_div_round_nearest(mul1: u64, mul2: u64, div: u64) -> u64 {
    let product = (mul1 as u128) * (mul2 as u128);
    let divisor = div as u128;
    ((product + divisor / 2) / divisor) as u64
}

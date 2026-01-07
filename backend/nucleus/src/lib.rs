pub mod action;
pub mod board;
pub mod consts;
pub mod fees;
pub mod player;
pub mod types;

#[inline]
fn mul_div_round_nearest(mul1: i64, mul2: i64, div: i64) -> i64 {
    let product = (mul1 as i128) * (mul2 as i128);
    let divisor = div as i128;
    ((product + divisor / 2) / divisor) as i64
}

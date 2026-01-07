use nucleus::{action, fees::translation_fee};
use pinocchio::error::ProgramError;
use pinocchio::ProgramResult;

use crate::accounts::{AccountIter, FromAccounts, TranslationAccounts};

pub(crate) fn process_translation<'a, I: AccountIter<'a>>(it: &mut I) -> ProgramResult {
    let TranslationAccounts { charge, src, dst } = TranslationAccounts::parse(it)?;
    let fee = translation_fee(charge, src, dst);

    let remainder = charge.balance.checked_sub(fee);
    charge.balance = remainder.ok_or(ProgramError::ArithmeticOverflow)?;
    action::translate(charge, src, dst);
    if src.index > dst.index {
        src.pot += fee;
    } else {
        dst.pot += fee;
    }

    Ok(())
}

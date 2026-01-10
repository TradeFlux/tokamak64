//! Common types for test setup.

use super::prelude::*;

/// Pair of Pubkey and Account for test setup
#[derive(Clone)]
pub struct AccountWithPubkey {
    pub pubkey: Pubkey,
    pub account: Account,
}

impl From<(Pubkey, Account)> for AccountWithPubkey {
    fn from((pubkey, account): (Pubkey, Account)) -> Self {
        Self { pubkey, account }
    }
}

impl From<AccountWithPubkey> for (Pubkey, Account) {
    fn from(value: AccountWithPubkey) -> Self {
        (value.pubkey, value.account)
    }
}

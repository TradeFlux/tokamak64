//! Instruction and account meta builder macros.

/// Build instruction from variant and metas
#[macro_export]
macro_rules! ix {
    ($variant:expr, $value:expr, $metas:expr) => {
        {
            let mut data = ::std::vec::Vec::new();
            data.extend_from_slice(&($variant as u64).to_le_bytes());
            data.extend_from_slice(&$value.to_le_bytes());
            ::solana_sdk::instruction::Instruction::new_with_bytes(
                tokamak_program::ID,
                &data,
                $metas,
            )
        }
    };
    ($variant:expr, $metas:expr) => {
        {
            let data = ($variant as u64).to_le_bytes().to_vec();
            ::solana_sdk::instruction::Instruction::new_with_bytes(
                tokamak_program::ID,
                &data,
                $metas,
            )
        }
    };
}

/// Build account metas: first arg is signer (writable), rest are writable
#[macro_export]
macro_rules! metas {
    ($signer:expr) => {
        vec![::solana_sdk::instruction::AccountMeta::new($signer.pubkey, true)]
    };
    ($signer:expr, $($rest:expr),+ $(,)?) => {
        vec![
            ::solana_sdk::instruction::AccountMeta::new($signer.pubkey, true),
            $(::solana_sdk::instruction::AccountMeta::new($rest.pubkey, false),)+
        ]
    };
}

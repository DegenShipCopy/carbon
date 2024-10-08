
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2435e7b807b505ee")]
pub struct SaberAddDecimals{
}

pub struct SaberAddDecimalsInstructionAccounts {
    pub add_decimals_program: solana_sdk::pubkey::Pubkey,
    pub wrapper: solana_sdk::pubkey::Pubkey,
    pub wrapper_mint: solana_sdk::pubkey::Pubkey,
    pub wrapper_underlying_tokens: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub user_underlying_tokens: solana_sdk::pubkey::Pubkey,
    pub user_wrapped_tokens: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SaberAddDecimals {
    type ArrangedAccounts = SaberAddDecimalsInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let add_decimals_program = accounts.get(0)?;
        let wrapper = accounts.get(1)?;
        let wrapper_mint = accounts.get(2)?;
        let wrapper_underlying_tokens = accounts.get(3)?;
        let owner = accounts.get(4)?;
        let user_underlying_tokens = accounts.get(5)?;
        let user_wrapped_tokens = accounts.get(6)?;
        let token_program = accounts.get(7)?;

        Some(SaberAddDecimalsInstructionAccounts {
            add_decimals_program: *add_decimals_program,
            wrapper: *wrapper,
            wrapper_mint: *wrapper_mint,
            wrapper_underlying_tokens: *wrapper_underlying_tokens,
            owner: *owner,
            user_underlying_tokens: *user_underlying_tokens,
            user_wrapped_tokens: *user_wrapped_tokens,
            token_program: *token_program,
        })
    }
}

use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x35924408127511b9")]
pub struct DeleteTokenBadge{
}

pub struct DeleteTokenBadgeInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpools_config_extension: solana_sdk::pubkey::Pubkey,
    pub token_badge_authority: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_badge: solana_sdk::pubkey::Pubkey,
    pub receiver: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for DeleteTokenBadge {
    type ArrangedAccounts = DeleteTokenBadgeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let whirlpools_config_extension = accounts.get(1)?;
        let token_badge_authority = accounts.get(2)?;
        let token_mint = accounts.get(3)?;
        let token_badge = accounts.get(4)?;
        let receiver = accounts.get(5)?;

        Some(DeleteTokenBadgeInstructionAccounts {
            whirlpools_config: *whirlpools_config,
            whirlpools_config_extension: *whirlpools_config_extension,
            token_badge_authority: *token_badge_authority,
            token_mint: *token_mint,
            token_badge: *token_badge,
            receiver: *receiver,
        })
    }
}
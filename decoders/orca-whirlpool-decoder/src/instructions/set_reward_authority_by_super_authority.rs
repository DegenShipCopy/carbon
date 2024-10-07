
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf09ac9c6945d3819")]
pub struct SetRewardAuthorityBySuperAuthority{
    pub reward_index: u8,
}

pub struct SetRewardAuthorityBySuperAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_sdk::pubkey::Pubkey,
    pub new_reward_authority: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetRewardAuthorityBySuperAuthority {
    type ArrangedAccounts = SetRewardAuthorityBySuperAuthorityInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let whirlpool = accounts.get(1)?;
        let reward_emissions_super_authority = accounts.get(2)?;
        let new_reward_authority = accounts.get(3)?;

        Some(SetRewardAuthorityBySuperAuthorityInstructionAccounts {
            whirlpools_config: *whirlpools_config,
            whirlpool: *whirlpool,
            reward_emissions_super_authority: *reward_emissions_super_authority,
            new_reward_authority: *new_reward_authority,
        })
    }
}
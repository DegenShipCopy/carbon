
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x9ae6fa0decd14bdf")]
pub struct UpdateFeesAndRewards{
}

pub struct UpdateFeesAndRewardsInstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for UpdateFeesAndRewards {
    type ArrangedAccounts = UpdateFeesAndRewardsInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpool = accounts.get(0)?;
        let position = accounts.get(1)?;
        let tick_array_lower = accounts.get(2)?;
        let tick_array_upper = accounts.get(3)?;

        Some(UpdateFeesAndRewardsInstructionAccounts {
            whirlpool: *whirlpool,
            position: *position,
            tick_array_lower: *tick_array_lower,
            tick_array_upper: *tick_array_upper,
        })
    }
}
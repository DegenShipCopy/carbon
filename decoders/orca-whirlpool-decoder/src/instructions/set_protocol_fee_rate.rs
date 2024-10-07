
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5f0704329a4f9c83")]
pub struct SetProtocolFeeRate{
    pub protocol_fee_rate: u16,
}

pub struct SetProtocolFeeRateInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetProtocolFeeRate {
    type ArrangedAccounts = SetProtocolFeeRateInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let whirlpool = accounts.get(1)?;
        let fee_authority = accounts.get(2)?;

        Some(SetProtocolFeeRateInstructionAccounts {
            whirlpools_config: *whirlpools_config,
            whirlpool: *whirlpool,
            fee_authority: *fee_authority,
        })
    }
}
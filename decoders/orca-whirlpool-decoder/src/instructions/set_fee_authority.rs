
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1f013257ed656184")]
pub struct SetFeeAuthority{
}

pub struct SetFeeAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub new_fee_authority: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetFeeAuthority {
    type ArrangedAccounts = SetFeeAuthorityInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let fee_authority = accounts.get(1)?;
        let new_fee_authority = accounts.get(2)?;

        Some(SetFeeAuthorityInstructionAccounts {
            whirlpools_config: *whirlpools_config,
            fee_authority: *fee_authority,
            new_fee_authority: *new_fee_authority,
        })
    }
}
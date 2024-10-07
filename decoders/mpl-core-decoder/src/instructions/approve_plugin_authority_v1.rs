
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x374224f0fa496724")]
pub struct ApprovePluginAuthorityV1{
    pub approve_plugin_authority_v1_args: ApprovePluginAuthorityV1Args,
}

pub struct ApprovePluginAuthorityV1InstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for ApprovePluginAuthorityV1 {
    type ArrangedAccounts = ApprovePluginAuthorityV1InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let asset = accounts.get(0)?;
        let collection = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let log_wrapper = accounts.get(5)?;

        Some(ApprovePluginAuthorityV1InstructionAccounts {
            asset: *asset,
            collection: *collection,
            payer: *payer,
            authority: *authority,
            system_program: *system_program,
            log_wrapper: *log_wrapper,
        })
    }
}
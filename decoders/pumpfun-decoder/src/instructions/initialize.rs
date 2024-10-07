
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize{
}

pub struct InitializeInstructionAccounts {
    pub global: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let global = accounts.get(0)?;
        let user = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(InitializeInstructionAccounts {
            global: *global,
            user: *user,
            system_program: *system_program,
        })
    }
}
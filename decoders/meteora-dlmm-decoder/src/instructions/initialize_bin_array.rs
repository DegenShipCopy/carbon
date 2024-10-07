
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x235613b94ed44bd3")]
pub struct InitializeBinArray{
    pub index: i64,
}

pub struct InitializeBinArrayInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializeBinArray {
    type ArrangedAccounts = InitializeBinArrayInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let bin_array = accounts.get(1)?;
        let funder = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(InitializeBinArrayInstructionAccounts {
            lb_pair: *lb_pair,
            bin_array: *bin_array,
            funder: *funder,
            system_program: *system_program,
        })
    }
}
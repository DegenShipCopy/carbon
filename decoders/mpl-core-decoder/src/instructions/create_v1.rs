
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1176a4f2c899957d")]
pub struct CreateV1{
    pub create_v1_args: CreateV1Args,
}

pub struct CreateV1InstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CreateV1 {
    type ArrangedAccounts = CreateV1InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let asset = accounts.get(0)?;
        let collection = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let owner = accounts.get(4)?;
        let update_authority = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let log_wrapper = accounts.get(7)?;

        Some(CreateV1InstructionAccounts {
            asset: *asset,
            collection: *collection,
            authority: *authority,
            payer: *payer,
            owner: *owner,
            update_authority: *update_authority,
            system_program: *system_program,
            log_wrapper: *log_wrapper,
        })
    }
}
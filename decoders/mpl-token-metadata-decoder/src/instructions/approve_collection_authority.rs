
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfe88d02741421b6f")]
pub struct ApproveCollectionAuthority{
}

pub struct ApproveCollectionAuthorityInstructionAccounts {
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
    pub new_collection_authority: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for ApproveCollectionAuthority {
    type ArrangedAccounts = ApproveCollectionAuthorityInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let collection_authority_record = accounts.get(0)?;
        let new_collection_authority = accounts.get(1)?;
        let update_authority = accounts.get(2)?;
        let payer = accounts.get(3)?;
        let metadata = accounts.get(4)?;
        let mint = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent = accounts.get(7)?;

        Some(ApproveCollectionAuthorityInstructionAccounts {
            collection_authority_record: *collection_authority_record,
            new_collection_authority: *new_collection_authority,
            update_authority: *update_authority,
            payer: *payer,
            metadata: *metadata,
            mint: *mint,
            system_program: *system_program,
            rent: *rent,
        })
    }
}
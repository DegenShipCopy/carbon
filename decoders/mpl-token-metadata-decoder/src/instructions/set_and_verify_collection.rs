
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xebf279d89eeab4ea")]
pub struct SetAndVerifyCollection{
}

pub struct SetAndVerifyCollectionInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub collection_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition_account: solana_sdk::pubkey::Pubkey,
    pub collection_authority_record: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetAndVerifyCollection {
    type ArrangedAccounts = SetAndVerifyCollectionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let collection_authority = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let update_authority = accounts.get(3)?;
        let collection_mint = accounts.get(4)?;
        let collection = accounts.get(5)?;
        let collection_master_edition_account = accounts.get(6)?;
        let collection_authority_record = accounts.get(7)?;

        Some(SetAndVerifyCollectionInstructionAccounts {
            metadata: *metadata,
            collection_authority: *collection_authority,
            payer: *payer,
            update_authority: *update_authority,
            collection_mint: *collection_mint,
            collection: *collection,
            collection_master_edition_account: *collection_master_edition_account,
            collection_authority_record: *collection_authority_record,
        })
    }
}
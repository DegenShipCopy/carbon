
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x85a18d3078c65896")]
pub struct Verify{
    pub verification_args: VerificationArgs,
}

pub struct VerifyInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub delegate_record: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub collection_mint: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub collection_master_edition: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Verify {
    type ArrangedAccounts = VerifyInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let delegate_record = accounts.get(1)?;
        let metadata = accounts.get(2)?;
        let collection_mint = accounts.get(3)?;
        let collection_metadata = accounts.get(4)?;
        let collection_master_edition = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let sysvar_instructions = accounts.get(7)?;

        Some(VerifyInstructionAccounts {
            authority: *authority,
            delegate_record: *delegate_record,
            metadata: *metadata,
            collection_mint: *collection_mint,
            collection_metadata: *collection_metadata,
            collection_master_edition: *collection_master_edition,
            system_program: *system_program,
            sysvar_instructions: *sysvar_instructions,
        })
    }
}
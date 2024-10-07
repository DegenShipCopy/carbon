
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6892f2d1b0aeb9a3")]
pub struct Utilize{
    pub utilize_args: UtilizeArgs,
}

pub struct UtilizeInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub use_authority: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub ata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub use_authority_record: solana_sdk::pubkey::Pubkey,
    pub burner: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Utilize {
    type ArrangedAccounts = UtilizeInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let token_account = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let use_authority = accounts.get(3)?;
        let owner = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let ata_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;
        let use_authority_record = accounts.get(9)?;
        let burner = accounts.get(10)?;

        Some(UtilizeInstructionAccounts {
            metadata: *metadata,
            token_account: *token_account,
            mint: *mint,
            use_authority: *use_authority,
            owner: *owner,
            token_program: *token_program,
            ata_program: *ata_program,
            system_program: *system_program,
            rent: *rent,
            use_authority_record: *use_authority_record,
            burner: *burner,
        })
    }
}
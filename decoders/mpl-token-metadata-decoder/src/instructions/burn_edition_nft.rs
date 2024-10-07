
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdd69c440a41b5dc5")]
pub struct BurnEditionNft{
}

pub struct BurnEditionNftInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub print_edition_mint: solana_sdk::pubkey::Pubkey,
    pub master_edition_mint: solana_sdk::pubkey::Pubkey,
    pub print_edition_token_account: solana_sdk::pubkey::Pubkey,
    pub master_edition_token_account: solana_sdk::pubkey::Pubkey,
    pub master_edition_account: solana_sdk::pubkey::Pubkey,
    pub print_edition_account: solana_sdk::pubkey::Pubkey,
    pub edition_marker_account: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for BurnEditionNft {
    type ArrangedAccounts = BurnEditionNftInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let print_edition_mint = accounts.get(2)?;
        let master_edition_mint = accounts.get(3)?;
        let print_edition_token_account = accounts.get(4)?;
        let master_edition_token_account = accounts.get(5)?;
        let master_edition_account = accounts.get(6)?;
        let print_edition_account = accounts.get(7)?;
        let edition_marker_account = accounts.get(8)?;
        let spl_token_program = accounts.get(9)?;

        Some(BurnEditionNftInstructionAccounts {
            metadata: *metadata,
            owner: *owner,
            print_edition_mint: *print_edition_mint,
            master_edition_mint: *master_edition_mint,
            print_edition_token_account: *print_edition_token_account,
            master_edition_token_account: *master_edition_token_account,
            master_edition_account: *master_edition_account,
            print_edition_account: *print_edition_account,
            edition_marker_account: *edition_marker_account,
            spl_token_program: *spl_token_program,
        })
    }
}
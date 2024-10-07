
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2f9de2b40cf02147")]
pub struct InitializeBinArrayBitmapExtension{
}

pub struct InitializeBinArrayBitmapExtensionInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializeBinArrayBitmapExtension {
    type ArrangedAccounts = InitializeBinArrayBitmapExtensionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let bin_array_bitmap_extension = accounts.get(1)?;
        let funder = accounts.get(2)?;
        let system_program = accounts.get(3)?;
        let rent = accounts.get(4)?;

        Some(InitializeBinArrayBitmapExtensionInstructionAccounts {
            lb_pair: *lb_pair,
            bin_array_bitmap_extension: *bin_array_bitmap_extension,
            funder: *funder,
            system_program: *system_program,
            rent: *rent,
        })
    }
}
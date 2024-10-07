
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc26b90097e8f3579")]
pub struct DeprecatedMintPrintingTokens{
    pub mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
}

pub struct DeprecatedMintPrintingTokensInstructionAccounts {
    pub destination: solana_sdk::pubkey::Pubkey,
    pub printing_mint: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for DeprecatedMintPrintingTokens {
    type ArrangedAccounts = DeprecatedMintPrintingTokensInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let destination = accounts.get(0)?;
        let printing_mint = accounts.get(1)?;
        let update_authority = accounts.get(2)?;
        let metadata = accounts.get(3)?;
        let master_edition = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let rent = accounts.get(6)?;

        Some(DeprecatedMintPrintingTokensInstructionAccounts {
            destination: *destination,
            printing_mint: *printing_mint,
            update_authority: *update_authority,
            metadata: *metadata,
            master_edition: *master_edition,
            token_program: *token_program,
            rent: *rent,
        })
    }
}
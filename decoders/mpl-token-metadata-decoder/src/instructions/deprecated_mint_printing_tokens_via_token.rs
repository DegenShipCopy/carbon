
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x54229885913004df")]
pub struct DeprecatedMintPrintingTokensViaToken{
    pub mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
}

pub struct DeprecatedMintPrintingTokensViaTokenInstructionAccounts {
    pub destination: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub one_time_printing_authorization_mint: solana_sdk::pubkey::Pubkey,
    pub printing_mint: solana_sdk::pubkey::Pubkey,
    pub burn_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for DeprecatedMintPrintingTokensViaToken {
    type ArrangedAccounts = DeprecatedMintPrintingTokensViaTokenInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let destination = accounts.get(0)?;
        let token = accounts.get(1)?;
        let one_time_printing_authorization_mint = accounts.get(2)?;
        let printing_mint = accounts.get(3)?;
        let burn_authority = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let master_edition = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let rent = accounts.get(8)?;

        Some(DeprecatedMintPrintingTokensViaTokenInstructionAccounts {
            destination: *destination,
            token: *token,
            one_time_printing_authorization_mint: *one_time_printing_authorization_mint,
            printing_mint: *printing_mint,
            burn_authority: *burn_authority,
            metadata: *metadata,
            master_edition: *master_edition,
            token_program: *token_program,
            rent: *rent,
        })
    }
}
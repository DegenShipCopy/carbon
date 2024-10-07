
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5f81edf00831df84")]
pub struct CancelOrder{
}

pub struct CancelOrderInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub input_mint_reserve: solana_sdk::pubkey::Pubkey,
    pub maker_input_mint_account: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub input_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CancelOrder {
    type ArrangedAccounts = CancelOrderInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let signer = accounts.get(0)?;
        let maker = accounts.get(1)?;
        let order = accounts.get(2)?;
        let input_mint_reserve = accounts.get(3)?;
        let maker_input_mint_account = accounts.get(4)?;
        let input_mint = accounts.get(5)?;
        let input_token_program = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(CancelOrderInstructionAccounts {
            signer: *signer,
            maker: *maker,
            order: *order,
            input_mint_reserve: *input_mint_reserve,
            maker_input_mint_account: *maker_input_mint_account,
            input_mint: *input_mint,
            input_token_program: *input_token_program,
            event_authority: *event_authority,
            program: *program,
        })
    }
}
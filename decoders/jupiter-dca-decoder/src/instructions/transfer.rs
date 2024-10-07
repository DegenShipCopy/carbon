
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa334c8e78c0345ba")]
pub struct Transfer{
}

pub struct TransferInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub dca: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub dca_out_ata: solana_sdk::pubkey::Pubkey,
    pub user_out_ata: solana_sdk::pubkey::Pubkey,
    pub intermediate_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let dca = accounts.get(1)?;
        let user = accounts.get(2)?;
        let output_mint = accounts.get(3)?;
        let dca_out_ata = accounts.get(4)?;
        let user_out_ata = accounts.get(5)?;
        let intermediate_account = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;

        Some(TransferInstructionAccounts {
            keeper: *keeper,
            dca: *dca,
            user: *user,
            output_mint: *output_mint,
            dca_out_ata: *dca_out_ata,
            user_out_ata: *user_out_ata,
            intermediate_account: *intermediate_account,
            system_program: *system_program,
            token_program: *token_program,
            associated_token_program: *associated_token_program,
            event_authority: *event_authority,
            program: *program,
        })
    }
}
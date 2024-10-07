
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd12ad0b38c4e122b")]
pub struct CloseEscrowAccount{
}

pub struct CloseEscrowAccountInstructionAccounts {
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CloseEscrowAccount {
    type ArrangedAccounts = CloseEscrowAccountInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let escrow = accounts.get(0)?;
        let metadata = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let token_account = accounts.get(3)?;
        let edition = accounts.get(4)?;
        let payer = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let sysvar_instructions = accounts.get(7)?;

        Some(CloseEscrowAccountInstructionAccounts {
            escrow: *escrow,
            metadata: *metadata,
            mint: *mint,
            token_account: *token_account,
            edition: *edition,
            payer: *payer,
            system_program: *system_program,
            sysvar_instructions: *sysvar_instructions,
        })
    }
}
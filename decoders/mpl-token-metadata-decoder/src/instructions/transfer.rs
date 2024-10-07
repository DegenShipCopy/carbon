
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa334c8e78c0345ba")]
pub struct Transfer{
    pub transfer_args: TransferArgs,
}

pub struct TransferInstructionAccounts {
    pub token: solana_sdk::pubkey::Pubkey,
    pub token_owner: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub destination_owner: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub destination_token_record: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
    pub spl_ata_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token = accounts.get(0)?;
        let token_owner = accounts.get(1)?;
        let destination = accounts.get(2)?;
        let destination_owner = accounts.get(3)?;
        let mint = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let edition = accounts.get(6)?;
        let owner_token_record = accounts.get(7)?;
        let destination_token_record = accounts.get(8)?;
        let authority = accounts.get(9)?;
        let payer = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let sysvar_instructions = accounts.get(12)?;
        let spl_token_program = accounts.get(13)?;
        let spl_ata_program = accounts.get(14)?;
        let authorization_rules_program = accounts.get(15)?;
        let authorization_rules = accounts.get(16)?;

        Some(TransferInstructionAccounts {
            token: *token,
            token_owner: *token_owner,
            destination: *destination,
            destination_owner: *destination_owner,
            mint: *mint,
            metadata: *metadata,
            edition: *edition,
            owner_token_record: *owner_token_record,
            destination_token_record: *destination_token_record,
            authority: *authority,
            payer: *payer,
            system_program: *system_program,
            sysvar_instructions: *sysvar_instructions,
            spl_token_program: *spl_token_program,
            spl_ata_program: *spl_ata_program,
            authorization_rules_program: *authorization_rules_program,
            authorization_rules: *authorization_rules,
        })
    }
}
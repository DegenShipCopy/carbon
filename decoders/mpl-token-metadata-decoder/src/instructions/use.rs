
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x56cd74a60cb1fc53")]
pub struct Use{
    pub use_args: UseArgs,
}

pub struct UseInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub delegate_record: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Use {
    type ArrangedAccounts = UseInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let delegate_record = accounts.get(1)?;
        let token = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let metadata = accounts.get(4)?;
        let edition = accounts.get(5)?;
        let payer = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let sysvar_instructions = accounts.get(8)?;
        let spl_token_program = accounts.get(9)?;
        let authorization_rules_program = accounts.get(10)?;
        let authorization_rules = accounts.get(11)?;

        Some(UseInstructionAccounts {
            authority: *authority,
            delegate_record: *delegate_record,
            token: *token,
            mint: *mint,
            metadata: *metadata,
            edition: *edition,
            payer: *payer,
            system_program: *system_program,
            sysvar_instructions: *sysvar_instructions,
            spl_token_program: *spl_token_program,
            authorization_rules_program: *authorization_rules_program,
            authorization_rules: *authorization_rules,
        })
    }
}
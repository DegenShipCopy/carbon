
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2e527d92558de499")]
pub struct InitializePositionPda{
    pub lower_bin_id: i32,
    pub width: i32,
}

pub struct InitializePositionPdaInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub base: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for InitializePositionPda {
    type ArrangedAccounts = InitializePositionPdaInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let base = accounts.get(1)?;
        let position = accounts.get(2)?;
        let lb_pair = accounts.get(3)?;
        let owner = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let rent = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(InitializePositionPdaInstructionAccounts {
            payer: *payer,
            base: *base,
            position: *position,
            lb_pair: *lb_pair,
            owner: *owner,
            system_program: *system_program,
            rent: *rent,
            event_authority: *event_authority,
            program: *program,
        })
    }
}
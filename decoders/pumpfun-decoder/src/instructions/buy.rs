
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x66063d1201daebea")]
pub struct Buy{
    pub amount: u64,
    pub max_sol_cost: u64,
}

pub struct BuyInstructionAccounts {
    pub global: solana_sdk::pubkey::Pubkey,
    pub fee_recipient: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_bonding_curve: solana_sdk::pubkey::Pubkey,
    pub associated_user: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for Buy {
    type ArrangedAccounts = BuyInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let global = accounts.get(0)?;
        let fee_recipient = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let bonding_curve = accounts.get(3)?;
        let associated_bonding_curve = accounts.get(4)?;
        let associated_user = accounts.get(5)?;
        let user = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let rent = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;

        Some(BuyInstructionAccounts {
            global: *global,
            fee_recipient: *fee_recipient,
            mint: *mint,
            bonding_curve: *bonding_curve,
            associated_bonding_curve: *associated_bonding_curve,
            associated_user: *associated_user,
            user: *user,
            system_program: *system_program,
            token_program: *token_program,
            rent: *rent,
            event_authority: *event_authority,
            program: *program,
        })
    }
}

use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5ed6e86f8e3d7b1d")]
pub struct StabbleWeightedSwap{
}

pub struct StabbleWeightedSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_token_in: solana_sdk::pubkey::Pubkey,
    pub user_token_out: solana_sdk::pubkey::Pubkey,
    pub vault_token_in: solana_sdk::pubkey::Pubkey,
    pub vault_token_out: solana_sdk::pubkey::Pubkey,
    pub beneficiary_token_out: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub withdraw_authority: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub vault_authority: solana_sdk::pubkey::Pubkey,
    pub vault_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for StabbleWeightedSwap {
    type ArrangedAccounts = StabbleWeightedSwapInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let user = accounts.get(1)?;
        let user_token_in = accounts.get(2)?;
        let user_token_out = accounts.get(3)?;
        let vault_token_in = accounts.get(4)?;
        let vault_token_out = accounts.get(5)?;
        let beneficiary_token_out = accounts.get(6)?;
        let pool = accounts.get(7)?;
        let withdraw_authority = accounts.get(8)?;
        let vault = accounts.get(9)?;
        let vault_authority = accounts.get(10)?;
        let vault_program = accounts.get(11)?;
        let token_program = accounts.get(12)?;

        Some(StabbleWeightedSwapInstructionAccounts {
            swap_program: *swap_program,
            user: *user,
            user_token_in: *user_token_in,
            user_token_out: *user_token_out,
            vault_token_in: *vault_token_in,
            vault_token_out: *vault_token_out,
            beneficiary_token_out: *beneficiary_token_out,
            pool: *pool,
            withdraw_authority: *withdraw_authority,
            vault: *vault,
            vault_authority: *vault_authority,
            vault_program: *vault_program,
            token_program: *token_program,
        })
    }
}

use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb16b25b4a01331d1")]
pub struct CollectRewardV2{
    pub reward_index: u8,
    pub remaining_accounts_info: Option<RemainingAccountsInfo>,
}

pub struct CollectRewardV2InstructionAccounts {
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub reward_owner_account: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub reward_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CollectRewardV2 {
    type ArrangedAccounts = CollectRewardV2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let whirlpool = accounts.get(0)?;
        let position_authority = accounts.get(1)?;
        let position = accounts.get(2)?;
        let position_token_account = accounts.get(3)?;
        let reward_owner_account = accounts.get(4)?;
        let reward_mint = accounts.get(5)?;
        let reward_vault = accounts.get(6)?;
        let reward_token_program = accounts.get(7)?;
        let memo_program = accounts.get(8)?;

        Some(CollectRewardV2InstructionAccounts {
            whirlpool: *whirlpool,
            position_authority: *position_authority,
            position: *position,
            position_token_account: *position_token_account,
            reward_owner_account: *reward_owner_account,
            reward_mint: *reward_mint,
            reward_vault: *reward_vault,
            reward_token_program: *reward_token_program,
            memo_program: *memo_program,
        })
    }
}
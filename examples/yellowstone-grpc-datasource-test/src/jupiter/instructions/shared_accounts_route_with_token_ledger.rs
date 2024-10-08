
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use crate::jupiter::types::RoutePlanStep;

use super::super::types;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe6798f50779f6aaa")]
pub struct SharedAccountsRouteWithTokenLedger{
    pub id: u8,
    pub route_plan: Vec<RoutePlanStep>,
    pub quoted_out_amount: u64,
    pub slippage_bps: u16,
    pub platform_fee_bps: u8,
}

pub struct SharedAccountsRouteWithTokenLedgerInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub program_source_token_account: solana_sdk::pubkey::Pubkey,
    pub program_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub platform_fee_account: solana_sdk::pubkey::Pubkey,
    pub token2022_program: solana_sdk::pubkey::Pubkey,
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SharedAccountsRouteWithTokenLedger {
    type ArrangedAccounts = SharedAccountsRouteWithTokenLedgerInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let program_authority = accounts.get(1)?;
        let user_transfer_authority = accounts.get(2)?;
        let source_token_account = accounts.get(3)?;
        let program_source_token_account = accounts.get(4)?;
        let program_destination_token_account = accounts.get(5)?;
        let destination_token_account = accounts.get(6)?;
        let source_mint = accounts.get(7)?;
        let destination_mint = accounts.get(8)?;
        let platform_fee_account = accounts.get(9)?;
        let token2022_program = accounts.get(10)?;
        let token_ledger = accounts.get(11)?;
        let event_authority = accounts.get(12)?;
        let program = accounts.get(13)?;

        Some(SharedAccountsRouteWithTokenLedgerInstructionAccounts {
            token_program: *token_program,
            program_authority: *program_authority,
            user_transfer_authority: *user_transfer_authority,
            source_token_account: *source_token_account,
            program_source_token_account: *program_source_token_account,
            program_destination_token_account: *program_destination_token_account,
            destination_token_account: *destination_token_account,
            source_mint: *source_mint,
            destination_mint: *destination_mint,
            platform_fee_account: *platform_fee_account,
            token2022_program: *token2022_program,
            token_ledger: *token_ledger,
            event_authority: *event_authority,
            program: *program,
        })
    }
}
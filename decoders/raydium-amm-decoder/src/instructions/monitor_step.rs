
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfcdb123057b71a9a")]
pub struct MonitorStep{
    pub plan_order_limit: u16,
    pub place_order_limit: u16,
    pub cancel_order_limit: u16,
}

pub struct MonitorStepInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub amm: solana_sdk::pubkey::Pubkey,
    pub amm_authority: solana_sdk::pubkey::Pubkey,
    pub amm_open_orders: solana_sdk::pubkey::Pubkey,
    pub amm_target_orders: solana_sdk::pubkey::Pubkey,
    pub pool_coin_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_pc_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_withdraw_queue: solana_sdk::pubkey::Pubkey,
    pub serum_program: solana_sdk::pubkey::Pubkey,
    pub serum_market: solana_sdk::pubkey::Pubkey,
    pub serum_coin_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_pc_vault_account: solana_sdk::pubkey::Pubkey,
    pub serum_vault_signer: solana_sdk::pubkey::Pubkey,
    pub serum_req_q: solana_sdk::pubkey::Pubkey,
    pub serum_event_q: solana_sdk::pubkey::Pubkey,
    pub serum_bids: solana_sdk::pubkey::Pubkey,
    pub serum_asks: solana_sdk::pubkey::Pubkey,
    pub serum_fee_discount: solana_sdk::pubkey::Pubkey,
    pub referrer_pc_account: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for MonitorStep {
    type ArrangedAccounts = MonitorStepInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let rent = accounts.get(1)?;
        let clock = accounts.get(2)?;
        let amm = accounts.get(3)?;
        let amm_authority = accounts.get(4)?;
        let amm_open_orders = accounts.get(5)?;
        let amm_target_orders = accounts.get(6)?;
        let pool_coin_token_account = accounts.get(7)?;
        let pool_pc_token_account = accounts.get(8)?;
        let pool_withdraw_queue = accounts.get(9)?;
        let serum_program = accounts.get(10)?;
        let serum_market = accounts.get(11)?;
        let serum_coin_vault_account = accounts.get(12)?;
        let serum_pc_vault_account = accounts.get(13)?;
        let serum_vault_signer = accounts.get(14)?;
        let serum_req_q = accounts.get(15)?;
        let serum_event_q = accounts.get(16)?;
        let serum_bids = accounts.get(17)?;
        let serum_asks = accounts.get(18)?;
        let serum_fee_discount = accounts.get(19)?;
        let referrer_pc_account = accounts.get(20)?;

        Some(MonitorStepInstructionAccounts {
            token_program: *token_program,
            rent: *rent,
            clock: *clock,
            amm: *amm,
            amm_authority: *amm_authority,
            amm_open_orders: *amm_open_orders,
            amm_target_orders: *amm_target_orders,
            pool_coin_token_account: *pool_coin_token_account,
            pool_pc_token_account: *pool_pc_token_account,
            pool_withdraw_queue: *pool_withdraw_queue,
            serum_program: *serum_program,
            serum_market: *serum_market,
            serum_coin_vault_account: *serum_coin_vault_account,
            serum_pc_vault_account: *serum_pc_vault_account,
            serum_vault_signer: *serum_vault_signer,
            serum_req_q: *serum_req_q,
            serum_event_q: *serum_event_q,
            serum_bids: *serum_bids,
            serum_asks: *serum_asks,
            serum_fee_discount: *serum_fee_discount,
            referrer_pc_account: *referrer_pc_account,
        })
    }
}
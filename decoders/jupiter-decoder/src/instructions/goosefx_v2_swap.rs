use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb26cd0899ac2a8d5")]
pub struct GoosefxV2Swap {}

pub struct GoosefxV2SwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub pair: solana_sdk::pubkey::Pubkey,
    pub pool_registry: solana_sdk::pubkey::Pubkey,
    pub user_wallet: solana_sdk::pubkey::Pubkey,
    pub ssl_pool_in_signer: solana_sdk::pubkey::Pubkey,
    pub ssl_pool_out_signer: solana_sdk::pubkey::Pubkey,
    pub user_ata_in: solana_sdk::pubkey::Pubkey,
    pub user_ata_out: solana_sdk::pubkey::Pubkey,
    pub ssl_out_main_vault: solana_sdk::pubkey::Pubkey,
    pub ssl_out_secondary_vault: solana_sdk::pubkey::Pubkey,
    pub ssl_in_main_vault: solana_sdk::pubkey::Pubkey,
    pub ssl_in_secondary_vault: solana_sdk::pubkey::Pubkey,
    pub ssl_out_fee_vault: solana_sdk::pubkey::Pubkey,
    pub fee_destination: solana_sdk::pubkey::Pubkey,
    pub output_token_price_history: solana_sdk::pubkey::Pubkey,
    pub output_token_oracle: solana_sdk::pubkey::Pubkey,
    pub input_token_price_history: solana_sdk::pubkey::Pubkey,
    pub input_token_oracle: solana_sdk::pubkey::Pubkey,
    pub event_emitter: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for GoosefxV2Swap {
    type ArrangedAccounts = GoosefxV2SwapInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let pair = accounts.get(1)?;
        let pool_registry = accounts.get(2)?;
        let user_wallet = accounts.get(3)?;
        let ssl_pool_in_signer = accounts.get(4)?;
        let ssl_pool_out_signer = accounts.get(5)?;
        let user_ata_in = accounts.get(6)?;
        let user_ata_out = accounts.get(7)?;
        let ssl_out_main_vault = accounts.get(8)?;
        let ssl_out_secondary_vault = accounts.get(9)?;
        let ssl_in_main_vault = accounts.get(10)?;
        let ssl_in_secondary_vault = accounts.get(11)?;
        let ssl_out_fee_vault = accounts.get(12)?;
        let fee_destination = accounts.get(13)?;
        let output_token_price_history = accounts.get(14)?;
        let output_token_oracle = accounts.get(15)?;
        let input_token_price_history = accounts.get(16)?;
        let input_token_oracle = accounts.get(17)?;
        let event_emitter = accounts.get(18)?;
        let token_program = accounts.get(19)?;

        Some(GoosefxV2SwapInstructionAccounts {
            swap_program: *swap_program,
            pair: *pair,
            pool_registry: *pool_registry,
            user_wallet: *user_wallet,
            ssl_pool_in_signer: *ssl_pool_in_signer,
            ssl_pool_out_signer: *ssl_pool_out_signer,
            user_ata_in: *user_ata_in,
            user_ata_out: *user_ata_out,
            ssl_out_main_vault: *ssl_out_main_vault,
            ssl_out_secondary_vault: *ssl_out_secondary_vault,
            ssl_in_main_vault: *ssl_in_main_vault,
            ssl_in_secondary_vault: *ssl_in_secondary_vault,
            ssl_out_fee_vault: *ssl_out_fee_vault,
            fee_destination: *fee_destination,
            output_token_price_history: *output_token_price_history,
            output_token_oracle: *output_token_oracle,
            input_token_price_history: *input_token_price_history,
            input_token_oracle: *input_token_oracle,
            event_emitter: *event_emitter,
            token_program: *token_program,
        })
    }
}

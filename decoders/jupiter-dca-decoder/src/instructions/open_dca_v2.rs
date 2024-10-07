
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8e772b6da2340bb1")]
pub struct OpenDcaV2{
    pub application_idx: u64,
    pub in_amount: u64,
    pub in_amount_per_cycle: u64,
    pub cycle_frequency: i64,
    pub min_out_amount: Option<u64>,
    pub max_out_amount: Option<u64>,
    pub start_at: Option<i64>,
}

pub struct OpenDcaV2InstructionAccounts {
    pub dca: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub user_ata: solana_sdk::pubkey::Pubkey,
    pub in_ata: solana_sdk::pubkey::Pubkey,
    pub out_ata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for OpenDcaV2 {
    type ArrangedAccounts = OpenDcaV2InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let dca = accounts.get(0)?;
        let user = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let input_mint = accounts.get(3)?;
        let output_mint = accounts.get(4)?;
        let user_ata = accounts.get(5)?;
        let in_ata = accounts.get(6)?;
        let out_ata = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let event_authority = accounts.get(11)?;
        let program = accounts.get(12)?;

        Some(OpenDcaV2InstructionAccounts {
            dca: *dca,
            user: *user,
            payer: *payer,
            input_mint: *input_mint,
            output_mint: *output_mint,
            user_ata: *user_ata,
            in_ata: *in_ata,
            out_ata: *out_ata,
            system_program: *system_program,
            token_program: *token_program,
            associated_token_program: *associated_token_program,
            event_authority: *event_authority,
            program: *program,
        })
    }
}
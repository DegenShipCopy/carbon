
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa39fa319f3a16c4a")]
pub struct HeliumTreasuryManagementRedeemV0{
}

pub struct HeliumTreasuryManagementRedeemV0InstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub treasury_management: solana_sdk::pubkey::Pubkey,
    pub treasury_mint: solana_sdk::pubkey::Pubkey,
    pub supply_mint: solana_sdk::pubkey::Pubkey,
    pub treasury: solana_sdk::pubkey::Pubkey,
    pub circuit_breaker: solana_sdk::pubkey::Pubkey,
    pub from: solana_sdk::pubkey::Pubkey,
    pub to: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub circuit_breaker_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for HeliumTreasuryManagementRedeemV0 {
    type ArrangedAccounts = HeliumTreasuryManagementRedeemV0InstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let treasury_management = accounts.get(1)?;
        let treasury_mint = accounts.get(2)?;
        let supply_mint = accounts.get(3)?;
        let treasury = accounts.get(4)?;
        let circuit_breaker = accounts.get(5)?;
        let from = accounts.get(6)?;
        let to = accounts.get(7)?;
        let owner = accounts.get(8)?;
        let circuit_breaker_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(HeliumTreasuryManagementRedeemV0InstructionAccounts {
            swap_program: *swap_program,
            treasury_management: *treasury_management,
            treasury_mint: *treasury_mint,
            supply_mint: *supply_mint,
            treasury: *treasury,
            circuit_breaker: *circuit_breaker,
            from: *from,
            to: *to,
            owner: *owner,
            circuit_breaker_program: *circuit_breaker_program,
            token_program: *token_program,
        })
    }
}

use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use super::super::types::*;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x7b86510031446262")]
pub struct ClosePosition{
}

pub struct ClosePositionInstructionAccounts {
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_lower: solana_sdk::pubkey::Pubkey,
    pub bin_array_upper: solana_sdk::pubkey::Pubkey,
    pub sender: solana_sdk::pubkey::Pubkey,
    pub rent_receiver: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for ClosePosition {
    type ArrangedAccounts = ClosePositionInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::pubkey::Pubkey>) -> Option<Self::ArrangedAccounts> {
        let position = accounts.get(0)?;
        let lb_pair = accounts.get(1)?;
        let bin_array_lower = accounts.get(2)?;
        let bin_array_upper = accounts.get(3)?;
        let sender = accounts.get(4)?;
        let rent_receiver = accounts.get(5)?;
        let event_authority = accounts.get(6)?;
        let program = accounts.get(7)?;

        Some(ClosePositionInstructionAccounts {
            position: *position,
            lb_pair: *lb_pair,
            bin_array_lower: *bin_array_lower,
            bin_array_upper: *bin_array_upper,
            sender: *sender,
            rent_receiver: *rent_receiver,
            event_authority: *event_authority,
            program: *program,
        })
    }
}
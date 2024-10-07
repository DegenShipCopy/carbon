
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d80977b6a1166718e")]
pub struct CompositionFee{
    pub from: solana_sdk::pubkey::Pubkey,
    pub bin_id: i16,
    pub token_x_fee_amount: u64,
    pub token_y_fee_amount: u64,
    pub protocol_token_x_fee_amount: u64,
    pub protocol_token_y_fee_amount: u64,
}

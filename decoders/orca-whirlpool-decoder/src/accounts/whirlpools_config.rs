 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x9d1431e0d957c1fe")] 
pub struct WhirlpoolsConfig { 
        pub fee_authority: solana_sdk::pubkey::Pubkey, 
        pub collect_protocol_fees_authority: solana_sdk::pubkey::Pubkey, 
        pub reward_emissions_super_authority: solana_sdk::pubkey::Pubkey, 
        pub default_protocol_fee_rate: u16, 
}
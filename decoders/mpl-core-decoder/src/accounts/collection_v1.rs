 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xf1e9caaec3d4e560")] 
pub struct CollectionV1 { 
        pub key: Key, 
        pub update_authority: solana_sdk::pubkey::Pubkey, 
        pub name: String, 
        pub uri: String, 
        pub num_minted: u32, 
        pub current_size: u32, 
}
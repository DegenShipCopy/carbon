 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x480b791a6fb5555d")] 
pub struct Metadata { 
        pub key: Key, 
        pub update_authority: solana_sdk::pubkey::Pubkey, 
        pub mint: solana_sdk::pubkey::Pubkey, 
        pub data: Data, 
        pub primary_sale_happened: bool, 
        pub is_mutable: bool, 
        pub edition_nonce: Option<u8>, 
        pub token_standard: Option<TokenStandard>, 
        pub collection: Option<Collection>, 
        pub uses: Option<Uses>, 
        pub collection_details: Option<CollectionDetails>, 
        pub programmable_config: Option<ProgrammableConfig>, 
}
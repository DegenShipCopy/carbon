
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;
use super::super::types::*;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d85d642e0400c07bf")]
pub struct UpdatePositionLockReleasePoint{
    pub position: solana_sdk::pubkey::Pubkey,
    pub current_point: u64,
    pub new_lock_release_point: u64,
    pub old_lock_release_point: u64,
    pub sender: solana_sdk::pubkey::Pubkey,
}

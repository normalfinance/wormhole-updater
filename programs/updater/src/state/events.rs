use anchor_lang::prelude::*;

#[event]
pub struct RatedUpdatedRecord {
    /// unix_timestamp of action
    pub ts: i64,
    pub ethereum_block_number: u128,
    pub total_active_stake: u32,
}

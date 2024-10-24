use anchor_lang::prelude::*;

#[event]
pub struct RatedUpdatedRecord {
    /// unix_timestamp of action
    pub ts: i64,
    pub ethereumBlockNumber: u128,
    pub totalActiveStake: u32,
}

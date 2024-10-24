use anchor_lang::prelude::*;
use crate::state::traits::Size;

use borsh::{ BorshSerialize, BorshDeserialize };

#[account]
#[derive(Default)]
#[repr(C)]
pub struct State {
    pub admin: Pubkey,
    pub last_update_ethereum_block_number: u128,
    pub stake_pool_account: EthereumAddress,
    pub total_active_stake: u32,
    pub allowed_update_staleness: i64,
    pub allowed_rate_staleness: i64,
    pub padding: [u8; 10],
}

impl State {}

impl Size for State {
    const SIZE: usize = 992;
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq)]
pub struct EthereumAddress(pub [u8; 32]);

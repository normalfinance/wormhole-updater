use anchor_lang::prelude::*;

use instructions::*;

use crate::state::state::*;

declare_id!("HkDXBFRS9Tv9295d9wEVRL61c1pUXj3WZHiaTNZ9Q7TQ");

pub mod error;
pub mod instructions;
pub mod macros;
pub mod math;
pub mod state;

#[program]
pub mod updater {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        stake_pool_account: EthereumAddress,
        allowed_update_staleness: u32,
        allowed_rate_staleness: u32
    ) -> Result<()> {
        instructions::initialize(
            ctx,
            stake_pool_account,
            allowed_update_staleness,
            allowed_rate_staleness
        )
    }

    pub fn close_signatures(ctx: Context<CloseSignatures>) -> Result<()> {
        instructions::close_signatures(ctx)
    }

    pub fn post_signatures(
        ctx: Context<PostSignatures>,
        guardian_signatures: Vec<[u8; 66]>,
        total_signatures: u8
    ) -> Result<()> {
        instructions::post_signatures(ctx, guardian_signatures, total_signatures)
    }

    // Serves as an example function by simply printing our the request and response data
    pub fn verify_query(
        ctx: Context<VerifyQuery>,
        bytes: Vec<u8>,
        guardian_set_index: u32
    ) -> Result<()> {
        instructions::verify_query(ctx, bytes, guardian_set_index)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

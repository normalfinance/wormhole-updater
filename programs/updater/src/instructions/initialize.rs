use anchor_lang::prelude::*;

use crate::state::state::{ State, EthereumAddress };

pub fn initialize(
    ctx: Context<Initialize>,
    stake_pool_account: EthereumAddress,
    allowed_update_staleness: u32,
    allowed_rate_staleness: u32
) -> Result<()> {
    **ctx.accounts.state = State {
        admin: *ctx.accounts.admin.key,
        last_update_ethereum_block_number: 0,
        stake_pool_account: stake_pool_account,
        allowed_update_staleness: allowed_update_staleness,
        allowed_rate_staleness: allowed_rate_staleness,
        total_active_stake: 0,
        padding: [0; 10],
    };

    // Done.
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, seeds = [b"updater_state".as_ref()], space = State::SIZE, bump, payer = admin)]
    pub state: Box<Account<'info, State>>,
    /// CHECK: checked in `initialize`
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

use anchor_lang::prelude::*;

use crate::State;

pub fn get_rate(ctx: Context<GetRate>) -> Result<u32> {
    let state = &mut ctx.accounts.state;

    Ok(state.total_active_stake)
}

#[derive(Accounts)]
pub struct GetRate<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        mut,
        has_one = admin
    )]
    pub state: Box<Account<'info, State>>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

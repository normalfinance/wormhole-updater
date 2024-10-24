use crate::{
    error::ErrorCode,
    math::ethereum::eth_address_to_bytes,
    state::{ GuardianSignatures, RatedUpdatedRecord, WormholeGuardianSet },
    State,
};
use anchor_lang::prelude::*;
use wormhole_query_sdk::structs::{ ChainSpecificResponse, QueryResponse };
use wormhole_solana_consts::CORE_BRIDGE_PROGRAM_ID;

use crate::instructions::constraints::is_valid_signature;

use crate::math::constants::{ ETHEREUM_CHAIN_ID, ETHEREUM_STAKE_POOL_PROGRAM };

use crate::validate;

#[access_control(
    is_valid_signature(&ctx.accounts.guardian_set, &ctx.accounts.guardian_signatures, &bytes)
)]
pub fn update_pool(
    ctx: Context<UpdatePool>,
    bytes: Vec<u8>,
    _guardian_set_index: u32
) -> Result<()> {
    let response = QueryResponse::deserialize(&bytes).map_err(
        |_| ErrorCode::FailedToParseResponse
    )?;

    // Validations

    validate!(
        response.responses.len() != 1,
        ErrorCode::UnexpectedResultLength,
        "unexpected result length"
    )?;

    validate!(
        response.request_chain_id != (ETHEREUM_CHAIN_ID as u16),
        ErrorCode::InvalidForeignChainID,
        "invalid foreign chain ID"
    )?;

    let per_chain_query_response = &response.responses[0];

    match &per_chain_query_response.response {
        ChainSpecificResponse::EthCallQueryResponse(eth_response) => {
            msg!(
                "EthCallQueryResponse: {}, {}, {:?}. {}, {}",
                per_chain_query_response.chain_id,
                eth_response.block_number,
                eth_response.block_hash,
                eth_response.block_time,
                eth_response.results.len()
            );

            validate!(
                eth_response.results.len() != 2,
                ErrorCode::UnexpectedResultLength,
                "unexpected result length"
            )?;

            for result_idx in 0..eth_response.results.len() {
                let result = &eth_response.results[result_idx];
                msg!("result: {:?}", result);
            }

            // TODO: unsure how to properly access response data as results is a Vec<Vec<u8>>
            // let state = &mut ctx.accounts.state;

            // validate!(
            //     eth_response.results[0].account != state.stake_pool_account,
            //     ErrorCode::InvalidAccount,
            //     "invalid account"
            // )?;

            // validate!(
            //     eth_response.results[0].owner != eth_address_to_bytes(ETHEREUM_STAKE_POOL_PROGRAM)?,
            //     ErrorCode::InvalidAccountOwner,
            //     "invalid account owner"
            // )?;

            // let new_total_active_stake = &eth_response.results[0].data;
            // state.total_active_stake = new_total_active_stake;

            // let now_ts = Clock::get()?.unix_timestamp;

            // emit!(RatedUpdatedRecord {
            //     ts: now_ts,
            //     ethereum_block_number: 0,
            //     total_active_stake: new_total_active_stake,
            // });
        }
        ChainSpecificResponse::EthCallByTimestampQueryResponse(_) => {
            msg!("EthCallByTimestampQueryResponse");
        }
        ChainSpecificResponse::EthCallWithFinalityQueryResponse(_) => {
            msg!("EthCallWithFinalityQueryResponse");
        }
        ChainSpecificResponse::SolanaAccountQueryResponse(_) => {
            msg!("SolanaAccountQueryResponse");
        }
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(_bytes: Vec<u8>, guardian_set_index: u32)]
pub struct UpdatePool<'info> {
    /// Guardian set used for signature verification.
    #[account(
        seeds = [WormholeGuardianSet::SEED_PREFIX, guardian_set_index.to_be_bytes().as_ref()],
        bump,
        seeds::program = CORE_BRIDGE_PROGRAM_ID
    )]
    guardian_set: Account<'info, WormholeGuardianSet>,

    /// Stores unverified guardian signatures as they are too large to fit in the instruction data.
    #[account(mut, has_one = refund_recipient, close = refund_recipient)]
    guardian_signatures: Account<'info, GuardianSignatures>,

    /// CHECK: This account is the refund recipient for the above signature_set
    #[account(address = guardian_signatures.refund_recipient)]
    refund_recipient: AccountInfo<'info>,

    #[account(mut)]
    pub state: Box<Account<'info, State>>,
}

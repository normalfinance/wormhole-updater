use crate::{ error::ErrorCode, state::{ GuardianSignatures, WormholeGuardianSet } };
use anchor_lang::prelude::*;
use wormhole_query_sdk::structs::{ ChainSpecificQuery, ChainSpecificResponse, QueryResponse };
use wormhole_solana_consts::CORE_BRIDGE_PROGRAM_ID;

use crate::instructions::constraints::is_valid_signature;

#[access_control(
    is_valid_signature(&ctx.accounts.guardian_set, &ctx.accounts.guardian_signatures, &bytes)
)]
pub fn verify_query(
    ctx: Context<VerifyQuery>,
    bytes: Vec<u8>,
    _guardian_set_index: u32
) -> Result<()> {
    let response = QueryResponse::deserialize(&bytes).map_err(
        |_| ErrorCode::FailedToParseResponse
    )?;
    msg!(
        "response: version: {}, req_chain: {}, req_id: {:?}, req_version: {}, req_nonce: {}, reqs_len: {}, resp_len: {}",
        response.version,
        response.request_chain_id,
        response.request_id,
        response.request.version,
        response.request.nonce,
        response.request.requests.len(),
        response.responses.len()
    );
    for idx in 0..response.request.requests.len() {
        let request = &response.request.requests[idx];
        match &request.query {
            ChainSpecificQuery::EthCallQueryRequest(q) => {
                msg!(
                    "EthCallQueryRequest: {}, {}, {}",
                    request.chain_id,
                    q.block_tag,
                    q.call_data.len()
                );
                for call_idx in 0..q.call_data.len() {
                    let call = &q.call_data[call_idx];
                    msg!("call: {:?}, {:?}", call.to, call.data);
                }
            }
            ChainSpecificQuery::EthCallByTimestampQueryRequest(_) => {
                msg!("EthCallByTimestampQueryRequest");
            }
            ChainSpecificQuery::EthCallWithFinalityQueryRequest(_) => {
                msg!("EthCallWithFinalityQueryRequest");
            }
            ChainSpecificQuery::SolanaAccountQueryRequest(_) => {
                msg!("SolanaAccountQueryRequest");
            }
        }
    }
    for idx in 0..response.responses.len() {
        let response = &response.responses[idx];
        match &response.response {
            ChainSpecificResponse::EthCallQueryResponse(eth_response) => {
                msg!(
                    "EthCallQueryResponse: {}, {}, {:?}. {}, {}",
                    response.chain_id,
                    eth_response.block_number,
                    eth_response.block_hash,
                    eth_response.block_time,
                    eth_response.results.len()
                );
                for result_idx in 0..eth_response.results.len() {
                    let result = &eth_response.results[result_idx];
                    msg!("result: {:?}", result);
                }
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
    }

    // Done.
    Ok(())
}

#[derive(Accounts)]
#[instruction(_bytes: Vec<u8>, guardian_set_index: u32)]
pub struct VerifyQuery<'info> {
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
}

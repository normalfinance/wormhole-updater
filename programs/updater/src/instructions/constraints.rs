use anchor_lang::accounts::account::Account;

use crate::{ error::ErrorCode, state::{ GuardianSignatures, WormholeGuardianSet } };

use anchor_lang::{ prelude::*, solana_program::{ self, keccak } };
use wormhole_query_sdk::{ MESSAGE_PREFIX, QUERY_MESSAGE_LEN };
use wormhole_raw_vaas::{ utils::quorum, GuardianSetSig };

use crate::math::guardian::verify_guardian_signature;

pub fn is_valid_signature(
    guardian_set: &Account<WormholeGuardianSet>,
    guardian_signatures: &Account<GuardianSignatures>,
    bytes: &Vec<u8>
) -> anchor_lang::Result<bool> {
    // Check that the guardian set is still active.
    let timestamp = Clock::get()?.unix_timestamp.try_into().expect("timestamp overflow");
    require!(guardian_set.is_active(&timestamp), ErrorCode::GuardianSetExpired);

    // Compute the message hash.
    let message_hash = [
        MESSAGE_PREFIX,
        &solana_program::keccak::hashv(&[&bytes]).to_bytes(),
    ].concat();

    // SECURITY: defense-in-depth, check again that these are the expected length
    require_eq!(message_hash.len(), QUERY_MESSAGE_LEN, ErrorCode::InvalidMessageHash);

    let guardian_signatures = &guardian_signatures.guardian_signatures;

    // This section is borrowed from https://github.com/wormhole-foundation/wormhole/blob/wen/solana-rewrite/solana/programs/core-bridge/src/processor/parse_and_verify_vaa/verify_encoded_vaa_v1.rs#L72-L103
    // Also similarly used here https://github.com/pyth-network/pyth-crosschain/blob/6771c2c6998f53effee9247347cb0ac71612b3dc/target_chains/solana/programs/pyth-solana-receiver/src/lib.rs#L121-L159
    // Do we have enough signatures for quorum?
    let guardian_keys = &guardian_set.keys;
    let quorum = quorum(guardian_keys.len());
    require!(guardian_signatures.len() >= quorum, ErrorCode::NoQuorum);

    let digest = keccak::hash(message_hash.as_slice());

    // Verify signatures
    let mut last_guardian_index = None;
    for sig_bytes in guardian_signatures {
        let sig = GuardianSetSig::try_from(sig_bytes.as_slice()).map_err(
            |_| ErrorCode::InvalidSignature
        )?;
        // We do not allow for non-increasing guardian signature indices.
        let index = usize::from(sig.guardian_index());
        if let Some(last_index) = last_guardian_index {
            require!(index > last_index, ErrorCode::InvalidGuardianIndexNonIncreasing);
        }

        // Does this guardian index exist in this guardian set?
        let guardian_pubkey = guardian_keys
            .get(index)
            .ok_or_else(|| { error!(ErrorCode::InvalidGuardianIndexOutOfRange) })?;

        // Now verify that the signature agrees with the expected Guardian's pubkey.
        verify_guardian_signature(&sig, guardian_pubkey, digest.as_ref())?;

        last_guardian_index = Some(index);
    }
    // End borrowed section

    // Done.
    Ok(true)
}

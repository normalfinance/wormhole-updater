use anchor_lang::{
    prelude::*,
    solana_program::{ keccak, program_memory::sol_memcpy, secp256k1_recover::secp256k1_recover },
};

use wormhole_raw_vaas::GuardianSetSig;

use crate::error::ErrorCode;

/**
 * Borrowed from https://github.com/wormhole-foundation/wormhole/blob/wen/solana-rewrite/solana/programs/core-bridge/src/processor/parse_and_verify_vaa/verify_encoded_vaa_v1.rs#L121
 * Also used here https://github.com/pyth-network/pyth-crosschain/blob/6771c2c6998f53effee9247347cb0ac71612b3dc/target_chains/solana/programs/pyth-solana-receiver/src/lib.rs#L432
 */
pub fn verify_guardian_signature(
    sig: &GuardianSetSig,
    guardian_pubkey: &[u8; 20],
    digest: &[u8]
) -> Result<()> {
    // Recover using `solana_program::secp256k1_recover`. Public key recovery costs 25k compute
    // units. And hashing this public key to recover the Ethereum public key costs about 13k.
    let recovered = {
        // Recover EC public key (64 bytes).
        let pubkey: anchor_lang::solana_program::secp256k1_recover::Secp256k1Pubkey = secp256k1_recover(
            digest,
            sig.recovery_id(),
            &sig.rs()
        ).map_err(|_| ErrorCode::InvalidSignature)?;

        // The Ethereum public key is the last 20 bytes of keccak hashed public key above.
        let hashed = keccak::hash(&pubkey.to_bytes());

        let mut eth_pubkey = [0; 20];
        sol_memcpy(&mut eth_pubkey, &hashed.0[12..], 20);

        eth_pubkey
    };

    // The recovered public key should agree with the Guardian's public key at this index.
    require!(recovered == *guardian_pubkey, ErrorCode::InvalidGuardianKeyRecovery);

    // Done.
    Ok(())
}

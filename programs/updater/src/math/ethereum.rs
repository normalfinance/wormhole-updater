use anchor_lang::prelude::*;
use hex::FromHex;

// Function to convert the Ethereum contract address string to a byte array
pub fn eth_address_to_bytes(address: &str) -> Result<[u8; 20], &'static str> {
    let address = address.strip_prefix("0x").unwrap_or(address);
    let bytes = <[u8; 20]>::from_hex(address).map_err(|_| "Invalid hex format")?;
    Ok(bytes)
}

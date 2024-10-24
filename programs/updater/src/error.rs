//! Errors that may arise when interacting with the Updater Program.

use anchor_lang::prelude::*;

pub type NormalResult<T = ()> = std::result::Result<T, ErrorCode>;

/// * \>= 0x100  -- Query Verification.
///
/// NOTE: All of these error codes when triggered are offset by `ERROR_CODE_OFFSET` (6000). So for
/// example, `InvalidMessageHash` will return as 6256.
#[error_code]
pub enum ErrorCode {
    #[msg("WriteAuthorityMismatch")]
    WriteAuthorityMismatch = 0x100,

    #[msg("GuardianSetExpired")]
    GuardianSetExpired = 0x101,

    #[msg("InvalidMessageHash")]
    InvalidMessageHash = 0x102,

    #[msg("NoQuorum")]
    NoQuorum = 0x103,

    #[msg("InvalidGuardianIndexNonIncreasing")]
    InvalidGuardianIndexNonIncreasing = 0x104,

    #[msg("InvalidGuardianIndexOutOfRange")]
    InvalidGuardianIndexOutOfRange = 0x105,

    #[msg("InvalidSignature")]
    InvalidSignature = 0x106,

    #[msg("InvalidGuardianKeyRecovery")]
    InvalidGuardianKeyRecovery = 0x107,

    #[msg("FailedToParseResponse")]
    FailedToParseResponse = 0x110,

    /// From (https://github.com/jito-foundation/jitosol-wormhole-updater/blob/main/src/StakePoolRate.sol#L8)

    #[msg("InvalidAccount")]
    InvalidAccount = 0x111,

    #[msg("InvalidAccountOwner")]
    InvalidAccountOwner = 0x112,

    #[msg("InvalidForeignChainID")]
    InvalidForeignChainID = 0x113,

    #[msg("UnexpectedDataLength")]
    UnexpectedDataLength = 0x114,
}

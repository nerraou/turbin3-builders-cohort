use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Minimum Bet")]
    MinimumBet,

    #[msg("Maximum Roll")]
    MaximumRoll,

    #[msg("Minimum Roll")]
    MinimumRoll,

    #[msg("Overflow")]
    Overflow,

    #[msg("Timeout not reached")]
    TimeoutNotReached,

    #[msg("Bad ED25519 signature")]
    ED25519Signature,

    #[msg("ED25519 signature must be one")]
    ED25519SignatureMustBeOne,

    #[msg("ED25519 Header")]
    ED25519Header,

    #[msg("ED25519 Data Length")]
    ED25519DataLength,
}

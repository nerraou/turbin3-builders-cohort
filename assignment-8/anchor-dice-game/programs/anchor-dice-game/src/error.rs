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
}

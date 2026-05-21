use anchor_lang::prelude::*;
use constant_product_curve::CurveError;

#[error_code]
pub enum AmmError {
    #[msg("Pool is locked: deposits, withdrawals, and swaps are disabled")]
    PoolLocked,

    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Pool is already initialized")]
    PoolAlreadyInitialized,

    #[msg("Invalid token mint provided")]
    InvalidMint,

    #[msg("Identical token mints are not allowed")]
    IdenticalMints,

    #[msg("Invalid liquidity amount")]
    InvalidLiquidityAmount,

    #[msg("Insufficient LP tokens")]
    InsufficientLpTokens,

    #[msg("Swap output is below minimum expected amount")]
    SlippageExceeded,

    #[msg("Arithmetic overflow")]
    MathOverflow,

    #[msg("Arithmetic underflow")]
    MathUnderflow,

    #[msg("Invalid fee configuration")]
    InvalidFee,

    #[msg("Pool reserves cannot be zero")]
    EmptyPool,

    #[msg("Unauthorized action")]
    Unauthorized,

    #[msg("Invalid vault account")]
    InvalidVault,

    #[msg("Invalid LP mint")]
    InvalidLpMint,

    #[msg("Curve precision is invalid")]
    InvalidPrecision,

    #[msg("Insufficient Balance")]
    InsufficientBalance,

    #[msg("No Liquidity")]
    NoLiquidity,
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::Overflow => AmmError::MathOverflow,
            CurveError::Underflow => AmmError::MathOverflow,
            CurveError::InvalidFeeAmount => AmmError::InvalidFee,
            CurveError::InsufficientBalance => AmmError::InsufficientBalance,
            CurveError::ZeroBalance => AmmError::NoLiquidity,
            CurveError::SlippageLimitExceeded => AmmError::SlippageExceeded,
        }
    }
}

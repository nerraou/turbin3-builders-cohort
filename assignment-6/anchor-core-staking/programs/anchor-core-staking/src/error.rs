use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("NFT owner does not match the signer")]
    InvalidOwner,

    #[msg("Invalid update authority for the NFT")]
    InvalidUpdateAuthority,

    #[msg("Asset is already staked")]
    AlreadyStaked,

    #[msg("Asset is not currently staked")]
    AssetNotStaked,

    #[msg("Provided timestamp is invalid")]
    InvalidTimestamp,

    #[msg("Freeze period has not elapsed yet")]
    FreezePeriodNotElapsed,

    #[msg("Rewards basis points value is invalid")]
    InvalidRewardsBPS,

    #[msg("Asset is already frozen")]
    FrozenAsset,

    #[msg("Required asset attributes plugin is missing")]
    MissingAttributes,

    #[msg("Collection staking count attribute is missing")]
    MissingStakedCount,

    #[msg("Collection staking state is invalid")]
    InvalidCollection,
}

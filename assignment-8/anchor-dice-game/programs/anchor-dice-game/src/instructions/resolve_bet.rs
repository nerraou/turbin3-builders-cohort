use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use solana_ed25519_program::{
    Ed25519SignatureOffsets, PUBKEY_SERIALIZED_SIZE, SIGNATURE_OFFSETS_SERIALIZED_SIZE,
    SIGNATURE_OFFSETS_START, SIGNATURE_SERIALIZED_SIZE,
};

use solana_instructions_sysvar::load_instruction_at_checked;

use solana_sha256_hasher::hash;

use crate::error::ErrorCode;
use crate::*;

#[derive(Accounts)]

pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    #[account(mut)]

    /// CHECK
    pub player: UncheckedAccount<'info>,

    #[account(
		mut,
		seeds = [b"vault", house.key().as_ref()],
		bump
	)]
    pub vault: SystemAccount<'info>,

    #[account(
		mut,
		has_one = player,
		close = player,
		seeds = [b"bet", vault.key().as_ref(), player.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
		bump = bet.bump
	)]
    pub bet: Account<'info, Bet>,

    /// CHECK:
    #[account(
		address = solana_sdk_ids::sysvar::instructions::ID
	)]
    pub instruction_sysvar: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

struct Ed25519InstructionData<'a> {
    public_key: &'a [u8],
    signature: &'a [u8],
    message: &'a [u8],
}

fn read_ed25519_instruction_data(data: &[u8], offset: u16, size: usize) -> Result<&[u8]> {
    let start = usize::from(offset);
    let end = start.checked_add(size).ok_or(ErrorCode::Overflow)?;

    data.get(start..end)
        .ok_or(ErrorCode::ED25519DataLength.into())
}

fn deserialize_ed25519_instruction_data(data: &[u8]) -> Result<Ed25519InstructionData<'_>> {
    require!(
        data.len() >= SIGNATURE_OFFSETS_START,
        ErrorCode::ED25519Signature
    );

    require_eq!(data[0], 1, ErrorCode::ED25519SignatureMustBeOne);

    let offsets_start = SIGNATURE_OFFSETS_START;
    let offsets_end = offsets_start
        .checked_add(SIGNATURE_OFFSETS_SERIALIZED_SIZE)
        .ok_or(ErrorCode::Overflow)?;
    let offset_data = data
        .get(offsets_start..offsets_end)
        .ok_or(ErrorCode::ED25519Header)?;

    let offsets = Ed25519SignatureOffsets {
        signature_offset: u16::from_le_bytes([offset_data[0], offset_data[1]]),
        signature_instruction_index: u16::from_le_bytes([offset_data[2], offset_data[3]]),
        public_key_offset: u16::from_le_bytes([offset_data[4], offset_data[5]]),
        public_key_instruction_index: u16::from_le_bytes([offset_data[6], offset_data[7]]),
        message_data_offset: u16::from_le_bytes([offset_data[8], offset_data[9]]),
        message_data_size: u16::from_le_bytes([offset_data[10], offset_data[11]]),
        message_instruction_index: u16::from_le_bytes([offset_data[12], offset_data[13]]),
    };

    require!(
        offsets.signature_instruction_index == u16::MAX
            && offsets.public_key_instruction_index == u16::MAX
            && offsets.message_instruction_index == u16::MAX,
        ErrorCode::ED25519Header
    );

    let public_key =
        read_ed25519_instruction_data(data, offsets.public_key_offset, PUBKEY_SERIALIZED_SIZE)?;

    let signature =
        read_ed25519_instruction_data(data, offsets.signature_offset, SIGNATURE_SERIALIZED_SIZE)?;

    let message = read_ed25519_instruction_data(
        data,
        offsets.message_data_offset,
        usize::from(offsets.message_data_size),
    )?;

    Ok(Ed25519InstructionData {
        public_key,
        signature,
        message,
    })
}

impl<'info> ResolveBet<'info> {}

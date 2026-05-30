use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use mpl_core::accounts::BaseCollectionV1;

use crate::error::ErrorCode;
use crate::state::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
		init,
		payer = admin,
		space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
		seeds = [b"config", collection.key().as_ref()],
		bump
	)]
    pub config: Account<'info, Config>,

    #[account(
		has_one = update_authority @ ErrorCode::InvalidUpdateAuthority
	)]
    pub collection: Account<'info, BaseCollectionV1>,

    /// CHECK: this account is not initiazed and is being used fro signing purposes only
    #[account(
		seeds = [b"update_authority", collection.key().as_ref()],
		bump,
	)]
    pub update_authority: UncheckedAccount<'info>,

    #[account(
		init,
		payer = admin,
		mint::decimals = 6,
        mint::authority = config,
		seeds = [b"rewards_mint", config.key().as_ref()],
		bump
	)]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        rewards_bps: u16,
        freeze_period: u16,
        rewards_bumps: u8,
        config_bumps: u8,
    ) -> Result<()> {
        self.config.set_inner(Config {
            rewards_bps,
            freeze_period,
            rewards_bumps,
            bump: config_bumps,
        });

        Ok(())
    }
}

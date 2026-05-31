use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{mint_to_checked, MintToChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    fetch_plugin,
    instructions::UpdatePluginV1CpiBuilder,
    types::{Attribute, Attributes, Plugin, PluginType, UpdateAuthority},
    ID as MPL_CORE_ID,
};

use crate::error::ErrorCode;
use crate::state::Config;
use crate::SECONDS_PER_DAY;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds=[b"config", collection.key().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        has_one = owner,
        constraint = asset.update_authority
            == UpdateAuthority::Collection(collection.key())
    )]
    pub asset: Account<'info, BaseAssetV1>,

    #[account(
        mut,
        has_one = update_authority
    )]
    pub collection: Account<'info, BaseCollectionV1>,

    /// CHECK:
    #[account(
        seeds = [b"update_authority", collection.key().as_ref()],
        bump
    )]
    pub update_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"rewards_mint", config.key().as_ref()],
        bump = config.rewards_bumps,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = rewards_mint,
        associated_token::authority = owner,
    )]
    pub user_rewards_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,

    /// CHECK:
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> ClaimRewards<'info> {
    pub fn claim_rewards(&mut self, update_authority_bump: u8) -> Result<()> {
        let attributes = fetch_plugin::<BaseAssetV1, Attributes>(
            &self.asset.to_account_info(),
            PluginType::Attributes,
        )?
        .1;

        let now = Clock::get()?.unix_timestamp;

        let mut last_claimed_at = 0_i64;

        let mut attributes_list = Vec::new();

        for attribute in &attributes.attribute_list {
            if attribute.key == "staked" {
                require!(attribute.value == "true", ErrorCode::AssetNotStaked);

                attributes_list.push(attribute.clone());
            } else if attribute.key == "last_claimed_at" {
                last_claimed_at = attribute
                    .value
                    .parse::<i64>()
                    .map_err(|_| ErrorCode::InvalidTimestamp)?;

                attributes_list.push(Attribute {
                    key: "last_claimed_at".to_string(),
                    value: now.to_string(),
                });
            } else {
                attributes_list.push(attribute.clone());
            }
        }

        let elapsed = now
            .checked_sub(last_claimed_at)
            .ok_or(ErrorCode::InvalidTimestamp)?;

        let reward_days = elapsed
            .checked_div(SECONDS_PER_DAY)
            .ok_or(ErrorCode::InvalidTimestamp)?;

        let amount = (reward_days as u64)
            .checked_mul(self.config.rewards_bps as u64)
            .ok_or(ErrorCode::InvalidRewardsBPS)?
            .checked_mul(10u64.pow(self.rewards_mint.decimals as u32))
            .ok_or(ErrorCode::InvalidRewardsBPS)?
            .checked_div(10_000)
            .ok_or(ErrorCode::InvalidRewardsBPS)?;

        let collection_key = self.collection.key();

        let signer_seeds: &[&[u8]; 3] = &[
            b"update_authority",
            collection_key.as_ref(),
            &[update_authority_bump],
        ];

        UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.owner.to_account_info())
            .authority(Some(&self.update_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::Attributes(Attributes {
                attribute_list: attributes_list,
            }))
            .invoke_signed(&[signer_seeds])?;

        let mint_signer: &[&[&[u8]]] =
            &[&[b"config", collection_key.as_ref(), &[self.config.bump]]];

        let cpi_accounts = MintToChecked {
            mint: self.rewards_mint.to_account_info(),
            authority: self.config.to_account_info(),
            to: self.user_rewards_ata.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            mint_signer,
        );

        mint_to_checked(cpi_ctx, amount, self.rewards_mint.decimals)?;

        Ok(())
    }
}

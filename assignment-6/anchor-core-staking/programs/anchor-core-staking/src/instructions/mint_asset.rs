use anchor_lang::prelude::*;
use mpl_core::{accounts::BaseCollectionV1, instructions::CreateV2CpiBuilder, ID as MPL_CORE_ID};

#[derive(Accounts)]
pub struct MintAsset<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub asset: Signer<'info>,

    #[account(
		has_one = update_authority
	)]
    pub collection: Account<'info, BaseCollectionV1>,

    /// CHECK: this account is not initiazed and is being used fro signing purposes only
    #[account(
		seeds = [b"update_authority", collection.key().as_ref()],
		bump,
	)]
    pub update_authority: UncheckedAccount<'info>,

    /// CHECK: THIS IS THE ID FOR THE MPL CORE PROGRAM
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> MintAsset<'info> {
    pub fn mint_asset(&mut self, name: String, uri: String, bumps: u8) -> Result<()> {
        let collections_key = self.collection.key();

        let signer_seeds: &[&[u8]; 3] = &[b"update_authority", collections_key.as_ref(), &[bumps]];

        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .authority(Some(&self.update_authority.to_account_info()))
            .payer(&self.user.to_account_info())
            .update_authority(Some(&self.user.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(name)
            .uri(uri)
            .invoke_signed(&[signer_seeds])?;

        Ok(())
    }
}

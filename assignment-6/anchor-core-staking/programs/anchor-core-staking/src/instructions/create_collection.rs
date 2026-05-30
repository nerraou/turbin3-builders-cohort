use anchor_lang::prelude::*;
use mpl_core::{instructions::CreateCollectionV2CpiBuilder, ID as MPL_CORE_ID};

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub collection: Signer<'info>,

    /// CHECK: this account is not initiazed and is being used fro signing purposes only
    #[account(
		seeds = [b"update_authority", collection.key().as_ref()],
		bump,
	)]
    pub update_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: THIS IS THE ID FOR THE MPL CORE PROGRAM
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> CreateCollection<'info> {
    pub fn create_collection(&mut self, name: String, uri: String, bumps: u8) -> Result<()> {
        let collections_key = self.collection.key();

        let signer_seeds: &[&[u8]; 3] = &[b"update_authority", collections_key.as_ref(), &[bumps]];

        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .payer(&self.user.to_account_info())
            .update_authority(Some(&self.update_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(name)
            .uri(uri)
            .invoke_signed(&[signer_seeds])?;

        Ok(())
    }
}

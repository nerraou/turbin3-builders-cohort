use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

	#[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

	#[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,


	#[account(
		mut,
		associated_token::mint = mint_a,
		associated_token::authority = maker,
		associated_token::token_program = token_program ,
	)]
	pub maker_ata_a : InterfaceAccount<'info, TokenAccount>,

	
	#[account(
		init,
		payer= maker,
		seeds = [b"escrow", maker.key().as_ref(), seeds.to_le_bytes().as_ref()],
		space = Escrow::DISCRIMINATOR.len() + Escrow::InitSpace,
		bump, 
	)]
	pub escrow : Account<'info, Escrow>,


	#[account(
		init,
		payer = maker,
		associated_token::mint = mint_a,
		associated_token::authority = escrow,
		associated_token::token_program = token_program
	)]
	pub vault : InterfaceAccount<'info, TokenAccount>,


	pub token_program : Interface<'info, TokenInterface>,
	pub assosiated_token_program: Program<'info, AssociatedToken>,
	pub system_program: Program<'info, System>,

}

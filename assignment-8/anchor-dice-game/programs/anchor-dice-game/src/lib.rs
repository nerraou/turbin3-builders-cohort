pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8hjbFtnfY87ZzpEpx26u5tx4KjxkrqiWGUEcWFbDNn7h");

#[program]
pub mod anchor_dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.initialize(amount)
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        seed: u128,
        amount: u64,
        guess_roll: u8,
    ) -> Result<()> {
        ctx.accounts
            .create_bet(seed, &ctx.bumps, guess_roll, amount)?;
        ctx.accounts.deposit(amount)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, sig: Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&ctx.bumps, &sig)
    }

    pub fn refund_bet(ctx: Context<RefundBet>) -> Result<()> {
        ctx.accounts.refund_bet(&ctx.bumps)
    }
}

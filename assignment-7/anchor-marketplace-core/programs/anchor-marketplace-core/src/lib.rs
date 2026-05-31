pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6KQmaV2t8mesPNWAmn4cWJY4FnwAAzqQDbzGET9Km5G2");

#[program]
pub mod anchor_marketplace_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}

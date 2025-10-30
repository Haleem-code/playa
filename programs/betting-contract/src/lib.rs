use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;
pub mod error;

pub use instructions::initialize::*;
pub use instructions::place_bet::*;
pub use instructions::declare_winner::*;
pub use instructions::payout_winners::*;
pub use instructions::payout_creator::*;
pub use instructions::payout_platform::*;

declare_id!("DRNEUsSx9gNre6f6mLFhrHDVRDfD4eMGu68dussziUgi");

#[program]
pub mod betting_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, stream_id: String, betting_deadline: i64, moderator_pubkey: Pubkey) -> Result<()> {
        instructions::initialize::handler(ctx, stream_id, betting_deadline, moderator_pubkey)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, prediction: u8, amount: u64) -> Result<()> {
        instructions::place_bet::handler(ctx, prediction, amount)
    }

    pub fn declare_winner(ctx: Context<DeclareWinner>, winning_outcome: u8) -> Result<()> {
        instructions::declare_winner::handler(ctx, winning_outcome)
    }
    
    pub fn payout_winners(ctx: Context<PayoutWinners>) -> Result<()> {
        instructions::payout_winners::handler(ctx)
    }
    
    pub fn payout_creator(ctx: Context<PayoutCreator>) -> Result<()> {
        instructions::payout_creator::handler(ctx)
    }
    
    pub fn payout_platform(ctx: Context<PayoutPlatform>) -> Result<()> {
        instructions::payout_platform::handler(ctx)
    }
}
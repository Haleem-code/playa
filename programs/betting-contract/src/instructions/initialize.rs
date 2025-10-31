use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::BettingError;

#[derive(Accounts)]
#[instruction(stream_id: String, betting_deadline: i64, moderator_pubkey: Pubkey,  platform_treasury: Pubkey)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = BettingPool::LEN,
        seeds = [b"betting_pool", stream_id.as_bytes()],
        bump
    )]
    pub betting_pool: Account<'info, BettingPool>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: Platform treasury can be any address
    pub platform_treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    stream_id: String,
    betting_deadline: i64,
    moderator_pubkey: Pubkey,
    platform_treasury: Pubkey,
) -> Result<()> {
    // Validate stream ID length
    if stream_id.len() > 32 {
        return Err(BettingError::StreamIdTooLong.into());
    }
    
    // Validate betting deadline is in the future
    let clock = Clock::get()?;
    if betting_deadline <= clock.unix_timestamp {
        return Err(BettingError::InvalidDeadline.into());
    }
    let pool_key = ctx.accounts.betting_pool.key();
    let admin_key = ctx.accounts.admin.key();
    let stream_id_clone = stream_id.clone();
    
    // Now take mutable reference
    let betting_pool = &mut ctx.accounts.betting_pool;
    
    // Initialize betting pool with your specifications
    betting_pool.admin = admin_key;

    betting_pool.moderator = moderator_pubkey;
    betting_pool.stream_id = stream_id.clone();
    betting_pool.total_pool = 0;
    betting_pool.player1_bets = 0;
    betting_pool.player2_bets = 0;
    betting_pool.player1_bet_count = 0;
    betting_pool.player2_bet_count = 0;
    betting_pool.winner_declared = false;
    betting_pool.winning_outcome = 0; // 0 means not set yet
    betting_pool.betting_deadline = betting_deadline;
    betting_pool.creator_fee_rate = 500;
    betting_pool.platform_fee_rate = 250;
    betting_pool.platform_treasury = platform_treasury;
    betting_pool.is_payout_complete = false;
    betting_pool.created_at = clock.unix_timestamp;
    betting_pool.bump = ctx.bumps.betting_pool;
    
    // Store values for event and logging before dropping mutable reference
    let betting_deadline = betting_pool.betting_deadline;
    let created_at = betting_pool.created_at;
    let platform_fee_rate = betting_pool.platform_fee_rate;
    
    // Emit event for frontend to track
    emit!(BettingPoolCreated {
        pool: pool_key,
        admin: admin_key,
        stream_id: stream_id_clone.clone(),
        betting_deadline,
        created_at,
    });
    
    msg!("Betting pool created for stream: {}", stream_id_clone);
    msg!("Betting closes at: {}", betting_deadline);
    msg!("Platform fee: {}%", platform_fee_rate as f64 / 100.0);
    
    Ok(())
}

// Event emitted when a new betting pool is created
#[event]
pub struct BettingPoolCreated {
    pub pool: Pubkey,
    pub admin: Pubkey,
    pub stream_id: String,
    pub betting_deadline: i64,
    pub created_at: i64,
}

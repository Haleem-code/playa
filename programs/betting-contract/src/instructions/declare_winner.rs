

use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::BettingError;

#[derive(Accounts)]
pub struct DeclareWinner<'info> {
    #[account(
        mut,
        seeds = [b"betting_pool", betting_pool.stream_id.as_bytes()],
        bump = betting_pool.bump,
    )]
    pub betting_pool: Account<'info, BettingPool>,
    #[account(mut)]
    pub signer: Signer<'info>, // Can be admin or moderator
}

pub fn handler(ctx: Context<DeclareWinner>, winning_outcome: u8) -> Result<()> {
    // Validate winning outcome (1 = Player 1 wins, 2 = Player 2 wins)
    if winning_outcome != 1 && winning_outcome != 2 {
        return Err(BettingError::InvalidWinningOutcome.into());
    }
    let signer_key = ctx.accounts.signer.key();
    let betting_pool = &mut ctx.accounts.betting_pool;
    // Check if signer is admin or moderator
    if signer_key != betting_pool.admin && signer_key != betting_pool.moderator {
        return Err(BettingError::UnauthorizedAdmin.into());
    }
    if betting_pool.winner_declared {
        return Err(BettingError::WinnerAlreadyDeclared.into());
    }
    let clock = Clock::get()?;
    betting_pool.winner_declared = true;
    betting_pool.winning_outcome = winning_outcome;
    let betting_pool_key = betting_pool.key();
    let stream_id = betting_pool.stream_id.clone();
    let total_pool = betting_pool.total_pool;
    let player1_bets = betting_pool.player1_bets;
    let player2_bets = betting_pool.player2_bets;
    let player1_bet_count = betting_pool.player1_bet_count;
    let player2_bet_count = betting_pool.player2_bet_count;
    let platform_fee = betting_pool.calculate_platform_fee();
    let prize_pool = betting_pool.prize_pool();
    let (winning_bets, winning_bet_count, losing_bets) = if winning_outcome == 1 {
        (player1_bets, player1_bet_count, player2_bets)
    } else {
        (player2_bets, player2_bet_count, player1_bets)
    };
    if winning_bets == 0 {
        return Err(BettingError::NoBetsOnWinningOutcome.into());
    }
    emit!(WinnerDeclared {
        betting_pool: betting_pool_key,
        admin: betting_pool.admin,
        stream_id: stream_id.clone(),
        winning_outcome,
        total_pool,
        winning_bets,
        winning_bet_count,
        losing_bets,
        platform_fee,
        prize_pool,
        declared_at: clock.unix_timestamp,
    });
    msg!("Winner declared for stream: {}", stream_id);
    msg!("Winning outcome: Player {}", winning_outcome);
    msg!("Total pool: {} lamports", total_pool);
    msg!("Winning bets total: {} lamports ({} bets)", winning_bets, winning_bet_count);
    msg!("Losing bets total: {} lamports", losing_bets);
    msg!("Platform fee: {} lamports", platform_fee);
    msg!("Prize pool for winners: {} lamports", prize_pool);
    Ok(())
}

// Event emitted when winner is declared
#[event]
pub struct WinnerDeclared {
    pub betting_pool: Pubkey,
    pub admin: Pubkey,
    pub stream_id: String,
    pub winning_outcome: u8,
    pub total_pool: u64,
    pub winning_bets: u64,
    pub winning_bet_count: u32,
    pub losing_bets: u64,
    pub platform_fee: u64,
    pub prize_pool: u64,
    pub declared_at: i64,
}
use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::BettingError;

#[derive(Accounts)]
pub struct PayoutWinners<'info> {
  
    #[account(
        mut,
        seeds = [b"betting_pool", betting_pool.stream_id.as_bytes()],
        bump = betting_pool.bump,
    )]
    pub betting_pool: Account<'info, BettingPool>,
    
    /// The winning bet being paid out
    #[account(
        mut,
        seeds = [
            b"bet",
            betting_pool.key().as_ref(),
            bet.user.as_ref(),
            &bet.bet_index.to_le_bytes()
        ],
        bump = bet.bump,
        has_one = betting_pool @ BettingError::InvalidBettingPool,
    )]
    pub bet: Account<'info, Bet>,
    
    /// CHECK: Winner account is validated against the betting pool's winner field in the handler logic
    #[account(mut)]
    pub winner: UncheckedAccount<'info>,
 
    /// CHECK: Platform treasury account is provided by the program and used for fee collection
    #[account(mut)]
    pub platform_treasury: UncheckedAccount<'info>,
    

    #[account(mut)]
    pub payer: Signer<'info>,
    

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PayoutWinners>) -> Result<()> {
    
    let betting_pool_key = ctx.accounts.betting_pool.key();
    let bet_key = ctx.accounts.bet.key();
    let winner_key = ctx.accounts.winner.key();
    let platform_treasury_key = ctx.accounts.platform_treasury.key();
    
  
    if !ctx.accounts.betting_pool.winner_declared {
        return Err(BettingError::WinnerNotDeclared.into());
    }
    
    
    if ctx.accounts.bet.is_paid_out {
        return Err(BettingError::BetAlreadyPaidOut.into());
    }
    

    if !ctx.accounts.bet.is_winner(ctx.accounts.betting_pool.winning_outcome) {
        return Err(BettingError::BetNotWinner.into());
    }
    

    if ctx.accounts.bet.user != winner_key {
        return Err(ProgramError::InvalidAccountData.into());
    }
    
   
    if ctx.accounts.betting_pool.admin != platform_treasury_key {
        return Err(BettingError::UnauthorizedAdmin.into());
    }
    
    let total_winning_bets = if ctx.accounts.betting_pool.winning_outcome == 1 {
        ctx.accounts.betting_pool.player1_bets
    } else {
        ctx.accounts.betting_pool.player2_bets
    };
    
    
    if total_winning_bets == 0 {
        return Err(BettingError::NoBetsOnWinningOutcome.into());
    }
    
  
    let prize_pool = ctx.accounts.betting_pool.prize_pool();
    let bet_share = (ctx.accounts.bet.amount as u128 * prize_pool as u128) / total_winning_bets as u128;
    let payout_amount = bet_share as u64;
    
   
    let total_platform_fee = ctx.accounts.betting_pool.calculate_platform_fee();
    let bet_platform_fee = (ctx.accounts.bet.amount as u128 * total_platform_fee as u128) / ctx.accounts.betting_pool.total_pool as u128;
    let platform_fee_amount = bet_platform_fee as u64;
    
    
    let betting_pool_lamports = ctx.accounts.betting_pool.to_account_info().lamports();
    if betting_pool_lamports < payout_amount + platform_fee_amount {
        return Err(BettingError::InsufficientFunds.into());
    }
    

    let stream_id = ctx.accounts.betting_pool.stream_id.clone();
    let winning_outcome = ctx.accounts.betting_pool.winning_outcome;
    let bet_amount = ctx.accounts.bet.amount;
    let prediction = ctx.accounts.bet.prediction;
    
    // Transfer winnings to winner
    **ctx.accounts.betting_pool.to_account_info().try_borrow_mut_lamports()? -= payout_amount;
    **ctx.accounts.winner.to_account_info().try_borrow_mut_lamports()? += payout_amount;
    
    // Transfer platform fee to treasury
    **ctx.accounts.betting_pool.to_account_info().try_borrow_mut_lamports()? -= platform_fee_amount;
    **ctx.accounts.platform_treasury.to_account_info().try_borrow_mut_lamports()? += platform_fee_amount;
    
    // Mark this bet as paid out
    ctx.accounts.bet.is_paid_out = true;
    
    let clock = Clock::get()?;
    
    // Emit payout event
    emit!(WinnerPaidOut {
        betting_pool: betting_pool_key,
        bet: bet_key,
        winner: winner_key,
        stream_id: stream_id.clone(),
        winning_outcome,
        bet_amount,
        payout_amount,
        platform_fee_amount,
        prediction,
        paid_out_at: clock.unix_timestamp,
    });
    
    // Log payout details
    msg!("Payout completed for stream: {}", stream_id);
    msg!("Winner: {}", winner_key);
    msg!("Original bet: {} lamports", bet_amount);
    msg!("Payout amount: {} lamports", payout_amount);
    msg!("Platform fee: {} lamports", platform_fee_amount);
    msg!("Winning outcome: Player {}", winning_outcome);
    
    Ok(())
}

// Event emitted when a winner is paid out
#[event]
pub struct WinnerPaidOut {
    pub betting_pool: Pubkey,
    pub bet: Pubkey,
    pub winner: Pubkey,
    pub stream_id: String,
    pub winning_outcome: u8,
    pub bet_amount: u64,
    pub payout_amount: u64,
    pub platform_fee_amount: u64,
    pub prediction: u8,
    pub paid_out_at: i64,
}
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::*;
use crate::error::BettingError;

#[derive(Accounts)]
#[instruction(prediction: u8, amount: u64)]
pub struct PlaceBet<'info> {
   
    #[account(
        mut,
        seeds = [b"betting_pool", betting_pool.stream_id.as_bytes()],
        bump = betting_pool.bump,
    )]
    pub betting_pool: Account<'info, BettingPool>,
    
  
    #[account(
        init,
        payer = user,
        space = Bet::LEN,
        seeds = [
            b"bet",
            betting_pool.key().as_ref(),
            user.key().as_ref(),
            &betting_pool.total_bet_count().to_le_bytes()
        ],
        bump
    )]
    pub bet: Account<'info, Bet>,
    
   
    #[account(mut)]
    pub user: Signer<'info>,
    
 
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PlaceBet>, prediction: u8, amount: u64) -> Result<()> {
    
    if prediction != 1 && prediction != 2 {
        return Err(BettingError::InvalidPrediction.into());
    }
    
 
    if amount == 0 {
        return Err(BettingError::InsufficientFunds.into());
    }
    
  
    if !ctx.accounts.betting_pool.is_betting_open() {
        return Err(BettingError::BettingClosed.into());
    }
    
    let clock = Clock::get()?;
    
    
    let user_key = ctx.accounts.user.key();
    let betting_pool_key = ctx.accounts.betting_pool.key();
    let bet_key = ctx.accounts.bet.key();
    let betting_pool_account_info = ctx.accounts.betting_pool.to_account_info();
    
  
    let transfer_instruction = system_program::Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: betting_pool_account_info,
    };
    
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
    );
    
    system_program::transfer(cpi_context, amount)?;
    
   
    let betting_pool = &mut ctx.accounts.betting_pool;
    
    // Store the current bet index before incrementing counters
    let bet_index = betting_pool.total_bet_count();
   
    betting_pool.total_pool = betting_pool.total_pool
        .checked_add(amount)
        .ok_or(BettingError::ArithmeticOverflow)?;
    
    if prediction == 1 {
        betting_pool.player1_bets = betting_pool.player1_bets
            .checked_add(amount)
            .ok_or(BettingError::ArithmeticOverflow)?;
        betting_pool.player1_bet_count = betting_pool.player1_bet_count
            .checked_add(1)
            .ok_or(BettingError::ArithmeticOverflow)?;
    } else {
        betting_pool.player2_bets = betting_pool.player2_bets
            .checked_add(amount)
            .ok_or(BettingError::ArithmeticOverflow)?;
        betting_pool.player2_bet_count = betting_pool.player2_bet_count
            .checked_add(1)
            .ok_or(BettingError::ArithmeticOverflow)?;
    }
    

    let bet = &mut ctx.accounts.bet;
    bet.user = user_key;
    bet.betting_pool = betting_pool_key;
    bet.amount = amount;
    bet.prediction = prediction;
    bet.timestamp = clock.unix_timestamp;
    bet.is_paid_out = false;
    bet.bet_index = bet_index;
    bet.bump = ctx.bumps.bet;
    
  
    let total_pool = betting_pool.total_pool;
    let player1_bets = betting_pool.player1_bets;
    let player2_bets = betting_pool.player2_bets;
    let stream_id = betting_pool.stream_id.clone();
    
    
    emit!(BetPlaced {
        bet: bet_key,
        user: user_key,
        betting_pool: betting_pool_key,
        stream_id: stream_id.clone(),
        prediction,
        amount,
        total_pool,
        player1_bets,
        player2_bets,
        timestamp: clock.unix_timestamp,
    });
    
   
    msg!("Bet placed on stream: {}", stream_id);
    msg!("User: {}", user_key);
    msg!("Prediction: Player {}", prediction);
    msg!("Amount: {} lamports", amount);
    msg!("Total pool: {} lamports", total_pool);
    msg!("Player 1 total: {} lamports", player1_bets);
    msg!("Player 2 total: {} lamports", player2_bets);
    
    Ok(())
}


#[event]
pub struct BetPlaced {
    pub bet: Pubkey,
    pub user: Pubkey,
    pub betting_pool: Pubkey,
    pub stream_id: String,
    pub prediction: u8,
    pub amount: u64,
    pub total_pool: u64,
    pub player1_bets: u64,
    pub player2_bets: u64,
    pub timestamp: i64,
}
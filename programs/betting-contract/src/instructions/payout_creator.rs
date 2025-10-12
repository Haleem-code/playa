use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::BettingError;

#[derive(Accounts)]
pub struct PayoutCreator<'info> {
    /// The betting pool account
    #[account(
        mut,
        seeds = [b"betting_pool", betting_pool.stream_id.as_bytes()],
        bump = betting_pool.bump,
        has_one = admin @ BettingError::UnauthorizedAdmin
    )]
    pub betting_pool: Account<'info, BettingPool>,
    
    /// The admin/creator who will receive the fee
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// System program
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PayoutCreator>) -> Result<()> {
    let betting_pool = &mut ctx.accounts.betting_pool;
    
    // Must have declared winner first
    require!(betting_pool.winner_declared, BettingError::WinnerNotDeclared);
    
    // Calculate creator fee (2.5%)
    let creator_fee = betting_pool.calculate_creator_fee();
    require!(creator_fee > 0, BettingError::NothingToPayout);
    
    // Transfer creator fee from pool to creator
    let pool_account = betting_pool.to_account_info();
    let admin_account = ctx.accounts.admin.to_account_info();
    
    **pool_account.try_borrow_mut_lamports()? -= creator_fee;
    **admin_account.try_borrow_mut_lamports()? += creator_fee;
    
    msg!("Creator fee paid: {} lamports ({} SOL)", creator_fee, creator_fee as f64 / 1_000_000_000.0);
    
    Ok(())
}

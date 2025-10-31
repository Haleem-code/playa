use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::BettingError;

#[derive(Accounts)]
pub struct PayoutPlatform<'info> {
    /// The betting pool account
    #[account(
        mut,
        seeds = [b"betting_pool", betting_pool.stream_id.as_bytes()],
        bump = betting_pool.bump,
    )]
    pub betting_pool: Account<'info, BettingPool>,
    
    /// CHECK: This is the platform's wallet address
    #[account(
        mut,
        constraint = platform_wallet.key() == betting_pool.platform_treasury @ BettingError::InvalidPlatformWallet
    )]
    pub platform_wallet: AccountInfo<'info>,
    
    /// Any authorized signer (can be anyone after winner is declared)
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// System program
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PayoutPlatform>) -> Result<()> {
    let betting_pool = &mut ctx.accounts.betting_pool;
    
    // Must have declared winner first
    require!(betting_pool.winner_declared, BettingError::WinnerNotDeclared);
    
    // Calculate platform fee (2.5%)
    let platform_fee = betting_pool.calculate_platform_fee();
    require!(platform_fee > 0, BettingError::NothingToPayout);
    
    // Transfer platform fee from pool to platform wallet
    let pool_account = betting_pool.to_account_info();
    let platform_account = ctx.accounts.platform_wallet.to_account_info();
    
    **pool_account.try_borrow_mut_lamports()? -= platform_fee;
    **platform_account.try_borrow_mut_lamports()? += platform_fee;
    
    msg!("Platform fee paid: {} lamports ({} SOL)", platform_fee, platform_fee as f64 / 1_000_000_000.0);
    
    Ok(())
}

use anchor_lang::prelude::*;

#[error_code]
pub enum BettingError {
    #[msg("Betting is closed for this pool")]
    BettingClosed,
    
    #[msg("Invalid betting prediction. Must be 1 (Player 1) or 2 (Player 2)")]
    InvalidPrediction,
    
    #[msg("Insufficient funds for bet")]
    InsufficientFunds,
    
    #[msg("Winner has already been declared for this pool")]
    WinnerAlreadyDeclared,
    
    #[msg("Winner has not been declared yet")]
    WinnerNotDeclared,
    
    #[msg("Only the admin can declare winners")]
    UnauthorizedAdmin,
    
    #[msg("Invalid winning outcome. Must be 1 (Player 1) or 2 (Player 2)")]
    InvalidWinningOutcome,
    
    #[msg("Payouts have already been completed")]
    PayoutsAlreadyCompleted,
    
    #[msg("This bet has already been paid out")]
    BetAlreadyPaidOut,
    
    #[msg("This bet is not a winner")]
    BetNotWinner,
    
    #[msg("Stream ID is too long (max 32 characters)")]
    StreamIdTooLong,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("No bets placed on winning outcome")]
    NoBetsOnWinningOutcome,
    
    #[msg("Invalid betting pool for this bet")]
    InvalidBettingPool,
    
    #[msg("Betting deadline must be in the future")]
    InvalidDeadline,
    
    #[msg("Invalid platform wallet")]
    InvalidPlatformWallet,
    
    #[msg("Nothing to payout")]
    NothingToPayout,
}
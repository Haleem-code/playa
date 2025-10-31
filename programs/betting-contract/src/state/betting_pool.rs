use anchor_lang::prelude::*;

#[account]
pub struct BettingPool {
    pub admin: Pubkey,
    pub moderator: Pubkey, // NEW: Public key of the designated moderator
    pub stream_id: String,
    pub total_pool: u64,
    pub player1_bets: u64,
    pub player2_bets: u64,
    pub player1_bet_count: u32,
    pub player2_bet_count: u32,
    pub winner_declared: bool,
    pub winning_outcome: u8,
    pub betting_deadline: i64,
    pub creator_fee_rate: u16,
    pub platform_fee_rate: u16,
    pub platform_treasury: Pubkey,
    pub is_payout_complete: bool,
    pub created_at: i64,
    pub bump: u8,
}

impl BettingPool {
   
    pub const LEN: usize = 8 +
        32 + // admin
        32 + // moderator
        4 + 32 + // stream_id (max 32 chars)
        8 + // total_pool
        8 + // player1_bets
        8 + // player2_bets
        4 + // player1_bet_count
        4 + // player2_bet_count
        1 + // winner_declared
        1 + // winning_outcome
        8 + // betting_deadline
        2 + // creator_fee_rate
        2 + // platform_fee_rate
        32 + // platform_treasury
        1 + // is_payout_complete
        8 + // created_at
        1; // bump

    pub fn is_betting_open(&self) -> bool {
        let now = Clock::get().unwrap().unix_timestamp;
        !self.winner_declared && now < self.betting_deadline
    }

 
    pub fn total_bet_count(&self) -> u32 {
        self.player1_bet_count + self.player2_bet_count
    }

    /// Calculate creator fee (2.5% of total pool)
    pub fn calculate_creator_fee(&self) -> u64 {
        (self.total_pool * self.creator_fee_rate as u64) / 10000
    }

    /// Calculate platform fee (2.5% of total pool)
    pub fn calculate_platform_fee(&self) -> u64 {
        (self.total_pool * self.platform_fee_rate as u64) / 10000
    }

    /// Calculate prize pool after deducting both creator and platform fees (95% of total)
    pub fn prize_pool(&self) -> u64 {
        self.total_pool - self.calculate_creator_fee() - self.calculate_platform_fee()
    }
}

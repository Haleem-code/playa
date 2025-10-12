use anchor_lang::prelude::*;

#[account]
pub struct BettingPool {
 
    pub admin: Pubkey,
    
    
    pub stream_id: String,
    
   
    pub total_pool: u64,
    
  
    pub player1_bets: u64,
  
    pub player2_bets: u64,
    
 
    pub player1_bet_count: u32,
    
   
    pub player2_bet_count: u32,
    
   
    pub winner_declared: bool,
    
    
    pub winning_outcome: u8,

    
    pub betting_deadline: i64,
    
    /// Creator fee rate in basis points (250 = 2.5%)
    pub creator_fee_rate: u16,
    
    /// Platform fee rate in basis points (250 = 2.5%)
    pub platform_fee_rate: u16,
    
   
    pub is_payout_complete: bool,
    
 
    pub created_at: i64,
    
    pub bump: u8,
}

impl BettingPool {
   
    pub const LEN: usize = 8 + 
        32 +  
        4 + 32 + 
        8 +   
        8 +   
        8 +   
        4 +  
        4 +   
        1 +   
        1 +   
        8 +  
        2 +   
        2 +   
        1 +   
        8 +   
        1;    

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
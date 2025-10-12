use anchor_lang::prelude::*;

#[account]
pub struct Bet {
    /// The user who placed this bet
    pub user: Pubkey,
    
    /// The betting pool this bet belongs to
    pub betting_pool: Pubkey,
    
    /// Amount of SOL bet (in lamports)
    pub amount: u64,
    
    /// User's prediction: 1 = Player 1 wins, 2 = Player 2 wins
    pub prediction: u8,
    
    /// Unix timestamp when this bet was placed
    pub timestamp: i64,
    
    /// Whether this bet has been paid out (prevents double payouts)
    pub is_paid_out: bool,
    
    /// The index of this bet in the pool (for PDA derivation)
    pub bet_index: u32,
    
    /// Bump seed for PDA derivation
    pub bump: u8,
}

impl Bet {
    /// Calculate space needed for this account
    /// 8 (discriminator) + account data
    pub const LEN: usize = 8 +
        32 +  // user: Pubkey
        32 +  // betting_pool: Pubkey
        8 +   // amount: u64
        1 +   // prediction: u8
        8 +   // timestamp: i64
        1 +   // is_paid_out: bool
        4 +   // bet_index: u32
        1;    // bump: u8

    /// Check if this bet is a winning bet
    pub fn is_winner(&self, winning_outcome: u8) -> bool {
        self.prediction == winning_outcome && winning_outcome != 0
    }

    /// Validate that prediction is either 1 or 2
    pub fn is_valid_prediction(&self) -> bool {
        self.prediction == 1 || self.prediction == 2
    }
}
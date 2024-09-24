#[allow(unused)]
use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub total_bets: u64,
    pub total_winnings: u64,
    pub total_losses: u64,
    pub total_draws: u64,
    pub bump: u8,
}

impl Space for User {
    const INIT_SPACE: usize = 8 + (8 * 4) + 1;
}

impl User {
    pub fn increase_bets(&mut self) {
        self.total_bets += 1;
    }

    pub fn increase_winnigs(&mut self) {
        self.total_winnings += 1;
    }

    pub fn increase_losses(&mut self) {
        self.total_losses += 1;
    }

    pub fn increase_draws(&mut self) {
        self.total_draws += 1;
    }

    pub fn decrease_bets(&mut self) {
        self.total_bets -= 1;
    }
}
use anchor_lang::prelude::*;

#[account]
pub struct House {
    pub admin: Pubkey,
    pub protocl_fees: i16,
    pub bump: u8,
    pub treasury_bump: u8,
}

impl Space for House {
    const INIT_SPACE: usize = 8 + 8 + 2 + (1 * 8);
}
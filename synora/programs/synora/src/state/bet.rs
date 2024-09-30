use anchor_lang::prelude::*;

#[account]
pub struct Bet {
    pub maker: Pubkey,
    pub opponent: Option<Pubkey>,
    pub token_mint: Pubkey, //the token mint he want to predict bet on
    pub odds: Odds,
    pub status: BetStatus,
    pub price_prediction: i64, // price prediction of the token_mint he provided
    pub deadline_to_join: i64, //opponent can join before this
    pub start_time: i64,       //bet start
    pub end_time: i64,         //bet end
    pub maker_deposit: u64,    //amount he's placing about the price movement in lamports
    pub amount_settled: bool,
    pub seed: u64,
    pub bump: u8,
    pub vault_pool_bump: u8,
    pub opponent_deposit: u64, //in lamports should store
    pub winner: Option<Pubkey>,
    //TODO - feed_injector constant should be defined here
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Odds {
    pub maker_odds: u64,
    pub opponent_odds: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum BetStatus {
    FindingOpponent,
    WaitingToStart,
    Ongoing,
    Completed,
}

impl Space for Bet {
    const INIT_SPACE: usize =
        8 + 8 + (1 + 32) + 32 + (8 + 8) + 1 + (8 * 5) + 1 + 4 + 1 + 1 + (1 + 32);
}
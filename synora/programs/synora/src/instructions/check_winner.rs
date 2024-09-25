use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use pyth_solana_receiver_sdk::get_feed_id_from_hex;

use crate::errors::Errors;
use crate::state::Bet;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct CheckWinner<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: BET MAKER ACCOUNT
    pub maker: UncheckedAccount<'info>,
    ///CHECK : BET OPPONENET
    pub opponent: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds=[b"bet",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump=bet.bump
    )]
    pub bet: Account<'info, Bet>,
    //also if necessary we need to add the both user accounts for updating their states
    // Add the Pyth price feed account
    pub pyth_price_account: Account<'info, PriceUpdateV2>,
}

impl<'info> CheckWinner<'info> {
    pub fn check_winner(&mut self) -> Result<()> {
        let signer: Pubkey = self.signer.key();
        let bet: &Account<'info, Bet> = &self.bet;
        let clock = Clock::get()?;
        // println!("the price is ({} ± {})", &clock);
        // ensure the bet has ended
        require!(clock.unix_timestamp >= bet.end_time, Errors::BetNotEndedYet);
        // ensure the signer is authorized (either the maker or opponent)
        require!(
            signer == bet.maker || bet.opponent.map_or(false, |opponent| signer == opponent),
            Errors::UnauthorizedAccess
        );
        //TODO : fetch data from the oracles and validate the bet 

        // fetching the current price-updates from the Pyth price account
        let price_update = &self.pyth_price_account;
        // get_price_no_older_than will fail if the price update is more than 30 seconds old
        let maximum_age: u64 = 30; // Ensure the price is no older than 30 seconds
        // This string is the id of the BTC/USD feed. See https://pyth.network/developers/price-feed-ids
        let feed_id: [u8; 32] = get_feed_id_from_hex("0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43")?; // BTC/USD feed ID
        let current_price = price_update.get_price_no_older_than(&clock, maximum_age, &feed_id)?;

        // Compare the current price with the bet's price prediction to determine the winner
        if current_price.price > bet.price_prediction {
            // Opponent wins
            bet.winner = Some(self.opponent.key());
            bet.status = BetStatus::Completed;
        } else if current_price.price < bet.price_prediction {
            // Maker wins
            bet.winner = Some(self.maker.key());
            bet.status = BetStatus::Completed;
        } else {
            // It's a draw
            bet.winner = None;
            bet.status = BetStatus::Completed;
        }
        // * Should test the fetching of the price
        Ok(())
    }
}
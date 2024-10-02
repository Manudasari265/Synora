use anchor_lang::prelude::*;

use switchboard_on_demand::{on_demand::accounts::pull_feed::PullFeedAccountData, prelude::rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal}};

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
    //? Adding the Switchboard price data feed account
    /// CHECK: This is the Switchboard feed account. We don't need to validate it here as we check it in the instruction logic.
    pub feed_injector: UncheckedAccount<'info>,
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

        // * Should test the fetching of the price
        Ok(())
    }

    //TODO : fetch data from the oracles and validate the bet 
    // * Should test the fetching of the price
    pub fn check_winner_bet_dummy(&mut self) -> Result<()> {
        //? Ensuring the bet hasn't been resolved yet
        require!(self.bet.winner.is_none(), Errors::BetAlreadyResolved);

        //? Ensuring the bet has an opponent and has been accepted
        require!(self.bet.opponent.is_some(), Errors::BetNotAccepted);

        //? For dummy implementation, simple random number to determine the winner is ok i guess!!
        let random_price = Clock::get()?.unix_timestamp as i64 % 1000; // Use timestamp as a source of randomness
        
        let creator_wins = if random_price >= self.bet.price_prediction {
            true
        } else {
            false
        };

        //? Setting the winner
        self.bet.winner = Some(if creator_wins {
            self.maker.key()
        } else {
            self.opponent.key()
        });

        msg!("Dummy winner determined. Random price: {}", random_price);
        Ok(())
    }

    pub fn check_winner_bet_switchboard(&mut self) -> Result<()> {
        //? again checking whether the bet has been resolved or not
        require!(self.bet.winner.is_none(), Errors::BetAlreadyResolved);

        //? ensuring the bet has an opp & has been accepted
        require!(self.bet.opponent.is_some(), Errors::BetNotAccepted);

        //? Accessing the feed data
        let feed_data_account = self.feed_injector.data.borrow();

        if self.feed_injector.key() != self.bet.feed_injector {
            return Err(Errors::MismatchFeed.into());
        }

        //? Parse the Switchboard data
        let parsed_feed = match PullFeedAccountData::parse(feed_data_account) {
            Ok(parsed_feed) => parsed_feed,
            Err(_e) => return Err(Errors::NoFeedData.into()),
        };
        //? Defining max stale slots and minimum samples
        let max_stale_slots = 300; // to prevent network delays
        let min_samples = 1; // latency issues
        let price_to_decimal = match parsed_feed.get_value(&Clock::get()?, max_stale_slots, min_samples, true) {
            Ok(price_to_decimal) => price_to_decimal,
            Err(_e) => return Err(Errors::NoValueFound.into()),
        };
        //? price conversion to u64 and multiplied by 10^10
        let multiplied_price = Decimal::checked_mul(price_to_decimal, Decimal::from_i64(10_i64.pow(10)).unwrap()).unwrap(); // to prevet decimaa precision

        let price_u64 = Decimal::to_u64(&multiplied_price)
            .ok_or(Errors::NoValueFound)?
            .checked_mul(10_u64.pow(10))
            .ok_or(Errors::PriceConversionOverflow)?;

        let initiator_wins = if self.bet.creator_estimate { // direction of the bet >= or <
            price_u64 >= self.bet.price_prediction.try_into().unwrap()
        } else {
            price_u64 < self.bet.price_prediction.try_into().unwrap()
        };
     
        self.bet.winner = Some(if initiator_wins {
            self.maker.key()
        } else {
            self.bet.opponent.unwrap()
        });

        Ok(())
    }
}
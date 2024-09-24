use anchor_lang::prelude::*;

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
    //also if necessary we need to add the both user accounts for updating there states
}

impl<'info> CheckWinner<'info> {
    pub fn check_winner(&mut self) -> Result<()> {
        let signer: Pubkey = self.signer.key();
        let bet: &Account<'info, Bet> = &self.bet;
        let clock = Clock::get()?;
        require!(clock.unix_timestamp >= bet.end_time, Errors::BetNotEndedYet);
        require!(
            signer == bet.maker || bet.opponent.map_or(false, |opponent| signer == opponent),
            Errors::UnauthorizedAccess
        );
        //TODO : fetch data from the oracles and validate the bet
        Ok(())
    }
}
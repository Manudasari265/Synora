use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::errors::Errors;
use crate::state::{Bet, User};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct AcceptBet<'info> {
    #[account(mut)]
    pub opponent: Signer<'info>,
    /// CHECK: BET MAKER ADDRESS FOR DERIVING PDA
    pub maker: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds=[b"bet",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump=bet.bump
    )]
    pub bet: Account<'info, Bet>,
    #[account(
        seeds=[b"vault",bet.key().as_ref()],
        bump=bet.vault_pool_bump
    )]
    pub vault_pool: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer=opponent,
        space=User::INIT_SPACE,
        seeds=[b"user_profile",opponent.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

impl<'info> AcceptBet<'info> {
    pub fn accept_bet(&mut self, bumps: &AcceptBetBumps) -> Result<()> {
        let user = &mut self.user_account;
        if user.total_bets == 0
            && user.total_winnings == 0
            && user.total_losses == 0
            && user.total_draws == 0
        {
            user.total_bets = 0;
            user.total_winnings = 0;
            user.total_losses = 0;
            user.total_draws = 0;
            user.bump = bumps.user_account
        }
        let clock = Clock::get()?;
        require!(
            self.bet.opponent.is_none() && clock.unix_timestamp < self.bet.deadline_to_join,
            Errors::EventAlreadyStarted
        );
        self.bet.opponent = Some(self.opponent.key());
        self.user_account.increase_bets();
        self.transfer_money()
    }

    pub fn transfer_money(&mut self) -> Result<()> {
        let accounts = Transfer {
            from: self.opponent.to_account_info(),
            to: self.vault_pool.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, self.bet.opponent_deposit)
    }
}
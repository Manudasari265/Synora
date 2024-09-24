use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::errors::Errors;
use crate::state::{Bet, User};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct CancelBet<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        close=maker,
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
        mut,
        seeds=[b"user_profile",maker.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

impl<'info> CancelBet<'info> {
    pub fn cancel_bet(&mut self) -> Result<()> {
        require!(self.bet.opponent.is_none(), Errors::EventCantCancel);
        self.user_account.decrease_bets();
        self.return_funds()
    }

    pub fn return_funds(&mut self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            self.bet.to_account_info().key.as_ref(),
            &[self.bet.vault_pool_bump],
        ]];
        let accounts = Transfer {
            from: self.vault_pool.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(ctx, self.vault_pool.lamports())
    }
}
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{errors::Errors, state::Bet, BetStatus, Odds, User};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct CreateBet<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        space=Bet::INIT_SPACE,
        payer=maker,
        seeds=[b"bet",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,
    #[account(
        seeds=[b"vault",bet.key().as_ref()],
        bump
    )]
    pub vault_pool: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer=maker,
        space=User::INIT_SPACE,
        seeds=[b"user_profile",maker.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateBet<'info> {
    pub fn create_bet(
        &mut self,
        token_mint: Pubkey,
        maker_odds: u64,
        opponent_odds: u64,
        price_prediction: i64,
        deadline_to_join: i64,
        start_time: i64,
        end_time: i64,
        amount: u64,
        seed: u64,
        bumps: &CreateBetBumps,
    ) -> Result<()> {
        //calculate the depositing amount
        require!(maker_odds == 1 || opponent_odds == 1, Errors::InvalidOdds);
        let odds = Odds {
            maker_odds,
            opponent_odds,
        };
        let opponent_deposit = self.calculate_opponent_deposit(amount, &odds);
        self.bet.set_inner(Bet {
            maker: self.maker.key(),
            opponent: None,
            token_mint,
            odds,
            status: BetStatus::FindingOpponent,
            price_prediction,
            deadline_to_join,
            start_time,
            end_time,
            maker_deposit: amount, //sol stored in lamports
            amount_settled: false,
            seed,
            bump: bumps.bet,
            vault_pool_bump: bumps.vault_pool,
            opponent_deposit, //sol in lamports
            winner: None,
        });

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
        self.user_account.increase_bets();
        self.send_money_to_vault(amount)
    }

    pub fn calculate_opponent_deposit(&mut self, maker_deposit: u64, odds: &Odds) -> u64 {
        if odds.maker_odds == 1 {
            return maker_deposit * odds.opponent_odds;
        }
        maker_deposit / odds.maker_odds
    }

    pub fn send_money_to_vault(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.maker.to_account_info(),
            to: self.vault_pool.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount)
    }
}
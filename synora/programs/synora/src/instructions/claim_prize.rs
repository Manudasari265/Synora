use anchor_lang::{prelude::*, system_program::{Transfer,transfer}};

use crate::{state::Bet, BetStatus, Errors};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub winner: Signer<'info>,
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
        bump
    )]
    pub vault_pool: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimPrize<'info> {
    pub fn claim_prize(&mut self) -> Result<()> {
        let signer= self.winner.key();
        let bet= &self.bet;
        let clock = Clock::get()?;
        require!(clock.unix_timestamp >= bet.end_time, Errors::BetNotEndedYet);
        require!(
            signer == bet.maker.key() || bet.opponent.map_or(false, |opponent| signer == opponent.key()),
            Errors::UnauthorizedAccess
        );
        require!(bet.status==BetStatus::Completed,Errors::BetNotResolvedYet);
        //TODO:MAY BE DO SOME MORE CHECKS IF NEEDED
        self.transfer_amount()
}
    pub fn transfer_amount(&mut self)->Result<()>{
        let amount=self.vault_pool.lamports();
        let accounts=Transfer{
                from:self.vault_pool.to_account_info(),
                to:self.winner.to_account_info()
        };
        let binding_key=self.maker.key();
        let bump_binding=[self.bet.vault_pool_bump];
        let signer_seeds=&[&[b"vault",binding_key.as_ref(),&bump_binding][..]];
        let ctx=CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);
        //TODO: Need to cut some fees for protocol
        transfer(ctx, amount)
    }
}
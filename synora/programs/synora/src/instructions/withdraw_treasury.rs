use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::House;

#[derive(Accounts)]
pub struct WithdrawTreasury<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"house", admin.key().as_ref()],
        bump=house.bump
    )]
    pub house: Account<'info, House>,
    #[account(
        seeds=[b"treasury",house.key().as_ref()],
        bump=house.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawTreasury<'info> {
    pub fn withdraw_treasury(&mut self) -> Result<()> {
        let binding_key = self.house.key();
        let bump_binding = [self.house.treasury_bump];
        let signer_seeds = &[&[b"treasury", 
        binding_key.as_ref(), 
        &bump_binding][..]];

        
        let accounts = Transfer {
            from: self.treasury.to_account_info(),
            to: self.admin.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(ctx, self.treasury.lamports())
    }
}
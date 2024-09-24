use anchor_lang::prelude::*;

use crate::state::House;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = House::INIT_SPACE,
        seeds = [b"house", admin.key().as_ref()],
        bump
    )]
    pub house: Account<'info, House>,
    #[account(
        seeds=[b"treasury",house.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Init<'info> {
    pub fn init(&mut self, fees: i16, bumps: &InitBumps) -> Result<()> {
        self.house.set_inner(House {
            admin: self.admin.key(),
            protocl_fees: fees,
            bump: bumps.house,
            treasury_bump: bumps.treasury,
        });
        Ok(())
    }
}
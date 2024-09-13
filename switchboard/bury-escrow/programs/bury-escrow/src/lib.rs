use anchor_lang::prelude::*;
use instructions::deposit::*;
use instructions::withdraw::*;
use state::*;
 
pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("E5kv2j41SfsrZyCeEohk8SQ3i71Yzgiv32ey8ekeL5mQ");

#[program]
pub mod bury_escrow {
    use super::*;
 
    pub fn deposit(ctx: Context<Deposit>, escrow_amt: u64, unlock_price: u64) -> Result<()> {
        deposit_handler(ctx, escrow_amt, unlock_price)
    }
 
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

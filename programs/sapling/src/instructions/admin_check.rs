use anchor_lang::prelude::*;

use crate::Config;
use crate::error::ErrorCode;

pub fn auth_as_admin(ctx: Context<AuthAsAdmin>) -> Result<()> {
    let caller = ctx.accounts.caller.key();
    let admin = ctx.accounts.config.admin;
    msg!("Validating admin");
    require!(caller == admin, ErrorCode::Unauthorized);
    Ok(())
}

#[derive(Accounts)]
pub struct AuthAsAdmin<'info> {
    #[account(mut)]
    caller: Signer<'info>,

    config: Account<'info, Config>,
}

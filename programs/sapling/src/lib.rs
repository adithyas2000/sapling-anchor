use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
pub use instructions::*;
pub mod error;

declare_id!("DFeuaY7xMpbwKRWUbK82X6XJRnMDLNXFad429ysuSVgR");

#[program]
pub mod sapling {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config_acc = &mut ctx.accounts.config;
        config_acc.admin = ctx.accounts.deployer.key();
        msg!("Set {} as admin", config_acc.admin.to_string());
        msg!("Mint created: {}", ctx.accounts.mint.key());
        Ok(())
    }
    pub fn add_tree_variant(
        ctx: Context<AddTreeVariant>,
        id: String,
        name: String,
        cost_per_month: u64,
        max_lifetime_in_months: u64,
    ) -> Result<()> {
        add_tree_variant_as_admin(ctx, id, name, cost_per_month, max_lifetime_in_months)
    }

    pub fn remove_tree_variant(ctx: Context<RemoveTreeVariant>, id: String) -> Result<()> {
        remove_tree_variant_as_admin(ctx, id)
    }

    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        msg!(
            "Created ATA for signer: {}",
            ctx.accounts.token_account.key()
        );
        Ok(())
    }

    pub fn rent_tree(
        ctx: Context<RentTree>,
        rent_tree_id: String,
        rent_duration_months: u64,
    ) -> Result<()> {
        rent(ctx, rent_tree_id, rent_duration_months)
    }
}

#[account]
pub struct Config {
    admin: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=deployer,seeds=[b"config"],bump,space=8+32)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub deployer: Signer<'info>,
    #[account(
        init,
        payer=deployer,
        mint::decimals=0,
        mint::authority=mint.key(),
        mint::freeze_authority=mint.key(),
        seeds=[b"token_mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = deployer,
        token::mint = mint,
        token::authority = token_account,
        token::token_program = token_program,
        seeds = [b"token_account"],
        bump
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        associated_token::mint=mint,
        associated_token::authority=signer,
        associated_token::token_program=token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds=[b"token_mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

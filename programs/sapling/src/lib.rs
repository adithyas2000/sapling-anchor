use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub use instructions::*;
pub mod error;

declare_id!("CmUSs1qB9t4Wtv73DvBqf3XH8L8BLnLWhnXzCA3Qrf5L");

#[program]
pub mod sapling {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config_acc = &mut ctx.accounts.config;
        config_acc.admin = *ctx.accounts.deployer.key;
        msg!("Set {} as admin", config_acc.admin.to_string());
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
    pub system_program: Program<'info, System>,
}

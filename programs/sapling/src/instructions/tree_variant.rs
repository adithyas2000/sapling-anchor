use crate::error::ErrorCode;
use crate::{state::TreeVariant, Config};
use anchor_lang::prelude::*;

pub fn add_tree_variant_as_admin(
    ctx: Context<AddTreeVariant>,
    id: String,
    name: String,
    cost_per_month: u64,
    max_lifetime_in_months: u64,
) -> Result<()> {
    let new_variant = &mut ctx.accounts.tree_variant;
    new_variant.tree_type_id = id;
    new_variant.tree_type_name = name;
    new_variant.cost_per_month = cost_per_month;
    new_variant.max_lifetime_in_months = max_lifetime_in_months;
    new_variant.is_active = true;
    Ok(())
}

pub fn remove_tree_variant_as_admin(_ctx: Context<RemoveTreeVariant>, _id: String) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(id:String)]
pub struct AddTreeVariant<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds=[b"config"],bump,has_one=admin @ ErrorCode::Unauthorized)]
    pub config: Account<'info, Config>,

    #[account(init,payer=admin,seeds=[b"tree_variant",id.as_bytes()],bump,space=8+TreeVariant::INIT_SPACE)]
    pub tree_variant: Account<'info, TreeVariant>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(_id:String)]
pub struct RemoveTreeVariant<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds=[b"config"],bump,has_one=admin @ ErrorCode::Unauthorized)]
    pub config: Account<'info, Config>,
    #[account(mut,close=admin,seeds=[b"tree_variant",_id.as_bytes()],bump)]
    pub tree_variant: Account<'info, TreeVariant>,
    pub system_program: Program<'info, System>,
}

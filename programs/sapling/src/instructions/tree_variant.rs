use crate::error::ErrorCode;
use crate::{
    state::{TreeVariant, TreeVariantAccount},
    Config,
};
use anchor_lang::prelude::*;

pub fn add_tree_variant_as_admin(
    ctx: Context<AddTreeVariant>,
    id: String,
    name: String,
    cost_per_month: u64,
    max_lifetime_in_months: u64,
) -> Result<()> {
    let new_variant = TreeVariant {
        const_per_month: cost_per_month,
        tree_type_id: id,
        tree_type_name: name,
        max_lifetime_in_months: max_lifetime_in_months,
    };
    ctx.accounts
        .tree_variant_account
        .tree_variants
        .push(new_variant);
    Ok(())
}

pub fn remove_tree_variant_as_admin(ctx: Context<RemoveTreeVariant>, id: String) -> Result<()> {
    let variants = &mut ctx.accounts.tree_variant_account.tree_variants;
    let matching_variant = variants.iter().position(|tree| tree.tree_type_id == id);
    if let Some(i) = matching_variant {
        variants.remove(i);
        Ok(())
    } else {
        err!(ErrorCode::TreeVariantNotFound)
    }
}

#[derive(Accounts)]
pub struct AddTreeVariant<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds=[b"config"],bump,has_one=admin @ ErrorCode::Unauthorized)]
    pub config: Account<'info, Config>,

    #[account(mut,seeds=[b"tree_variant"],bump)]
    pub tree_variant_account: Account<'info, TreeVariantAccount>,
}
#[derive(Accounts)]
pub struct RemoveTreeVariant<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds=[b"config"],bump,has_one=admin @ ErrorCode::Unauthorized)]
    pub config: Account<'info, Config>,

    #[account(mut,seeds=[b"tree_variant"],bump)]
    pub tree_variant_account: Account<'info, TreeVariantAccount>,
}

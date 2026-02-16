use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TreeVariant {
    pub const_per_month: u64,
    pub tree_type_id: String,
    pub tree_type_name: String,
    pub max_lifetime_in_months: u64,
}

#[account]
pub struct TreeVariantAccount {
    pub tree_variants: Vec<TreeVariant>,
}

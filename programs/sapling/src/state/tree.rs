use anchor_lang::prelude::*;
#[derive(InitSpace)]
#[account]
pub struct TreeVariant {
    pub cost_per_month: u64,
    #[max_len(50)]
    pub tree_type_id: String,
    #[max_len(50)]
    pub tree_type_name: String,
    pub max_lifetime_in_months: u64,
    pub is_active: bool,
}
#[account]
#[derive(InitSpace)]
pub struct UserTreeRental {
    #[max_len(50)]
    pub tree_type_id: String,
    pub duration_in_months: u64,
}

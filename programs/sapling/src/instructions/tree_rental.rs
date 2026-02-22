use anchor_lang::prelude::*;

use crate::state::UserTreeRental;

pub fn rent(ctx: Context<RentTree>, rent_tree_id: String, rent_duration_months: u64) -> Result<()> {
    ctx.accounts.tree_rental_pda.tree_type_id = rent_tree_id;
    ctx.accounts.tree_rental_pda.duration_in_months = rent_duration_months;
    msg!("REntal account created: {}",ctx.accounts.tree_rental_pda.key());
    Ok(())
}

#[derive(Accounts)]
#[instruction(rent_tree_id:String,rent_duration_months:u64)]
pub struct RentTree<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,payer=signer,seeds=[&signer.key.to_bytes(),rent_tree_id.as_bytes(),&rent_duration_months.to_le_bytes()],bump,space=8+UserTreeRental::INIT_SPACE)]
    pub tree_rental_pda: Account<'info, UserTreeRental>,
    pub system_program: Program<'info, System>,
}

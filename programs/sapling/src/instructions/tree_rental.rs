use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::MintTo,
    token_interface::{self, Mint, Token2022, TokenAccount},
};

// use crate::state::UserTreeRental;

pub fn rent(
    ctx: Context<RentTree>,
    _rent_tree_id: String,
    _rent_duration_months: u64,
) -> Result<()> {
    msg!("Derived mint address: {}", ctx.accounts.mint.key());
    let signer_seeds: &[&[&[u8]]] = &[&[b"token_mint", &[ctx.bumps.mint]]];
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.associated_token_account.to_account_info(),
        authority: ctx.accounts.mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);
    let result = token_interface::mint_to(cpi_ctx, 1);
    msg!("minted 1 token to the signer");
    result
}

#[derive(Accounts)]
// #[instruction(rent_tree_id:String,rent_duration_months:u64)]
pub struct RentTree<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,seeds=[b"token_mint"],bump)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    #[account(
        mut,
        seeds = [b"token_account"],
        bump
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint=mint,
        associated_token::authority=signer,
        associated_token::token_program=token_program
    )]
    pub associated_token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

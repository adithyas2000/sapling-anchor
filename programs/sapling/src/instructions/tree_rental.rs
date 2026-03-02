use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::MintTo,
    token_interface::{self, Mint, Token2022, TokenAccount},
};

use crate::init_token_metadata;

pub fn rent(ctx: Context<RentTree>, rent_tree_id: String, rent_duration_months: u64) -> Result<()> {
    msg!("Derived mint address: {}", ctx.accounts.mint.key());
    let signer_address = ctx.accounts.signer.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"token_mint",
        signer_address.as_ref(),
        rent_tree_id.as_bytes(),
        &[ctx.bumps.mint],
    ]];
    msg!("Setting metadata...");
    init_token_metadata(&ctx, signer_seeds)?;
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.associated_token_account.to_account_info(),
        authority: ctx.accounts.mint.to_account_info(),
    };
    msg!("Setting metadata in account {}...",ctx.accounts.tree_metadata_account.key());
    ctx.accounts.tree_metadata_account.condition = "OK".to_string();
    ctx.accounts.tree_metadata_account.mint = ctx.accounts.mint.key();
    ctx.accounts.tree_metadata_account.level = 1;
    ctx.accounts.tree_metadata_account.owner = ctx.accounts.signer.key();
    ctx.accounts.tree_metadata_account.tree_id = (&rent_tree_id).to_string();
    ctx.accounts.tree_metadata_account.remaining_months = rent_duration_months;
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);
    let result = token_interface::mint_to(cpi_ctx, 1);
    msg!("minted 1 token to the signer");
    result
}

#[derive(Accounts)]
#[instruction(rent_tree_id:String,rent_duration_months:u64)]
pub struct RentTree<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer=signer,
        mint::decimals=0,
        mint::authority=mint.key(),
        mint::freeze_authority=mint.key(),
        seeds=[b"token_mint",signer.key().as_ref(),rent_tree_id.as_bytes()],
        extensions::metadata_pointer::authority = signer,
        extensions::metadata_pointer::metadata_address = mint,
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=signer,
        seeds=[b"tree_metadata",signer.key().as_ref(),rent_tree_id.as_bytes()],
        bump,
        space=TreeMetadata::INIT_SPACE
    )]
    pub tree_metadata_account: Account<'info, TreeMetadata>,
    pub token_program: Program<'info, Token2022>,
    #[account(
        init,
        payer = signer,
        token::mint = mint,
        token::authority = token_account,
        token::token_program = token_program,
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
#[derive(InitSpace)]
#[account]
pub struct TreeMetadata {
    mint: Pubkey,
    level: u8,
    owner: Pubkey,
    #[max_len(50)]
    tree_id: String,
    #[max_len(50)]
    condition: String,
    remaining_months: u64,
}

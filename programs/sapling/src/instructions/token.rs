use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use anchor_spl::token_interface::{token_metadata_initialize, TokenMetadataInitialize};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

use crate::{error::ErrorCode, RentTree};

pub fn init_token_metadata(ctx: &Context<RentTree>,signer_seeds:&[&[&[u8]]]) -> Result<()> {

    let name = "Sapling tree".to_string();
    let symbol = "TREE".to_string();
    let uri = "https://raw.githubusercontent.com/adithyas2000/sapling-anchor/refs/heads/main/metadata.json".to_string();
    let token_metadata = TokenMetadata {
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        ..Default::default()
    };
    let data_len = 4 + token_metadata
        .get_packed_len()
        .map_err(|_| error!(ErrorCode::MetadataPackingError))?;
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(data_len);
    msg!("Transfering {} lamports to mint account", lamports);
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.mint.to_account_info(),
            },
        ),
        lamports,
    )?;

    msg!("Setting up metadata...");
    token_metadata_initialize(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataInitialize {
                program_id: ctx.accounts.token_program.to_account_info(),
                metadata: ctx.accounts.mint.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                mint_authority: ctx.accounts.mint.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            signer_seeds,
        ),
        name,
        symbol,
        uri,
    )
}

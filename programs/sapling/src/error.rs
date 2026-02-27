use anchor_lang::prelude::*;
#[error_code]
pub enum ErrorCode {
    #[msg("Tree variant not found")]
    TreeVariantNotFound,
    #[msg("Caller is not the admin")]
    Unauthorized,
    #[msg("Error packing metadata")]
    MetadataPackingError
}

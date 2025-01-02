use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Below min health factor")]
    BelowMinHealthFactor,
    #[msg("Can not liquidate a healthy account")]
    AboveMinHealthFactor,
}
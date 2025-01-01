use anchor_lang::prelude::*;

declare_id!("231DaM3L5iSYy44TjTzJWCdEZBXsfHC1ChksyN2pTHX6");


use constants::*;
use instructions::*;
use error::*;
use state::*;
mod constants;
mod error;
mod instructions;
mod state;

#[program]
pub mod stablecoin {

    use super::*;


    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_tokens(
        ctx: Context<DepositCollateralAndMintTokens>, 
        amount_collateral: u64, 
        amount_to_mint: u64
    ) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_to_mint)
    }
}
 

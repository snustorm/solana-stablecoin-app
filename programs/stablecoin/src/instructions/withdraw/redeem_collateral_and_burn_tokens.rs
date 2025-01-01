use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::check_health_factor;
use crate::state::{Collateral, Config};
use crate::constants::{
    SEED_COLLATERAL_ACCOUNT, 
    SEED_CONFIG_ACCOUNT,
    SEED_SOL_ACCOUNT,
};




#[derive(Accounts)]
pub struct RedeemCollateralAndBurnTokens<'info> {

    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump = collateral_account.bump,
        has_one = sol_account,
        has_one = mint_account,
    )]
    pub collateral_account: Account<'info, Collateral>,

    pub price_update: Account<'info, PriceUpdateV2>,

}
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{check_health_factor, burn_tokens, withdraw_sol};
use crate::state::{Collateral, Config};
use crate::constants::{
    SEED_COLLATERAL_ACCOUNT, 
    SEED_CONFIG_ACCOUNT,
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
        has_one = token_account,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,  
    pub system_program: Program<'info, System>,

    pub price_update: Account<'info, PriceUpdateV2>,

}

pub fn process_redeem_collateral_and_burn_tokens(
    ctx: Context<RedeemCollateralAndBurnTokens>,
    amount_collateral: u64,
    amount_to_burn: u64,
) -> Result<()> {

    let collaral_account = &mut ctx.accounts.collateral_account;
    collaral_account.lamport_balance -= ctx.accounts.sol_account.lamports() - amount_collateral;
    collaral_account.amount_minted -= amount_to_burn;

    check_health_factor(
        &ctx.accounts.collateral_account, 
        &ctx.accounts.config_account, 
        &ctx.accounts.price_update,
    )?;

    burn_tokens(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        &ctx.accounts.depositor,
        amount_to_burn,
    )?;

    withdraw_sol(
        &ctx.accounts.system_program,
        &ctx.accounts.sol_account,
        &ctx.accounts.depositor,
        ctx.accounts.collateral_account.bump,
        &ctx.accounts.depositor.key(),
        amount_collateral,
    )?;

    Ok(())
}
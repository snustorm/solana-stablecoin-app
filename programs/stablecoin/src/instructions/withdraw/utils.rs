use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use anchor_spl::token_2022::{burn, Burn};
use crate::constants::{SEED_SOL_ACCOUNT};

pub fn withdraw_sol<'info>(
    system_program: &Program<'info, System>,
    from: &SystemAccount<'info>,
    to: &Signer<'info>,
    bump: u8,
    depositor_key: &Pubkey,
    amount: u64  
) -> Result<()>{


    let signer_seeds: &[&[&[u8]]] = &[&[
        SEED_SOL_ACCOUNT, 
        depositor_key.as_ref(),
        &[bump]]];

    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    Ok(())
}

pub fn burn_tokens<'info>(
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,  
    authority: &Signer<'info>,
    amount: u64,
) -> Result<()>{


    burn(
        CpiContext::new(
            token_program.to_account_info(),
            Burn {
                mint: mint_account.to_account_info(),
                from: token_account.to_account_info(),
                authority: authority.to_account_info()
            },
        ),
        amount,
    )

}
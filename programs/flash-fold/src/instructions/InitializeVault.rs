use anchor_lang::prelude::*;
use crate::states::{Vault,Session};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct InitializeVault<'info>{
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init_if_needed,
        space = 8 + Vault::INIT_SPACE,
        payer = authority,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Box<Account<'info,Vault>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}
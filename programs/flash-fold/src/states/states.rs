use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault{
    pub owner: Pubkey,
    pub vault_balance: u64,
    pub mint_account: Pubkey
}

#[account]
pub struct Session{
    pub createdAt: u64,
    pub endedAt: u64,
    pub is_active: bool,
}


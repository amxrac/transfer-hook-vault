use anchor_lang::prelude::*;

use crate::state::{Config, WhitelistEntry};

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(
        mut,
        constraint = admin.key() == config.admin @ ErrorCode::ConstraintAddress
    )]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        close = admin,
        seeds = [b"whitelist", user.key().as_ref()],
        bump,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
    pub system_program: Program<'info, System>,
}

impl<'info> RemoveFromWhitelist<'info> {
    pub fn remove_from_whitelist(&mut self) -> Result<()> {
        Ok(())
    }
}

use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::state::Whitelist;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(
        mut,
        //address =
    )]
    pub admin: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = admin,
        seeds = [b"whitelist", mint.key().as_ref(), user.as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> RemoveFromWhitelist<'info> {
    pub fn remove_from_whitelist(&mut self, _address: Pubkey) -> Result<()> {
        Ok(())
    }
}

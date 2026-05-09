use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECKED: user wallet
    pub user: UncheckedAccount<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 32 +1, // 8 bytes for discriminator, 32 bytes for wallet Pubkey, 32 bytes for mint Pubkey, 1 byte for bump
        seeds = [b"whitelist", mint.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWhitelist<'info> {
    pub fn initialize_whitelist(&mut self, bumps: InitializeWhitelistBumps) -> Result<()> {
        // Initialize the whitelist with an empty address vector
        self.whitelist.set_inner(Whitelist {
            address: Pubkey::default(),
            mint: Pubkey::default(),
            bump: bumps.whitelist,
        });

        Ok(())
    }
}

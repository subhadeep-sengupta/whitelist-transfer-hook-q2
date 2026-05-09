use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::state::Whitelist;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct AddToWhitelist<'info> {
    #[account(
        mut,
        //address =
    )]
    pub admin: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 32 +1, // 8 bytes for discriminator, 32 bytes for wallet Pubkey, 32 bytes for mint Pubkey, 1 byte for bump
        seeds = [b"whitelist", mint.key().as_ref(), user.as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddToWhitelist<'info> {
    pub fn add_to_whitelist(&mut self, user: Pubkey, bumps: AddToWhitelistBumps) -> Result<()> {
        msg!("mint: {}", self.mint.key());
        msg!("whitelist passed: {}", self.whitelist.key());

        let (expected, _) = Pubkey::find_program_address(
            &[b"whitelist", self.mint.key().as_ref(), user.as_ref()],
            &crate::ID,
        );
        msg!("expected: {}", expected);
        self.whitelist.set_inner(Whitelist {
            address: user,
            mint: self.mint.key(),
            bump: bumps.whitelist,
        });
        Ok(())
    }
}

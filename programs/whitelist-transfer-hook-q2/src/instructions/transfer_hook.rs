use std::cell::RefMut;

use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{Mint, TokenAccount},
};

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [b"whitelist", mint.key().as_ref(), owner.key().as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
}

impl<'info> TransferHook<'info> {
    /// This function is called when the transfer hook is executed.
    pub fn transfer_hook(&mut self, _amount: u64) -> Result<()> {
        // Fail this instruction if it is not called from within a transfer hook
        msg!("mint: {}", self.mint.key());
        msg!("owner (index 3): {}", self.owner.key());
        msg!("whitelist PDA passed: {}", self.whitelist.key());

        let (expected, _) = Pubkey::find_program_address(
            &[
                b"whitelist",
                self.mint.key().as_ref(),
                self.owner.key().as_ref(),
            ],
            &crate::ID,
        );
        msg!("expected whitelist PDA: {}", expected);
        self.check_is_transferring()?;

        require_keys_eq!(self.whitelist.mint, self.mint.key());
        require_keys_eq!(self.whitelist.address, self.owner.key());

        msg!("Transfer allowed: owner is whitelisted");

        Ok(())
    }

    /// Checks if the transfer hook is being executed during a transfer operation.
    fn check_is_transferring(&mut self) -> Result<()> {
        // // Ensure that the source token account has the transfer hook extension enabled
        //
        // // Get the account info of the source token account
        // let source_token_info = self.source_token.to_account_info();
        // // Borrow the account data mutably
        // let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
        //
        // // Unpack the account data as a PodStateWithExtensionsMut
        // // This will allow us to access the extensions of the token account
        // // We use PodStateWithExtensionsMut because TokenAccount is a POD (Plain Old Data) type
        // let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
        // // Get the TransferHookAccount extension
        // // Search for the TransferHookAccount extension in the token account
        // // The returning struct has a `transferring` field that indicates if the account is in the middle of a transfer operation
        // let account_extension = account.get_extension_mut::<TransferHookAccount>()?;
        //
        // // Check if the account is in the middle of a transfer operation
        // if !bool::from(account_extension.transferring) {
        //     panic!("TransferHook: Not transferring");
        // }
        let source_token_info = self.source_token.to_account_info();
        let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;

        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

        require!(
            bool::from(account_extension.transferring),
            TransferHookError::NotTransferring
        );
        Ok(())
    }
}

#[error_code]
pub enum TransferHookError {
    #[msg("Transfer hook invoked outside a transferring context")]
    NotTransferring,
}

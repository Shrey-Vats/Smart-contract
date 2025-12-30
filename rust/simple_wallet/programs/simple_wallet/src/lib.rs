pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("G6XUfo19ozWKDnMgA745J26dtib62qqpUofUsSYXWqkE");

const MIN_DEPOSIT: u64 = 1_000_000;

#[program]
pub mod simple_wallet {
    use anchor_lang::prelude::program::invoke_signed;

    use crate::{error::ErrorCode};

    use super::*;

    pub fn initialize(ctx: Context<Initializefn>) -> Result<()> {
        ctx.accounts.vault.amount = 0;
        ctx.accounts.vault.authority = ctx.accounts.user.key();
        ctx.accounts.vault.bump = *&ctx.accounts.vault.bump;

        emit!(InitializefnEvent {
            user: ctx.accounts.user.key(),
            amount: 0
        });

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, deposit: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let user = &ctx.accounts.user;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &user.key(),
            &vault.key(),
            deposit,
        );


        require!(deposit > MIN_DEPOSIT, 
        ErrorCode::InvalidAmount);

        let rent = Rent::get()?;
        let min_balance = rent.minimum_balance(vault.to_account_info().data_len());

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                user.to_account_info(),
                vault.to_account_info(),
            ],
        )?;

        let stored_amount = deposit.checked_sub(min_balance).ok_or(ErrorCode::InvalidAmount)?;

        ctx.accounts.vault.amount = vault
            .amount
            .checked_add(stored_amount)
            .ok_or(ErrorCode::AmountOverflow)?;

        emit!(DepositEvent {
            user: ctx.accounts.user.key(),
            deposit_amount: deposit - min_balance
        });

        return Ok(());
    }

    pub fn withdraw(ctx: Context<Withdraw>, withdraw_amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let vault = &ctx.accounts.vault;

        require!(user.key() == vault.authority, ErrorCode::Unauthorized);
        require!(withdraw_amount > MIN_DEPOSIT, ErrorCode::InvalidAmount);
        
        require!(
            vault.amount >= withdraw_amount,
            ErrorCode::InsufficientFunds
        );

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &vault.key(),
            &user.key(),
            withdraw_amount,
        );

        let user_key = ctx.accounts.user.key();

        let seeds: &[&[&[u8]]] = &[&[b"vault", user_key.as_ref(), &[vault.bump]]];

        invoke_signed(
            &ix,
            &[vault.to_account_info(), user.to_account_info()],
            &seeds
        )?;

        ctx.accounts.vault.amount = ctx
            .accounts
            .vault
            .amount
            .checked_sub(withdraw_amount)
            .ok_or(ErrorCode::InsufficientFunds)?;

        emit!(WithdrawEvent {
            user: user.key(),
            withdraw_amount: withdraw_amount
        });
        return Ok(());
    }
}

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("ufhNZNFMj6DhKZWrmiaSxFF7nAZRcYkBUrnJZ8QJ3Xk");

#[program]
pub mod escrow {

    use super::*;
    use crate::error::ErrorCode;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let escrow = &mut ctx.accounts.escrow_account;

        require!(
            escrow.amount == 0,
            ErrorCode::AlreadyDeposit
        );

        let to = ctx.accounts.to.key();
        let from = ctx.accounts.from.key();

        require!(to != from, ErrorCode::InvalidParties);

        escrow.to = to;
        escrow.from = from;
        escrow.amount = amount;
        escrow.bump = ctx.bumps.escrow_account;

        emit!(InitializeEvent {
            to,
            from,
            amount,
            escrow_pda: ctx.accounts.escrow_account.key()
        });

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, time_days: i64) -> Result<()> {
        let from = &ctx.accounts.from;
        let escrow = &mut ctx.accounts.escrow_account;

        require!(time_days > 0, ErrorCode::InvalidTime);

        // transfer logic
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &from.key(),
            &escrow.key(),
            escrow.amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[from.to_account_info(), escrow.to_account_info()],
        )?;

        let end_date = Clock::get()?.unix_timestamp + time_days * 24 * 60 * 60;

        escrow.end_date = end_date;

        emit!(DepositEvent {
            from: from.key(),
            to: escrow.to,
            amount: escrow.amount,
            end_date: end_date,
            escrow_pda: escrow.key()
        });

        Ok(())
    }

    pub fn released(ctx: Context<Released>) -> Result<()> {
        require!(
            Clock::get()?.unix_timestamp > ctx.accounts.escrow_account.end_date,
            ErrorCode::TimeExceed
        );
        require!(
            ctx.accounts.escrow_account.amount > 0,
            ErrorCode::DepositRequried
        );
        require!(
            ctx.accounts.escrow_account.to == ctx.accounts.to.key(),
            ErrorCode::MisMatchSeller
        );

        let escrow = &mut ctx.accounts.escrow_account;
        let amount = escrow.amount;
        require!(amount > 0, ErrorCode::InvalidAmount);

        let to = &ctx.accounts.to;
        let from = escrow.from;

        escrow.amount = 0;
        transfer_from_escrow(
            &escrow,
            &ctx.accounts.to,
            &ctx.accounts.system_program,
            amount
        )?;

        emit!(ReleasedEvent {
            amount,
            to: to.key(),
            from,
            escrow_pda: escrow.key(),
            end_date: escrow.end_date
        });

        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
        require!(
            Clock::get()?.unix_timestamp < escrow.end_date,
            ErrorCode::TimeExceed
        );
        let amount = escrow.amount;
        require!(amount > 0, ErrorCode::InvalidAmount);

        let to = escrow.to;
        let from = escrow.from;

        require!(amount > 0, ErrorCode::DepositRequried);

        escrow.amount = 0;

        transfer_from_escrow(
            &escrow,
            &ctx.accounts.from,
            &ctx.accounts.system_program,
            amount
        )?;

        emit!(RefundEvent {
            to,
            from,
            amount,
            escrow_pda: escrow.key(),
            end_date: escrow.end_date
        });

        Ok(())
    }

    pub fn extract(ctx: Context<Extract>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow_account;
      
        require!(
            ctx.accounts.receiver.key() == escrow.to,
            ErrorCode::MisMatchSeller
        );

        // require!(ctx.accounts.from.key() ==  escrow.from , ErrorCode::MisMatchSigner);

        require!(
            escrow.end_date < Clock::get()?.unix_timestamp,
            ErrorCode::TooEarlyToExtract
        );
        let amount = escrow.amount;
        require!(amount > 0, ErrorCode::InvalidAmount);

        let from = escrow.from;
        let to = escrow.to;

        escrow.amount = 0;
        transfer_from_escrow(
            &escrow,
            &ctx.accounts.receiver,
            &ctx.accounts.system_program,
            amount
        )?;

        emit!(ExtractEvent {
            to,
            from,
            amount,
            end_date: escrow.end_date,
            escrow_pda: escrow.key()
        });

        Ok(())
    }
}

pub fn transfer_from_escrow<'info>(
    escrow: &Account<'info, Escrow>,
    to: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
    amount: u64
) -> Result<()> {
    let seeds = &[
        b"escrow",
        escrow.from.as_ref(),
        escrow.to.as_ref(),
        &[escrow.bump],
    ];

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &escrow.key(),
        &to.key(),
        amount,
    );

    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            escrow.to_account_info(),
            to.clone(),
            system_program.to_account_info(),
        ],
        &[seeds],
    )?;

    Ok(())
}

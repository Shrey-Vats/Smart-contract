pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use anchor_spl::token::{self, Token, TokenAccount, Transfer};
pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("ufhNZNFMj6DhKZWrmiaSxFF7nAZRcYkBUrnJZ8QJ3Xk");

#[program]
pub mod escrow {

    use anchor_spl::token::{self, Transfer};

    use super::*;
    use crate::error::ErrorCode;

    pub fn initialize(ctx: Context<Initialize>, amount: u64, escrow_id: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);

        let escrow = &mut ctx.accounts.escrow_account;
        let mint = &ctx.accounts.mint;

        require!(escrow.amount == 0, ErrorCode::AlreadyDeposit);

        let to = ctx.accounts.to.key();
        let from = ctx.accounts.from.key();

        require!(to != from, ErrorCode::InvalidParties);

        escrow.to = to;
        escrow.from = from;
        escrow.amount = amount;
        escrow.bump = ctx.bumps.escrow_account;
        escrow.escrow_id = escrow_id;
        escrow.mint = mint.key();
        escrow.status = EscrowStatus::Initialized;

        emit!(InitializeEvent {
            escrow_id,
            to,
            from,
            amount,
            escrow_pda: ctx.accounts.escrow_account.key(),
            mint: mint.key()
        });

        Ok(())
    }

    pub fn fund(ctx: Context<Funds>, time_days: i64) -> Result<()> {
        let from = &ctx.accounts.from;
        let from_ata = &ctx.accounts.from_ata;

        let escrow = &mut ctx.accounts.escrow_account;
        let escrow_ata = &ctx.accounts.escrow_ata;

        let token_program = &ctx.accounts.token_program;

        require!(time_days > 0, ErrorCode::InvalidTime);
        let cpi_account = Transfer {
            from: from_ata.to_account_info(),
            to: escrow_ata.to_account_info(),
            authority: from.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();

        token::transfer(CpiContext::new(cpi_program, cpi_account), escrow.amount)?;

        let end_date = Clock::get()?.unix_timestamp + (time_days * 24 * 60 * 60);

        escrow.end_date = end_date;
        escrow.status = EscrowStatus::Funded;

        emit!(FundEvent {
            from: from.key(),
            to: escrow.to,
            amount: escrow.amount,
            end_date: end_date,
            escrow_pda: escrow.key()
        });

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {

        let escrow = &mut ctx.accounts.escrow_account;
        let now = Clock::get()?.unix_timestamp;

        if now > escrow.end_date {
            require_keys_eq!(
                ctx.accounts.from.key(),
                escrow.to,
                ErrorCode::MisMatchSeller
            );
        } else {
            require_keys_eq!(
                ctx.accounts.from.key(),
                escrow.from,
                ErrorCode::MisMatchSeller
            );
        }

        let amount = escrow.amount;

        let to = &ctx.accounts.to;
        let to_ata = &ctx.accounts.to_ata;
        let from = escrow.from;

        escrow.amount = 0;
        escrow.status = EscrowStatus::Claimed;

        transfer_from_escrow(&escrow, to_ata, &ctx.accounts.token_program, amount)?;

        emit!(ReleasedEvent {
            amount,
            to: to.key(),
            from,
            escrow_pda: escrow.key(),
            end_date: escrow.end_date
        });

        Ok(())
    }

}

pub fn transfer_from_escrow<'info>(
    escrow: &Account<'info, Escrow>,
    to: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let escrow_id = escrow.escrow_id.to_be_bytes();

    let seeds = &[
        b"escrow",
        escrow.from.as_ref(),
        escrow.to.as_ref(),
        escrow_id.as_ref(),
        &[escrow.bump],
    ];

    let cpi_accounts = Transfer {
        from: escrow.to_account_info(),
        to: to.to_account_info(),
        authority: escrow.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();

    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &[seeds]),
        amount,
    )?;

    Ok(())
}

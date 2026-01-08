pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EK5Xh1wxNEqQ7DZGr9aog5YVjhVkqHvn2FunB12Pa9SP");

#[program]
pub mod allowance_system {
    use anchor_spl::token::{self, transfer, Transfer};

    use crate::error::ErrorCode;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        vault.bump = ctx.bumps.vault;
        vault.owner = ctx.accounts.owner.key();

        emit!(InitializeEvent {
            owner: vault.owner.key(),
            vault: ctx.accounts.vault.key()
        });

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault_ata = &ctx.accounts.vault_ata;
        let owner_ata = &ctx.accounts.owner_ata;
        let owner = &ctx.accounts.owner;
        let token_program = &ctx.accounts.token_program;

        require!(owner_ata.amount >= amount, ErrorCode::InsufficentBalance);

        let cpi_account = Transfer {
            authority: owner.to_account_info(),
            from: owner_ata.to_account_info(),
            to: vault_ata.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();

        token::transfer(CpiContext::new(cpi_program, cpi_account), amount)?;

        emit!(DepositEvent {
            owner: owner.key(),
            vault_ata: vault_ata.key(),
            amount
        });

        Ok(())
    }

    pub fn approve(ctx: Context<Approve>, amount: u64) -> Result<()> {
        let allowance_account = &mut ctx.accounts.allowance_account;

        let owner = &ctx.accounts.owner;
        let spender = &ctx.accounts.spender;
        let vault_ata = &ctx.accounts.vault_ata;
        let mint_account = &ctx.accounts.mint_account;

        require!(amount > 0, ErrorCode::InvaildAmount);
        require!(vault_ata.amount >= amount, ErrorCode::InsufficentBalance);
        require_keys_eq!(mint_account.key(), vault_ata.mint.key(), ErrorCode::InvaildMint);

        allowance_account.owner = owner.key();
        allowance_account.spender = spender.key();
        allowance_account.mint = mint_account.key();
        allowance_account.vault = ctx.accounts.vault.key();

        allowance_account.remaining_amount = amount;

        allowance_account.bump = ctx.bumps.allowance_account;

        emit!(ApproveEvent {
            owner: owner.key(),
            spender: spender.key(),
            allowance_account: allowance_account.key(),
            mint_account: mint_account.key(),
            approve_amount: amount
        });

        Ok(())
    }

    pub fn spend_from_allowance(ctx: Context<SpendFromAllowance>, amount: u64) -> Result<()> {
        let allowance_account = &mut ctx.accounts.allowance_account;
        let vault = &ctx.accounts.vault;

        let reciver_ata = &ctx.accounts.reciver_ata;
        let vault_ata = &ctx.accounts.vault_ata;

        let token_program = &ctx.accounts.token_program;
        let sender = &ctx.accounts.sender.key();

        require!(allowance_account.remaining_amount >= amount, ErrorCode::InsufficentBalance);
        require!(&allowance_account.spender == sender, ErrorCode::Unauthorized);
        
        require_keys_eq!(allowance_account.mint, vault_ata.mint, ErrorCode::InvaildMint);
        require_keys_eq!(allowance_account.mint, reciver_ata.mint, ErrorCode::InvaildMint);


        allowance_account.remaining_amount = allowance_account
            .remaining_amount
            .checked_sub(amount)
            .ok_or(ErrorCode::FailedToDecrease)?;

        let seeds = &[
            b"vault",
            vault.owner.as_ref(),
            &[vault.bump],
        ];
        transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: vault_ata.to_account_info(),
                    to: reciver_ata.to_account_info(),
                    authority: vault.to_account_info(),
                },
                &[seeds],
            ),
            amount,
        )?;

        emit!(SpendFromAllowanceEvent {
            owner: allowance_account.owner,
            spender: allowance_account.spender,
            vault_ata: vault_ata.key(),
            reciver_ata: reciver_ata.key(),
            amount
        });

        Ok(())
    }

    pub fn revoke_allowance(
        ctx: Context<RevokeAllowance>,
        optional_remove_amount: Option<u64>,
    ) -> Result<()> {
        let allowance_account = &mut ctx.accounts.allowance_account;
        
        let owner = &ctx.accounts.owner;
        let spender = allowance_account.spender;

        let amount:u64;
        if let Some(value) = optional_remove_amount {
            require!(
                allowance_account.remaining_amount >= value,
                ErrorCode::InsufficentBalance
            );
            amount = value;

            allowance_account.remaining_amount = allowance_account
                .remaining_amount
                .checked_sub(value)
                .ok_or(ErrorCode::FailedToDecrease)?;
        } else {
            amount = allowance_account.remaining_amount;

            allowance_account
                .close(owner.to_account_info())?;
        }

        emit!(RevokeAllowanceEvent {
            owner: owner.key(),
            spender: spender,
            revoke_amount: amount,
        });

        Ok(())
    }
}

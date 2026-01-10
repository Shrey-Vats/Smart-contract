pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod helper;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use helper::*;

declare_id!("9J3NwFMJMEHZiTn7wh14G8Ae8ZL2FtEasJtrrNiMjfMz");

#[program]
pub mod erc4626_program {
    use anchor_spl::token::{self, transfer, Transfer};

    use crate::error::ErrorCode;

    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let sender = &mut ctx.accounts.sender;

        require!(sender.key() == CREATOR_KEY, ErrorCode::Unauthorized);

        require!(vault.mint == Pubkey::default(), ErrorCode::VaultAlreadyInitialized);

        vault.total_assets = 0;
        vault.total_shares = 0;
        vault.mint = ctx.accounts.mint.key();

        vault.bump = ctx.bumps.vault;

        emit!(InitializeVaultEvent {
            sender: sender.key(),
            mint: ctx.accounts.mint.key(),
            vault: vault.key()
        });

        Ok(())
    }

    pub fn initialize_position(ctx: Context<Initialize>) -> Result<()> {
        let position = &mut ctx.accounts.position;
        let owner = &ctx.accounts.owner;

        position.owner = owner.key();
        position.vault = ctx.accounts.vault.key();
        position.shares_owned = 0;
        position.bump = ctx.bumps.position;

        emit!(InitializeEvent {
            vault: ctx.accounts.vault.key(),
            owner: owner.key(),
            position: position.key()
        });

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let owner_ata = &ctx.accounts.owner_ata;
        let vault_ata = &ctx.accounts.vault_ata;

        let owner = &ctx.accounts.owner;

        let position = &mut ctx.accounts.position;
        let vault = &mut ctx.accounts.vault;

        let user_assets = amount as u128;

        require!(owner_ata.amount >= amount, ErrorCode::InsufficientBalance);

        let cpi_account = Transfer {
            from: owner_ata.to_account_info(),
            to: vault_ata.to_account_info(),
            authority: owner.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        token::transfer(CpiContext::new(cpi_program, cpi_account), amount)?;

        let user_share = if vault.total_shares == 0 {
            user_assets
        } else {
            convert_to_shares(user_assets, vault.total_assets, vault.total_shares)?
        };

        vault.total_assets = vault
            .total_assets
            .checked_add(user_assets)
            .ok_or(ErrorCode::MathOverflow)?;

        vault.total_shares = vault
            .total_shares
            .checked_add(user_share)
            .ok_or(ErrorCode::MathOverflow)?;

        position.shares_owned = position
            .shares_owned
            .checked_add(user_share)
            .ok_or(ErrorCode::MathOverflow)?;

        emit!(DepositEvent {
            vault: vault.key(),
            owner:  owner.key(),
            position_pda: position.key(),
            deposit_assets:  user_assets,
            user_share
        });

        Ok(())
    }
 
    pub fn withdraw(ctx: Context<Withdraw>, shares_amount: Option<u128>) -> Result<()> {
        let vault_ata = &ctx.accounts.vault_ata;
        let user_ata = &ctx.accounts.owner_ata;

        let vault = &mut ctx.accounts.vault;
        let position = &mut ctx.accounts.position;
        let token_program = &ctx.accounts.token_program;

        let user_share: u128;

        require!(position.vault == vault.key(), ErrorCode::InvalidVault);
        require!(vault.total_shares > 0, ErrorCode::InvalidVaultState);

        if let Some(withdraw_shares) = shares_amount {
            user_share = withdraw_shares;
            require!(
                position.shares_owned >= withdraw_shares,
                ErrorCode::InsufficientShares
            );
            require!(withdraw_shares as u64 > 0, ErrorCode::InsufficientShares);
        } else {
            user_share = position.shares_owned;
        }

        let user_assets = convert_to_assets(user_share, vault.total_assets, vault.total_shares)?;

        vault.total_assets = vault
            .total_assets
            .checked_sub(user_assets)
            .ok_or(ErrorCode::MathOverflow)?;

        vault.total_shares = vault
            .total_shares
            .checked_sub(user_share)
            .ok_or(ErrorCode::MathOverflow)?;
        position.shares_owned = position
            .shares_owned
            .checked_sub(user_share)
            .ok_or(ErrorCode::MathOverflow)?;

        transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                Transfer {
                    from: vault_ata.to_account_info(),
                    to: user_ata.to_account_info(),
                    authority: vault.to_account_info(),
                },
                &[&[b"vault", vault.mint.as_ref(), &[vault.bump]]],
            ),
            user_assets as u64,
        )?;

        if position.shares_owned == 0 {
            position.close(ctx.accounts.owner.to_account_info())?;
        }

        emit!(WithdrawEvent {
            owner: ctx.accounts.owner.key(),
            vault: vault.key(),
            position: position.key(),
            withdrew_assets: user_assets,
            withdrew_share: user_share
        });

        Ok(())
    }
}


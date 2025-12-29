pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("DitT8nhRyDEvJFRPTnEzumZ7mxHVRjTL9TM8oUQmniU1");

#[program]
pub mod counter {
    use crate::error::ErrorCode;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.user = ctx.accounts.user.key();

        emit!(CounterInitialize {
            user: counter.user.key(),
            value: counter.count
        });

        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        if ctx.accounts.user.key() != ctx.accounts.counter.user {
            return Err(ErrorCode::Unauthorized.into());
        }

        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_add(1).ok_or(ErrorCode::Overflow)?;

        emit!(CounterUpdated {
            user: ctx.accounts.user.key(),
            new_value: counter.count
        });

        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        if ctx.accounts.user.key() != ctx.accounts.counter.user {
            return Err(ErrorCode::Unauthorized.into());
        }

        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count.checked_sub(1).ok_or(ErrorCode::Underflow)?;

        Ok(())
    }
}

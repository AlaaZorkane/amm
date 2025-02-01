#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("escSmDEY26evSYow7Nio1WtkNFneo95DTq83P4myqer");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(mut ctx: Context<InitializeAccounts>, input: InitializeInput) -> Result<()> {
        _initialize(&mut ctx, &input)
    }

    pub fn deposit(mut ctx: Context<DepositAccounts>, input: DepositInput) -> Result<()> {
        _deposit(&mut ctx, &input)
    }

    pub fn withdraw(mut ctx: Context<WithdrawAccounts>, input: WithdrawInput) -> Result<()> {
        _withdraw(&mut ctx, &input)
    }

    pub fn swap(mut ctx: Context<SwapAccounts>, input: SwapInput) -> Result<()> {
        _swap(&mut ctx, &input)
    }
}

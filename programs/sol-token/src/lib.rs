use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, InitializeMint, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_mint(ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_mint(cpi_ctx, decimals, &ctx.accounts.authority.key(), None)
    }

    pub fn mint_to(ctx: Context<MintToCtx>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(init, payer = authority, space = 82)]
    pub mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintToCtx<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: Program<'info, token::Token>,
}

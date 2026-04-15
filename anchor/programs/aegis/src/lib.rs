use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer}; // Added these imports

declare_id!("2ZDYfDcLhgZHNTB34RoGSBDRNCXmJPStdT8sqthGbu83"); 

#[program]
pub mod aegis_finance {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = *ctx.accounts.authority.key;
        vault.total_deposits = 0;
        vault.shield_surplus = 0;
        vault.bump = ctx.bumps.vault;

        msg!("Aegis Vault Initialized!");
        msg!("Authority: {}", vault.authority);
        msg!("Project 56 & 57 Safety Net Active.");
        Ok(())
    }

    // Logic for Task 2: The Deposit
    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // 1. Transfer USDC from Tenant to the Vault's Token Account
        let cpi_accounts = Transfer {
            from: ctx.accounts.tenant_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.tenant.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;

        // 2. Update the Accounting for Project 56/57
        vault.total_deposits += amount;

        msg!("Deposit Successful: {} USDC added to Aegis Vault.", amount);
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub authority: Pubkey,      // 32 bytes
    pub total_deposits: u64,    // 8 bytes (USDC Principal)
    pub shield_surplus: u64,    // 8 bytes (Accumulated Yield)
    pub bump: u8,               // 1 byte (PDA security)
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 1,
        seeds = [b"aegis_vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Added this struct so the program knows which accounts to validate for deposits
#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut, seeds = [b"aegis_vault"], bump = vault.bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub tenant: Signer<'info>,
    #[account(mut)]
    pub tenant_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
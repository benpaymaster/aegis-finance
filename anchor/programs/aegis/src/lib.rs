use anchor_lang::prelude::*;

// Replace this with the ID generated when you click 'Build' in Solana Playground
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
        // Space: 8 (discriminator) + 32 (pubkey) + 8 (u64) + 8 (u64) + 1 (u8)
        space = 8 + 32 + 8 + 8 + 1,
        seeds = [b"aegis_vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
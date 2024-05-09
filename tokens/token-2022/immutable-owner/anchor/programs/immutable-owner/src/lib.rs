use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::{
    token_2022::{
        initialize_account3,
        spl_token_2022::{extension::ExtensionType, pod::PodAccount},
        InitializeAccount3,
    },
    token_interface::{immutable_owner_initialize, ImmutableOwnerInitialize, Mint, Token2022},
};

declare_id!("6g5URpqqurW8RbKjuGeRCVZBKky3J4kYcLeotQ6vj6UT");

#[program]
pub mod immutable_owner {
    use super::*;

    // There is currently not an anchor constraint to automatically initialize the ImmutableOwner extension
    // We can manually create and initialize the token account via CPIs in the instruction handler
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Calculate space required for token and extension data
        let token_account_size = ExtensionType::try_calculate_account_len::<PodAccount>(&[
            ExtensionType::ImmutableOwner,
        ])?;

        // Calculate minimum lamports required for size of token account with extensions
        let lamports = (Rent::get()?).minimum_balance(token_account_size);

        // Invoke System Program to create new account with space for token account and extension data
        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                },
            ),
            lamports,                          // Lamports
            token_account_size as u64,         // Space
            &ctx.accounts.token_program.key(), // Owner Program
        )?;

        // Initialize the token account with the immutable owner extension
        immutable_owner_initialize(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            ImmutableOwnerInitialize {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                token_account: ctx.accounts.token_account.to_account_info(),
            },
        ))?;

        // Initialize the standard token account data
        initialize_account3(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            InitializeAccount3 {
                account: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ))?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub token_account: Signer<'info>,
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

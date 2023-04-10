use anchor_lang::prelude::*;

// Define the program ID as a unique identifier for the program
declare_id!("97ico5tgMcM8xyeumNUroM51bKgnjjWgSbVdxqYPJYVb");

// Declare a module named `transfer_one_sol` for the program
#[program]
pub mod transfer_one_sol {
    use super::*;

    // Define a function named `send_one_sol` that takes a context and a message string
    pub fn send_one_sol(ctx: Context<Transaction>, msg: String) -> Result<()> {
        // Create a transfer instruction to transfer 1 SOL from `from` to `to`
        let transfer = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            1000000000,
        );

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info().clone(),
            ],
        )
        // Log an error if the transfer fails
        .expect("Error");

        // Log a message indicating that the transfer was successful
        msg!("{}", msg);

        // Return `Ok` to indicate that the function executed successfully
        Ok(())
    }
}

// Define a struct named `Transaction` that represents the accounts used by the program
#[derive(Accounts)]
pub struct Transaction<'info> {
    pub system_program: Program<'info, System>, // The System program account
    /// CHECK: This is the signer
    #[account(mut, signer)]
    pub from: AccountInfo<'info>, // The sender's account
    /// CHECK: This is the receiver
    #[account(mut)]
    pub to: AccountInfo<'info>, // The receiver's account
}

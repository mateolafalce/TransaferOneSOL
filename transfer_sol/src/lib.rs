use anchor_lang::prelude::*;

declare_id!("97ico5tgMcM8xyeumNUroM51bKgnjjWgSbVdxqYPJYVb");

#[program]
pub mod transfer_one_sol {
    use super::*;

    pub fn send_one_sol(ctx: Context<Transaction>, msg: String) -> Result<()> {
        // Create a transfer instruction to transfer 1 SOL from `from` to `to`
        let transfer = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            1000000000,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info().clone(),
            ],
        )
        .expect("Error");

        msg!("{}", msg);
        
        Ok(())
    }
}

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

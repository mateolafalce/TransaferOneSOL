use anchor_lang::prelude::*;

declare_id!("97ico5tgMcM8xyeumNUroM51bKgnjjWgSbVdxqYPJYVb");

#[program]
pub mod transfer_one_sol {
    use super::*;

    pub fn send_one_sol(ctx: Context<Transaction>, msg: String) -> Result<()> {
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
    pub system_program: Program<'info, System>,
    /// CHECK: This is the signer
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is receiver
    #[account(mut)]
    pub to: AccountInfo<'info>,
}

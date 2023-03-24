<div align="center">

![transfer_one_sol](transfer_one_sol.gif)

<h1>Transafer one SOL</h1>

</div>

<div style="text-align: justify;">

This program aims to transfer SOL natively on the Solana blockchain to another private key. The implementation was done using Rust to have applicability in desktop or mobile applications that wish to use Rust as the main language. In web development, there are already specific tools such as integrated wallets or desktop extensions that provide the same administrative security for SOL in the account, but they have a centralized structure that has been penetrated in several cases.

The importance of decentralized development of something as critical as payment methods has been implemented in this way of transferring SOL natively with a programming language as secure as Rust. The input of the private key is implemented to use the file system of the operating system in question.

To sign, the user's private key is required, which must be requested in JSON format. After obtaining the signature, the Solana system program is called to make a modification of the accounts in question, specifically their balance, and then the gas fee for the respective transaction will be paid.

<h3 align="center">Transfer program</h3>

</div>

```rust
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
```
This function and its related structure define a transaction that sends a fixed amount of Solana cryptocurrency (1 SOL) from a source account to a destination account. The send_one_sol function takes two arguments: ctx, which is a context object provided by the Anchor framework, and msg, an optional text string. The function returns a result value indicating whether the transaction completed successfully or failed.

Within the send_one_sol function, the system_instruction library provided by the Solana program is used to construct a transfer instruction. This instruction is later used in the invoke function to transfer 1 SOL from the from account to the to account. The invoke function also takes an array of AccountInfo references that represent the accounts involved in the transaction.

A message for the recipient is requested as input (which will later be printed via msg!). A variable that manages the sending of SOL could be dynamically implemented as an argument.

The Transaction structure defines the accounts that are used in the transaction, which include the system program account, and the from and to accounts. The from account is defined as a signer account and is marked as mutable, indicating that it will be modified during the transaction. The to account is also marked as mutable, but not as a signing account, which means that no additional signature will be required to modify it during the transaction.

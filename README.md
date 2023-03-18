<div align="center">

![transfer_one_sol](transfer_one_sol.gif)

<h1>Transafer one SOL</h1>

</div>

<div style="text-align: justify;">

This program aims to transfer SOL natively on the Solana blockchain to another private key. The implementation was done using Rust to have applicability in desktop or mobile applications that wish to use Rust as the main language. In web development, there are already specific tools such as integrated wallets or desktop extensions that provide the same administrative security for SOL in the account, but they have a centralized structure that has been penetrated in several cases.

The importance of decentralized development of something as critical as payment methods has been implemented in this way of transferring SOL natively with a programming language as secure as Rust. The input of the private key is implemented to use the file system of the operating system in question.

To sign, the user's private key is required, which must be requested in JSON format. After obtaining the signature, the Solana system program is called to make a modification of the accounts in question, specifically their balance, and then the gas fee for the respective transaction will be paid.

A message for the recipient is requested as input (which will later be printed via msg!). A variable that manages the sending of SOL could be dynamically implemented as an argument.

This program assumes that the developer who wants to implement this in their business or personal project has an HTTPS protocol with p2p encryption or runs the program locally on a computer without distribution to servers.

</div>

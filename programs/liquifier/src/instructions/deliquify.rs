use crate::state::*;
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::{AssociatedToken},
        token::{self, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
pub struct Deliquify<'info>{
    #[account(
        mut,
        seeds = [b"liquifier".as_ref(), liquifier_token_mint.key().as_ref(), liquifier_collection_mint.key().as_ref()],
        bump = liquifier.bump,
        has_one = liquifier_treasury_token_account,
        has_one = liquifier_token_mint,
        has_one = liquifier_collection_mint,
    )]
    // liquifier account with seeds as "liquifier", spl token mint address of $OCUL, collection mint address of 'SolOccult' 
    liquifier: Account<'info, Liquifier>,
    #[account(mut)]
    // $OCUL treasury token account owned by 'liquifier', with seeds as "liquifier_treasury_token", 'liquifier' address

    liquifier_treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    // $OCUL associated token account of 'user'
    user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    // user wallet for signing
    user: Signer<'info>,
    // mint address of $OCUL
    liquifier_token_mint: Account<'info, Mint>,
    // collection mint address of 'SolOccult'
    liquifier_collection_mint: Account<'info, Mint>,
    // mint address of the NFT to be deliquified
    mint_to_be_deliquified: Account<'info, Mint>,
    #[account(mut)]
    // associated token address of NFT to be deliquified associated with user account a.k.a wallet
    mint_to_be_deliquified_user_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed, 
        seeds = [b"mint_to_be_liquified_liquifier_account".as_ref(),liquifier.key().as_ref(), mint_to_be_deliquified.key().as_ref()],
        bump,
        payer = user,
        token::authority = liquifier,
        token::mint = mint_to_be_deliquified,
    )]
    // associated token address of NFT to be deliquified associated with liquifier account 
    mint_to_be_deliquified_liquifier_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    // metadata account of NFT to be deliquified
    mint_to_be_deliquified_metadata: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: AccountInfo<'info>,
    associated_token: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Deliquify>) -> Result<()>{
    // transfer NFT from 'liquifier' to user NFT associated token account 
    let cpi_accounts_nft_liq_user = Transfer {
        from: ctx.accounts.mint_to_be_deliquified_liquifier_account.to_account_info(),
        to: ctx.accounts.mint_to_be_deliquified_user_account.to_account_info(),
        authority: ctx.accounts.liquifier.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context_nft_liq_user = CpiContext::new(cpi_program, cpi_accounts_nft_liq_user);
    token::transfer(cpi_context_nft_liq_user, 1)?;

    // TODO: 1. check if user is sending tokens that belong to liquifier token mint address
    // TODO: 2. check if user is sending liquification_threshold number of tokens
    // TODO:    a. if yes -> transfer tokens from user token account to treasury token account
    // TODO:    b. if no -> throw an error to send exactly liquification_threshold number of tokens
    // TODO: 3. check if liquifier has 'exactly' one token in its mint_to_be_deliquified(a.k.a NFT) associated token account
    // TODO:    a. if yes -> transfer the NFT from liquifier to user NFT associated token account
    // TODO:    b. if no -> throw an error

    // --updates by chicha
    // TODO: 1. check if user is sending tokens that belong to liquifier token mint address
    // Assume that `token_account` is a reference to the account representing the tokens, and
    // `mint_address` is the address of the mint that you want to check membership for.
    //let metadata = Metadata::load(token_account).unwrap();
    // `liquification_threshold` is the number of tokens that you want to check for.
    //let balance = TokenAccount::get_balance(token_account).unwrap();
    //if metadata.mint == mint_address & balance == liquification_threshold {
        // transfer tokens from user token account to treasury token account
        //let cpi_accounts_token_user_liq = Transfer {
            //from: ctx.accounts.user_token_account.to_account_info(),
            //to: ctx.accounts.liquifier_treasury_token_account.to_account_info(),
            //authority: ctx.accounts.user.to_account_info(),
        //};
        //let cpi_program = ctx.accounts.token_program.to_account_info();
        //let cpi_context_token_user_liq = CpiContext::new(cpi_program, cpi_accounts_token_user_liq);
        //token::transfer(cpi_context_token_user_liq, 1_000_000)?;;
    //} else {
    //    println!("These tokens do not belong to the mint.");
    //}



    let liquifier = &mut ctx.accounts.liquifier;

    // transfer tokens from user token account to treasury token account
    let cpi_accounts_token_user_liq = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.liquifier_treasury_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context_token_user_liq = CpiContext::new(cpi_program, cpi_accounts_token_user_liq);
    token::transfer(cpi_context_token_user_liq, liquifier.liquification_threshold)?;
    Ok(())
}

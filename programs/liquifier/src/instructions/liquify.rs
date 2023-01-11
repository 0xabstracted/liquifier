use crate::state::*;
use crate::error::*;
use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{self, Mint, Token, TokenAccount, Transfer},
    },
    mpl_token_metadata::{
        state::Metadata,
        utils::assert_derivation
    },
};

#[derive(Accounts)]
pub struct Liquify<'info>{
    #[account(
        mut,
        seeds = [b"liquifier".as_ref(), liquifier_token_mint.key().as_ref(), liquifier_collection_mint.key().as_ref()],
        bump = liquifier.bump,
        has_one = liquifier_treasury_token_account,
        has_one = liquifier_token_mint,
        has_one = liquifier_collection_mint,
    )]
    // liquifier account with seeds as "liquifier", spl token mint address of $LIQUI, collection mint address of 'SolOccult' 
    liquifier: Account<'info, Liquifier>,
    // #[account(
    //     mut,
    //     seeds = [b"liquifier_treasury_token".as_ref(), liquifier.key().as_ref()],
    //     bump = liquifier.treasury_bump,
    // )]
    // $OCUL treasury token account owned by 'liquifier', with seeds as "liquifier_treasury_token", 'liquifier' address
    #[account(
        mut,
        constraint = liquifier_treasury_token_account.mint == liquifier_token_mint.key() @ LiquifierError::InvalidLiquifierTokenMint,
        constraint = liquifier_treasury_token_account.owner == liquifier.key() @ LiquifierError::InvalidLiquifierTreasuryTokenAccountOwner,
    )]
    liquifier_treasury_token_account: Account<'info, TokenAccount>,
    // $LIQUI associated token account of 'user'
    #[account(
        mut,
        constraint = user_token_account.mint == liquifier_token_mint.key() @ LiquifierError::InvalidLiquifierTokenMint,
        constraint = user_token_account.owner == user.key() @ LiquifierError::InvalidUserLiquifierTokenAccountOwner,
    )]
    user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    // 'user' wallet for signing
    user: Signer<'info>,
    // mint address of $LIQUI
    liquifier_token_mint: Account<'info, Mint>,
    // collection mint address of 'SolOccult'
    liquifier_collection_mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    liquifier_token_mint_metadata: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    liquifier_collection_mint_metadata: AccountInfo<'info>,
    // mint address of the NFT to be liquified
    mint_to_be_liquified: Account<'info, Mint>,
    // associated token address of NFT to be liquified associated with user account a.k.a wallet
    #[account(
        mut,
        constraint = mint_to_be_liquified_user_account.mint == mint_to_be_liquified.key() @ LiquifierError::InvalidMintToBeLiquified, 
        constraint = mint_to_be_liquified_user_account.owner == user.key() @ LiquifierError::InvalidUserLiquifierTokenAccountOwner,
    )]
    mint_to_be_liquified_user_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed, 
        seeds = [b"mint_to_be_liquified_liquifier_account".as_ref(),liquifier.key().as_ref(), mint_to_be_liquified.key().as_ref()],
        bump,
        payer = user,
        token::mint = mint_to_be_liquified,
        token::authority = liquifier,
    )]
    // associated token address of NFT to be liquified associated with liquifier account 
    mint_to_be_liquified_liquifier_account: Account<'info, TokenAccount>,
    // metadata account of NFT to be liquified
    /// CHECK: This is not dangerous because we don't read or write from this account
    mint_to_be_liquified_metadata: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: AccountInfo<'info>,
    associated_token: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

    
pub fn handler(ctx: Context<Liquify>) -> Result<()>{
    // mutable reference of 'liquifier' account
    let liquifier = &ctx.accounts.liquifier;


    // assert metadata account derivation
    assert_derivation(
        &mpl_token_metadata::id(),
        &ctx.accounts.mint_to_be_liquified_metadata.to_account_info(),
        &[
            mpl_token_metadata::state::PREFIX.as_bytes(),
            mpl_token_metadata::id().as_ref(),
            ctx.accounts.mint_to_be_liquified.key().as_ref(),
        ],
    )?;

    // assert metadata account derivation
    assert_derivation(
        &mpl_token_metadata::id(),
        &ctx.accounts.liquifier_token_mint_metadata.to_account_info(),
        &[
            mpl_token_metadata::state::PREFIX.as_bytes(),
            mpl_token_metadata::id().as_ref(),
            ctx.accounts.liquifier_token_mint.key().as_ref(),
        ],
    )?;

    // assert metadata account derivation
    assert_derivation(
        &mpl_token_metadata::id(),
        &ctx.accounts.liquifier_collection_mint_metadata.to_account_info(),
        &[
            mpl_token_metadata::state::PREFIX.as_bytes(),
            mpl_token_metadata::id().as_ref(),
            ctx.accounts.liquifier_collection_mint.key().as_ref(),
        ],
    )?;

    if  ctx.accounts.mint_to_be_liquified_metadata.data_is_empty() || 
        ctx.accounts.liquifier_token_mint_metadata.data_is_empty() ||
        ctx.accounts.liquifier_collection_mint_metadata.data_is_empty()
    {
        return Err(error!(LiquifierError::EmptyMetadataAccount));
    }

    if ctx.accounts.mint_to_be_liquified_metadata.to_account_info().owner.key() != mpl_token_metadata::id() || 
        ctx.accounts.liquifier_token_mint_metadata.to_account_info().owner.key() != mpl_token_metadata::id() ||
        ctx.accounts.liquifier_collection_mint_metadata.to_account_info().owner.key() != mpl_token_metadata::id()
    {
        return Err(error!(LiquifierError::InvalidMintMetadataOwner));
    }

    let mint_to_be_liquified_metadata_data = ctx.accounts.mint_to_be_liquified_metadata.try_borrow_mut_data().expect("Failed to borrow data");
    let mint_to_be_liquified_metadata = Metadata::deserialize(&mut mint_to_be_liquified_metadata_data.as_ref()).expect("Failed to deserialize metadata");
    if mint_to_be_liquified_metadata.mint != ctx.accounts.mint_to_be_liquified.key() {
        return Err(error!(LiquifierError::InvalidMintToBeLiquifiedMetadata));
    }

    if !mint_to_be_liquified_metadata.collection.is_some() {
        return Err(error!(LiquifierError::NoCollectionDefined));
    }
    let collection = mint_to_be_liquified_metadata.collection.unwrap();
    if !collection.verified{
        return Err(error!(LiquifierError::UnverifiedCollection));
    }
    if liquifier.liquifier_collection_mint != collection.key{
        return Err(error!(LiquifierError::InvalidCollectionMint));
    }

    // transfer NFT from user to 'liquifier' NFT associated token account
    let cpi_accounts_nft_user_liq = token::Transfer {
        from: ctx.accounts.mint_to_be_liquified_user_account.to_account_info(),
        to: ctx.accounts.mint_to_be_liquified_liquifier_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context_nft_user_liq = CpiContext::new(cpi_program, cpi_accounts_nft_user_liq);
    token::transfer(cpi_context_nft_user_liq, 1)?;
    // TODO: if three conditions are satisfied transfer the nft
    // TODO: 1. check if nft belongs to the collection
    // TODO: 2. check max supply of the NFT mint account to be 1
    // TODO: 3. check if the supply of mint_to_be_associated token account has exactly 1
    // TODO: 4. check if treasury wallet has more than liquification_threshold no of tokens
    // TODO:    a. if yes -> transfer liquification_threshold tokens from treasury token account to user token account
    // TODO:    b. if no -> mint from the token and transfer to user token account

    // transfer tokens from treasury token account to user token account
    let cpi_accounts_token_liq_user = Transfer {
        from: ctx.accounts.liquifier_treasury_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.liquifier.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context_token_liq_user = CpiContext::new(cpi_program, cpi_accounts_token_liq_user);
    token::transfer(cpi_context_token_liq_user, liquifier.liquification_threshold)?;
    Ok(())
}
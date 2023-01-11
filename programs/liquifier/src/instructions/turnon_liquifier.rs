use crate::state::*;
use crate::error::*;
use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke_signed,
    },
    anchor_spl::{
        token::{self, Mint, Token, TokenAccount},
    },
    mpl_token_metadata::{
        utils::assert_derivation,
        instruction::create_metadata_accounts_v3,
    },
};


#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TurnonLiquifierArgs {
    pub name: String,       // name of the liquifier token $OCUL
    pub symbol: String,     // symbol of the liquifier token $OCUL
    pub uri: String,        // URI pointing to json metadata of the liquifier token $OCUL
    pub collection_size: u64, // size of 'SolOccult' collection 
    pub liquification_threshold: u64, // number of $OCUL tokens per 'SolOccult' NFT  
}

#[derive(Accounts)]
#[instruction(ix: TurnonLiquifierArgs)]
pub struct TurnonLiquifier<'info> {
    // liquifier account with seeds as "liquifier", spl token mint address of $OCUL, collection mint address of 'SolOccult' 
    #[account(
        mut,
        seeds = [b"liquifier".as_ref(), liquifier_token_mint.key().as_ref(), liquifier_collection_mint.key().as_ref()],
        bump = liquifier.bump,
        has_one = liquifier_treasury_token_account,
        has_one = liquifier_token_mint,
        has_one = liquifier_collection_mint,
        constraint = liquifier.collection_size == ix.collection_size,
        constraint = liquifier.liquification_threshold == ix.liquification_threshold,
    )]
    liquifier: Box<Account<'info, Liquifier>>,
    
    // $OCUL treasury token account owned by 'liquifier', with seeds as "liquifier_treasury_token", 'liquifier' address
    #[account(
        mut,
        constraint = liquifier_treasury_token_account.mint == liquifier_token_mint.key() @ LiquifierError::InvalidLiquifierTokenMint,
        constraint = liquifier_treasury_token_account.owner == liquifier.key() @ LiquifierError::InvalidLiquifierTreasuryTokenAccountOwner,
    )]
    liquifier_treasury_token_account: Box<Account<'info, TokenAccount>>,
    // admin wallet for signing
    #[account(mut)]
    admin: Signer<'info>,
    // mint address of $OCUL
    #[account(constraint = assert_token_mint_address(&liquifier_token_mint.key()))]
    liquifier_token_mint: Box<Account<'info, Mint>>,
    // collection mint address of 'SolOccult'
    #[account(constraint = assert_collection_mint_address(&liquifier_collection_mint.key()))]
    liquifier_collection_mint: Box<Account<'info, Mint>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    liquifier_token_mint_metadata: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    liquifier_collection_mint_metadata: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<TurnonLiquifier>,
    ix: TurnonLiquifierArgs,
) -> Result<()> {
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

    // // assert metadata account derivation
    // assert_derivation(
    //     &mpl_token_metadata::id(),
    //     &ctx.accounts.liquifier_collection_mint_metadata.to_account_info(),
    //     &[
    //         mpl_token_metadata::state::PREFIX.as_bytes(),
    //         mpl_token_metadata::id().as_ref(),
    //         ctx.accounts.liquifier_collection_mint.key().as_ref(),
    //     ],
    // )?;

    // mutablle reference of 'liquifier' account
    let liquifier = &ctx.accounts.liquifier;

    let liquifier_seeds = &[b"liquifier".as_ref(), liquifier.liquifier_token_mint.as_ref(), liquifier.liquifier_collection_mint.as_ref(), &[liquifier.bump]];
    let liquifier_signer = &[&liquifier_seeds[..]];
    
    invoke_signed(
        &create_metadata_accounts_v3(
            *ctx.accounts.token_metadata_program.key,
            *ctx.accounts.liquifier_token_mint_metadata.key,
            ctx.accounts.liquifier_token_mint.key(),
            liquifier.key(),
            *ctx.accounts.admin.key,
            liquifier.key(),
            ix.name,
            ix.symbol,
            ix.uri,
            None,
            0,
            true,
            true,
            None,
            None,
            None,
        ),
        &[
            ctx.accounts.liquifier_token_mint_metadata.to_account_info(),
            ctx.accounts.liquifier_token_mint.to_account_info(),
            ctx.accounts.liquifier.to_account_info(),
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.liquifier.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        liquifier_signer,
    )?;

    // mint liquifier.max_tokens_liquified tokens to liquifier treasury token account
    let total_token_supply = u64::try_from(liquifier.max_tokens_liquified).unwrap();
    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.liquifier_token_mint.to_account_info(),
        to: ctx.accounts.liquifier_treasury_token_account.to_account_info(),
        authority: ctx.accounts.liquifier.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(liquifier_signer);
    token::mint_to(cpi_context, total_token_supply)?;

    let liquifier = &mut ctx.accounts.liquifier;

    // total $OCUL tokens sent to treasury wallet owned by 'liquifier'
    let liquifier_treasury_token_amount = u128::try_from(ix.collection_size).unwrap().checked_mul(u128::try_from(ix.liquification_threshold).unwrap());
    // store maximum $OCUL tokens that can be liqufied by this 'liquifier' account
    liquifier.max_tokens_liquified = liquifier_treasury_token_amount.unwrap();
    // store total $OCUL tokens sent to treasury wallet in 'liquifier' account
    liquifier.liquifier_treasury_token_amount = liquifier_treasury_token_amount.unwrap();
    // store liquified_nfts_count of 'SolOccult' with 'liquifier'
    liquifier.liquified_nfts_count = ix.collection_size;
    

    Ok(())
}

use mpl_token_metadata::state::Metadata;
use mpl_token_metadata::state::TokenStandard;
use spl_associated_token_account::get_associated_token_address;

use crate::state::*;
use crate::error::*;
use {
    anchor_lang::{
        prelude::*,
        solana_program::program::{invoke},
    },
    anchor_spl::{
        associated_token::{self, AssociatedToken},
        token::{self, Mint, Token, TokenAccount},
    },
    solana_program::{program_pack::Pack, system_instruction::create_account},
    mpl_token_metadata::utils::assert_derivation,
};


#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateLiquifierArgs {
    pub decimals: u8,       // decimals of the liquifier token $OCUL
    pub collection_size: u64, // size of 'SolOccult' collection 
    pub liquification_threshold: u64, // number of $OCUL tokens per 'SolOccult' NFT  
}

#[derive(Accounts)]
#[instruction(ix: CreateLiquifierArgs)]
pub struct CreateLiquifier<'info> {
    // liquifier account with seeds as "liquifier", spl token mint address of $OCUL, collection mint address of 'SolOccult' 
    #[account(
        init,
        seeds = [b"liquifier".as_ref(), liquifier_token_mint.key().as_ref(), liquifier_collection_mint.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Liquifier>(),
        payer = admin,
    )]
    liquifier: Box<Account<'info, Liquifier>>,
    // admin wallet for signing
    #[account(mut)]
    admin: Signer<'info>,
    // $OCUL treasury token account owned by 'liquifier', with seeds as "liquifier_treasury_token", 'liquifier' address
    #[account(mut)]
    liquifier_treasury_token_account: Box<Account<'info, TokenAccount>>,
    // mint address of $OCUL
    #[account(mut, constraint = assert_token_mint_address(&liquifier_token_mint.key()))]
    liquifier_token_mint: Box<Account<'info, Mint>>,
    // collection mint address of 'SolOccult'
    #[account(constraint = assert_collection_mint_address(&liquifier_collection_mint.key()))]
    liquifier_collection_mint: Box<Account<'info, Mint>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    liquifier_collection_mint_metadata: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = mpl_token_metadata::id())]
    token_metadata_program: AccountInfo<'info>,
    associated_token: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreateLiquifier>,
    ix: CreateLiquifierArgs,
) -> Result<()> {
    // mutable reference of 'liquifier' account

    // if liquification_threshold > 0 && collection_size > 0 {
    if ix.liquification_threshold != OCUL_PER_SOLOCCULT as u64 || 
       ix.collection_size != SOLOCCULT_COLLECTION_SIZE as u64 ||
       ix.decimals !=0
    {
        return Err(error!(LiquifierError::InvalidParamsForCollection));
    }

    let ata = get_associated_token_address(&ctx.accounts.liquifier.key(), &ctx.accounts.liquifier_token_mint.key());
    if ata != ctx.accounts.liquifier_treasury_token_account.key() {
        return Err(error!(LiquifierError::InvalidLiquifierTreasuryTokenAccountAddress));
    }




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



    if ctx.accounts.liquifier_collection_mint_metadata.data_is_empty()
    {
        return Err(error!(LiquifierError::EmptyMetadataAccount));
    }

    if ctx.accounts.liquifier_collection_mint_metadata.to_account_info().owner.key() != mpl_token_metadata::id()
    {
        return Err(error!(LiquifierError::InvalidMintMetadataOwner));
    }

    let liquifier_collection_mint_metadata_data = ctx.accounts.liquifier_collection_mint_metadata.try_borrow_mut_data().expect("Failed to borrow data");
    let liquifier_collection_mint_metadata =  Metadata::deserialize(&mut liquifier_collection_mint_metadata_data.as_ref()).expect("Failed to deserialize metadata");
    if  liquifier_collection_mint_metadata.mint != ctx.accounts.liquifier_collection_mint.key() ||
        liquifier_collection_mint_metadata.primary_sale_happened != false ||
        liquifier_collection_mint_metadata.token_standard != Some(TokenStandard::NonFungible) ||
        liquifier_collection_mint_metadata.data.seller_fee_basis_points != 0
    {
        return Err(error!(LiquifierError::InvalidCollectionMintMetadata));
    }


    if liquifier_collection_mint_metadata.data.creators.is_some() {
        let creators = liquifier_collection_mint_metadata.data.creators.unwrap();
        let find = creators.iter().find(|c| assert_collection_mint_verified_creator(&c.address) && c.verified);
        if !find.is_some() {
            return Err(error!(LiquifierError::InvalidCollectionMintMetadata));
        };
    }

    if liquifier_collection_mint_metadata.collection.is_some() {
        return Err(error!(LiquifierError::InvalidCollectionMintMetadata));
    }
    // let collection = liquifier_collection_mint_metadata.collection.unwrap();
    // if !collection.verified{
    //     return Err(error!(LiquifierError::UnverifiedCollection));
    // }
    // if liquifier.liquifier_collection_mint != collection.key{
    //     return Err(error!(LiquifierError::InvalidCollectionMint));
    // }

    let liquifier_token_mint = &ctx.accounts.liquifier_token_mint;


    // create mint
    invoke(
        &create_account(
            &ctx.accounts.admin.key(),
            &liquifier_token_mint.key(),
            ctx.accounts.rent.minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN as u64,
            &spl_token::id(),
        ),
        &[ctx.accounts.admin.to_account_info(), liquifier_token_mint.to_account_info()],
    )?;


    // Initialize mint
    let cpi_accounts = token::InitializeMint {
        mint: liquifier_token_mint.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::initialize_mint(cpi_context, ix.decimals, &ctx.accounts.liquifier.key(), Some(&ctx.accounts.liquifier.key()))?;

    

    // create associated token account for liquifier

    let cpi_accounts = associated_token::Create {
        payer: ctx.accounts.admin.to_account_info(),
        associated_token: ctx.accounts.liquifier_treasury_token_account.to_account_info(),
        authority: ctx.accounts.liquifier.to_account_info(),
        mint: liquifier_token_mint.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_program = ctx.accounts.associated_token.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    associated_token::create(cpi_context)?;

    
    let liquifier = &mut ctx.accounts.liquifier;

    // store bump of 'liquifier' account
    liquifier.bump = *ctx.bumps.get("liquifier").unwrap();
    // store 'liquifier' account address owned by 'liquifier' in 'liquifier' account
    liquifier.liquifier_id = liquifier.key();
    // store maximum $OCUL tokens that can be liquified by this 'liquifier' account
    liquifier.max_tokens_liquified = 0;
    // store total $OCUL tokens sent to treasury wallet in 'liquifier' account
    liquifier.liquifier_treasury_token_amount = 0;
    // store total collection size of 'SolOccult' 
    liquifier.collection_size = ix.collection_size;
    // store liquified_nfts_count of 'SolOccult' with 'liquifier'
    liquifier.liquified_nfts_count = 0;
    // store max $OCUL tokens liquified per NFT
    liquifier.liquification_threshold = ix.liquification_threshold;
    // store mint address of $OCUL in 'liquifier' account
    liquifier.liquifier_token_mint = liquifier_token_mint.key();
    // store $OCUL treasury token account in 'liquifier' account 
    liquifier.liquifier_treasury_token_account = ctx.accounts.liquifier_treasury_token_account.key();
    
    // store collection mint address of 'SolOccult' in 'liquifier' account
    liquifier.liquifier_collection_mint = ctx.accounts.liquifier_collection_mint.key();


    Ok(())
}

use anchor_lang::prelude::*;
use std::str::FromStr;

#[account]
#[derive(Default, Debug)]
pub struct Liquifier {
    // bump of 'liquifier' account
    pub bump: u8,
    // 'liquifier' account address 
    pub liquifier_id: Pubkey,
    // maximum $OCUL tokens that can be liquified by this 'liquifier' account
    pub max_tokens_liquified: u128,
    // total tokens sent to treasury wallet 
    pub liquifier_treasury_token_amount: u128,
    // total collection size 
    pub collection_size: u64,
    // liquified_nfts_count of "XX_collection_XX" with 'liquifier'
    pub liquified_nfts_count: u64,
    // max tokens liquified per NFT
    pub liquification_threshold: u64,
    //  mint address of token 
    pub liquifier_token_mint: Pubkey,
    // treasury token account address owned by 'liquifier' account 
    pub liquifier_treasury_token_account: Pubkey,
    // collection mint address 
    pub liquifier_collection_mint: Pubkey,
}

pub const OCUL_PER_SOLOCCULT: usize = 1_000_000_000;
pub const SOLOCCULT_COLLECTION_SIZE: usize = 10_000;

pub fn assert_token_mint_address(pubkey: &Pubkey) -> bool {
    pubkey.to_string() == Pubkey::from_str("oc1sWand58xiimhsdvH547u8arUe3gbgmCT3w9fcLxt").unwrap().to_string()
}

pub fn assert_collection_mint_address(pubkey: &Pubkey) -> bool {
    pubkey.to_string() == Pubkey::from_str("DF38t9y7iwsUtRAnz331HLxDGWd5onbrS8qoMPyUzJwS").unwrap().to_string()
}
pub fn assert_collection_mint_update_authority(pubkey: &Pubkey) -> bool {
    pubkey.to_string() == Pubkey::from_str("SALThRKa8JFD3XoGbLshzg61kaBHFFCeggDH8ydNEaE").unwrap().to_string()
}
pub fn assert_collection_mint_verified_creator(pubkey: &Pubkey) -> bool {
    pubkey.to_string() == Pubkey::from_str("SALThRKa8JFD3XoGbLshzg61kaBHFFCeggDH8ydNEaE").unwrap().to_string()
}

pub fn assert_mint_to_be_liquified_verified_creator(pubkey: &Pubkey) -> bool {
    pubkey.to_string() == Pubkey::from_str("SALThRKa8JFD3XoGbLshzg61kaBHFFCeggDH8ydNEaE").unwrap().to_string() //candymachine id
}
pub fn assert_mint_to_be_liquified_update_authority(pubkey: &Pubkey) -> bool {
    pubkey.to_string() == Pubkey::from_str("SALThRKa8JFD3XoGbLshzg61kaBHFFCeggDH8ydNEaE").unwrap().to_string()
}

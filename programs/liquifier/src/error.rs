use anchor_lang::prelude::*;

#[error_code]
pub enum LiquifierError {
    #[msg("Invalid parameters for Collection")]
    InvalidParamsForCollection,
    #[msg("Invalid Liquifier Treasury TokenAccount Owner")]
    InvalidLiquifierTreasuryTokenAccountOwner,
    #[msg("Invalid Liquifier Token Mint")]
    InvalidLiquifierTokenMint,
    #[msg("Invalid Mint To Be Liquified")]
    InvalidMintToBeLiquified,
    #[msg("Invalid User Liquifier TokenAccount Owner")]
    InvalidUserLiquifierTokenAccountOwner,
    #[msg("Empty Metadta Account")]
    EmptyMetadataAccount,
    #[msg("Invalid Mint Metadata Owner")]
    InvalidMintMetadataOwner,
    #[msg("Invalid Collection Mint Metadata")]
    InvalidCollectionMintMetadata,
    #[msg("No Collection Defined in Metadata")]
    NoCollectionDefined,
    #[msg("Unverified Collection in Metadata")]
    UnverifiedCollection,
    #[msg("Invalid Collection Mint in Metadata")]
    InvalidCollectionMint,
    #[msg("Invalid Liquifier Treasury TokenAccount Address")]
    InvalidLiquifierTreasuryTokenAccountAddress
}

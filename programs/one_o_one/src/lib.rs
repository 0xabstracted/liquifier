pub use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

pub use instructions::*;
pub use state::*;

declare_id!("GaTmPSkohwxmaK5THGQv3vms5Ernm4hYBaBDz8VCfmUp");

#[program]
pub mod one_o_one {

    use super::*;

    pub fn create_registry(
        ctx: Context<CreateRegistry>, 
        rate_token_in: u64, 
        rate_token_out: u64,
    ) -> Result<()> {
        instructions::create_registry::create_registry(ctx, rate_token_in, rate_token_out)
    }
    
    pub fn update_registry(
        ctx: Context<UpdateRegistry>, 
        rate_token_in: u64, 
        rate_token_out: u64,
    ) -> Result<()> {
        instructions::update_registry::update_registry(ctx, rate_token_in, rate_token_out)
    }

    pub fn swap(
        ctx: Context<Swap>,
        amount_requested: u64,
    ) -> Result<()> {
        instructions::swap::swap(ctx, amount_requested)
    }

    pub fn collect_proceeds(
        ctx: Context<CollectProceeds>,
    ) -> Result<()> {
        instructions::collect_proceeds::collect_proceeds(ctx)
    }

    pub fn collect_reserve(
        ctx: Context<CollectReserve>,
    ) -> Result<()> {
        instructions::collect_reserve::collect_reserve(ctx)
    }
}

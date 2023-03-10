pub use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

pub use instructions::*;
pub use state::*;

declare_id!("4gHkd41joA4RyNuFY9UYYUbsBwAgjTomRoZFJjLwd5ht");

#[program]
pub mod liquifier {

    use super::*;

    pub fn create_liquifier(
        ctx: Context<CreateLiquifier>, 
        ix: CreateLiquifierArgs,
    ) -> Result<()> {
        create_liquifier::handler(ctx, ix)
    }
    
    pub fn turnon_liquifier(
        ctx: Context<TurnonLiquifier>, 
        ix: TurnonLiquifierArgs,
    ) -> Result<()> {
        turnon_liquifier::handler(ctx, ix)
    }

    pub fn liquify(
        ctx: Context<Liquify>,
    ) -> Result<()> {
        liquify::handler(ctx)
    }

    pub fn deliquify(
        ctx: Context<Deliquify>,
    ) -> Result<()> {
        deliquify::handler(ctx)
    }
}

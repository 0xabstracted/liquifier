pub use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

pub use instructions::*;
pub use state::*;

declare_id!("LQU1Teo79ekWeqz6W8hcbjRRzhSrbarqhfNsgWEDe1g");

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

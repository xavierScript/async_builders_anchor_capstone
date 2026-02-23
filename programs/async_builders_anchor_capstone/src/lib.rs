use anchor_lang::prelude::*;

declare_id!("DE4Q3C4boDBeMobfbaSxETX9EmW4984HbZP9rKfDbMPQ");

#[program]
pub mod async_builders_anchor_capstone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

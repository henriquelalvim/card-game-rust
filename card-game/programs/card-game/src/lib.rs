use anchor_lang::prelude::*;

declare_id!("5zznXFxwcPVzb67wkVcRCaGF71jpYAQMQS8EXo6B8u9A");

#[program]
pub mod card_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

use anchor_lang::prelude::*;

declare_id!("5a9BvaEUnH8Kh7WGmTDsxTx82Ra7Nz8uPN3FqhkvDFsh");

#[program]
pub mod mycalculatordapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

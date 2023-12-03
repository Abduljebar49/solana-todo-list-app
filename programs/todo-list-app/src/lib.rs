use anchor_lang::prelude::*;

declare_id!("2JjnKZeCbrtDiNTahgN2zTWLzShBPSDz8tbqcU6zNSdb");

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

use anchor_lang::prelude::*;

declare_id!("6SJbPWDyQkrXMRoiVMnhF3g6XrNBuX5DEpvph95ur181");
pub const ANCHOR_DISCRIMATOR_SIZE: usize = 8;
#[program]
pub mod favourite {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    pub fn set_favourites(
        context: Context<SetFavourites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", context.program_id);
        let user_public_key = context.accounts.user.key();
        msg!("User {user_public_key}'s favourite number is {number}, favourite color is {color}, and their favourite hobbies are {hobbies:?}");

        context.accounts.favourites.set_inner(Favourites {
            number,
            color,
            hobbies,
        });
        Ok(())
    }
}
#[account]
#[derive(InitSpace)]
// #[derive(Accounts)]
pub struct Favourites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account
    (init_if_needed,
     payer = user,
      space = ANCHOR_DISCRIMATOR_SIZE + Favourites::INIT_SPACE,
    seeds = [b"favourites",user.key().as_ref()], bump)]
    pub favourites: Account<'info, Favourites>,
    pub system_program: Program<'info, System>,
}

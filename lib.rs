use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::clock::Clock;

declare_id!("6VsCQKutDjqhdKo7j3t7Qt1zKWVoxQUPRBTc7Ucaph7S");

const HAPPINESS_INCREASE: u32 = 10;
const HAPPINESS_DECREASE: u32 = 5;
const HAPPINESS_DECREASE_PERIOD: i64 = 24 * 60 * 60; // 24 hours in seconds
const MAX_NAME_LENGTH: usize = 20;

#[program]
pub mod solanapdas {
    use super::*;

    pub fn create_pet(ctx: Context<CreatePet>, name: String) -> ProgramResult {
        let pet = &mut ctx.accounts.pet;
        if name.len() > MAX_NAME_LENGTH {
            return Err(ProgramError::InvalidArgument);
        } 
        pet.name = name;
        pet.happiness = 50;
        pet.balance = 0;
        pet.owner = *ctx.accounts.user.key;
        pet.last_fed_timestamp = Clock::get()?.unix_timestamp;
        pet.last_happiness_decrease_timestamp = 0;
        pet.born_date = Clock::get()?.unix_timestamp;
       
        Ok(())
    }

    pub fn feed_pet(ctx: Context<FeedPet>, amount: u64) -> ProgramResult {
        let txn = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.pet.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &txn,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.pet.to_account_info()
            ],
         )?;
        (&mut ctx.accounts.pet).balance += amount;
        (&mut ctx.accounts.pet).happiness += HAPPINESS_INCREASE;
        (&mut ctx.accounts.pet).last_fed_timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }
    pub fn check_for_unfed(ctx: Context<CheckForUnfed>) -> ProgramResult {
        for pet in &mut ctx.accounts.pets.iter_mut() {
            let time_since_last_fed = ctx.accounts.clock.unix_timestamp - pet.last_fed_timestamp;
            let time_since_last_happiness_decrease = ctx.accounts.clock.unix_timestamp - pet.last_happiness_decrease_timestamp;
    
            if time_since_last_fed >= HAPPINESS_DECREASE_PERIOD && time_since_last_happiness_decrease >= HAPPINESS_DECREASE_PERIOD {
                pet.happiness -= HAPPINESS_DECREASE;
                pet.last_happiness_decrease_timestamp = ctx.accounts.clock.unix_timestamp;
            }
        }
        Ok(())
    }
    }



#[derive(Accounts)]
pub struct CreatePet<'info> {
    #[account(init, payer=user, space=500, seeds=[b"petaccount", user.key().as_ref()], bump)]
    pub pet: Account<'info, Pet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Pet {
    pub name: String,
    pub happiness: u32,
    pub balance: u64,
    pub owner: Pubkey,
    pub last_fed_timestamp: i64,
    pub last_happiness_decrease_timestamp: i64,
    pub born_date: i64,
}

#[derive(Accounts)]
pub struct FeedPet<'info>{
    #[account(mut)]
    pub pet: Account<'info, Pet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    
}
/*Chec 1 account *\
/* #[derive(Accounts)]
pub struct CheckForUnfed<'info>{
    #[account(mut)]
    pub pet: Account<'info, Pet>,
    pub clock: Sysvar<'info, Clock>,
} */

#[derive(Accounts)]
pub struct CheckForUnfed<'info> {
    #[account(mut)]
    pub pets: Vec<Account<'info, Pet>>,
    pub clock: Sysvar<'info, Clock>,

}

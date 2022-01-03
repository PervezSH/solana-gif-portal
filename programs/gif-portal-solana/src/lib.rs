// Its like import statement
// Importing tools Anchor provides us to make writing Solana programs easier
use anchor_lang::prelude::*;

// This is the Program Id
// has info for Solana on how to run our program
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// This is how we tell our program, everything in this little module below is our program
// this lets us actually call our Solana program from our frontend via a fetch request
// This are basically called macros, they attach code to our module. It's sorta like "inheriting" a class.
#[program]

// mod is a rust module, kinda like a class
pub mod gif_portal_solana {
    use super::*;
    // A function start_stuff_off which takes something called a Context and outputs a ProgramResult
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        Ok(())
    }
}

// Another macro
#[derive(Accounts)]
pub struct StartStuffOff {}

// Its like import statement
// Importing tools Anchor provides us to make writing Solana programs easier
use anchor_lang::prelude::*;

// This is the Program Id
// has info for Solana on how to run our program
declare_id!("x5znzH6GXVH77X3kGTaRor2DZXnHaVNxShXVF1wmaBN");

// This is how we tell our program, everything in this little module below is our program
// this lets us actually call our Solana program from our frontend via a fetch request
// This are basically called macros, they attach code to our module. It's sorta like "inheriting" a class.
#[program]

// mod is a rust module, kinda like a class
pub mod gif_portal_solana {
    use super::*;
    // A function start_stuff_off which takes something called a Context and outputs a ProgramResult
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // Get a reference to the account.
        //// We do &mut to get a "mutable reference" to base_account, gives us the power to make changes to base_account
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }

    // The function now accepts a gif_link param from the user
    pub fn add_gif(ctx: Context<BaseUser>, gif_link: String) -> ProgramResult {
        // Get refernce to the account, also user
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            gif_votes: 0
        };

        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    // Give user the ability to upvote a their fav gif
    pub fn upvote_gif(ctx: Context<BaseUser>, gif_index: String) -> ProgramResult {
        // Get refernce to the account, also user
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        
        // Upvote if user hasn't voted
        if !base_account.voters.contains(&*user.to_account_info().key) {
            let index: usize = gif_index.parse().unwrap();
            let gif_item = &mut base_account.gif_list[index];
            gif_item.gif_votes += 1;

            // Add user to voters vec
            base_account.voters.push(*user.to_account_info().key);
        }
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    // Tells solana how we want to initialize BaseAccount
    /*
        init, tells Solana to create a new account owned by our current program
        payer = user, tells our program who's paying for the account to be created
        space = 9000 which will allocate 9000 bytes of space for our account
    */
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    // proves to the program that the user calling this program actually owns their wallet account
    pub user: Signer<'info>,
    // It's basically a reference to the SystemProgram
    /*
        SystemProgram is the program that basically runs Solana
        It is responsible for a lot of stuff, but one of the main things it does is create accounts on Solana
    */
    pub system_program: Program <'info, System>,
}

// Specify base_account and user in BaseUser Conetext
#[derive(Accounts)]
pub struct BaseUser<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)] //Tells anchor how to serialize/deserialize the struct
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub gif_votes: u16
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    // BaseAccount holds one thing and it's an integer named total_gifs
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account
    pub gif_list: Vec<ItemStruct>,
    pub voters: Vec<Pubkey>
}

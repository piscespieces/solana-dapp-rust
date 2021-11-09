use anchor_lang::prelude::*;

declare_id!("4LTyrfW9kB6KqHv7DYBxtBJnrEKsHBK31GeG3CBJm3aX");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(_ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut _ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(_ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    // Get a reference to the account and increment total_gifs
    let base_account = &mut _ctx.accounts.base_account;

    let item = ItemStruct {
        gif_link: gif_link.to_string(),
        user_address: *base_account.to_account_info().key,
        gif_votes: 0,
    };

    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn update_item(_ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut _ctx.accounts.base_account;

    let mut item =
        base_account
        .gif_list
        .iter_mut()
        .find(|item| item.gif_link == gif_link)
        .ok_or(Err::NoGifLinkFound)?;

    item.gif_votes += 1;

    Ok(())
  }
}

#[error]
pub enum Err {
    #[msg("No gif_link found.")]
    NoGifLinkFound,
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things?
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub gif_votes: u64,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>
}
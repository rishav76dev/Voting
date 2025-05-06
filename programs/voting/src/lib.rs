use anchor_lang::prelude::*;

declare_id!("DXBjq1Fydn1DU1GE4biWnM2UWa1NDSKGmBZ4co8rEZpi");

#[program]
pub mod voting {
    use super::*;
    //step 1
    pub fn initialize(_ctx: Context<InitializePoll>, _poll_id: u64 ) -> Result<()> {
        
        Ok(())
    }
}

//Step 2
#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    //Step 4
    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), &poll_id.to_le_bytes().as_ref()],
        bump,
    )]

    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}


//step 3
#[account]
#[derive(InitSpace)]
pub struct PollAccount{
   
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}
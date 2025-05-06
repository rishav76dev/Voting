use anchor_lang::prelude::*;

declare_id!("DXBjq1Fydn1DU1GE4biWnM2UWa1NDSKGmBZ4co8rEZpi");

#[program]
pub mod voting {
    

    use super::*;
    //step 1
    pub fn initialize_poll(ctx: Context<InitializePoll>, 
        //Step 5
        poll_name: String,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        //Step 6 then we move to testing this code   I am skiping it for now as it was made next
        //which had idl
        let poll = &mut ctx.accounts.poll_account;
        poll.poll_name = poll_name;
        poll.poll_description = description;
        poll.poll_voting_start = poll_start;
        poll.poll_voting_end = poll_end;
        poll.candidate_amount = 0;

        Ok(())
    }

    pub fn intaialize_candidate(ctx: Context<InitializeCandidate>, 
        candidate_name: String,
        _poll_id: u64,
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_account;
        candidate.candidate_name = candidate_name;
        candidate.candidate_votes = 0;
    
        // ✅ Increment candidate count in the poll
        let poll = &mut ctx.accounts.poll_account;
        poll.candidate_amount += 1;
    
        Ok(())
    }
    
    //step8
    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()> {
        let candidate_account = &mut ctx.accounts.candidate_account;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time > (ctx.accounts.poll_account.poll_voting_end as i64) {
            return Err(ErrorCode::VotingEnded.into());
        }

        if current_time <= (ctx.accounts.poll_account.poll_voting_start as i64) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        candidate_account.candidate_votes += 1;

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


//step7

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
}

//step 10

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump)]
    pub candidate_account: Account<'info, CandidateAccount>,
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
    pub candidate_amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

//step 9

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
}
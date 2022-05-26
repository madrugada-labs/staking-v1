use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const JOB_APPLICATION_SEED: &'static [u8] = b"job_application";

// pub fn reset_data()

#[program]
pub mod application_factory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, job_ad_id: String) -> Result<()> {

        let details = &mut ctx.accounts.base_account;



        details.status = JobStatus::pending;
        details.stake_amount = 0;
        details.job_ad_id = job_ad_id;
        details.authority = ctx.accounts.authority.key();

        //TODO: initialize a new token mint

        Ok(())
    }

    pub fn update_status(ctx: Context<UpdateStatus>,bump: u8,  job_status: bool) -> Result<()> {

        let details = &mut ctx.accounts.base_account;

        details.status = job_status;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(job_ad_id: String)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, seeds = [JOB_APPLICATION_SEED, job_ad_id.as_bytes()[..18].as_ref(), job_ad_id.as_bytes()[18..].as_ref(), authority.key().as_ref()], bump, space = 1 + 4 + 32 + 40)]
    pub base_account: Account<'info, JobApplication>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Account)]
#[instruction(bump: u8)]
pub struct UpdateStatus<'info> {
    #[account(mut, seeds = [JOB_APPLICATION_SEED, job_ad_id.as_bytes()[..18].as_ref(), job_ad_id.as_bytes()[18..].as_ref(), authority.key().as_ref()], bump)]
    pub base_account: Account<'info, JobApplication>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum JobStatus {
    selected,
    rejected,
    pending
}

#[account]
pub struct JobApplication {
    pub status: JobStatus, // 1
    pub stake_amount: u32, // 4
    pub job_ad_id: String, // 40
    pub authority: Pubkey // 32
}


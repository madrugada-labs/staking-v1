use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const JOB_APPLICATION_SEED: &'static [u8] = b"job_application";

#[program]
pub mod application_factory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, job_ad_id: String) -> Result<()> {

        let details = &mut ctx.accounts.base_account;

        details.status = false,
        details.stake_amount = 0;
        details.job_ad_id = job_ad_id;
        details.authority = ctx.accounts.authority.key();

        //TODO: initialize a new token mint

        Ok(())
    }

    pub fn update_status(ctx: Context<UpdateStatus>, job_status: bool) -> Result<()> {

        let details = &mut ctx.accounts.base_account;

        details.status = job_status;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(job_ad_id: String)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, seeds = [JOB_APPLICATION_SEED, job_ad_id.as_bytes()[..18].as_ref(), job_ad_id.as_bytes()[18..].as_ref(), authority.key().as_ref()], bump, space = )]
    pub base_account: Account<'info, JobApplication>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct JobApplication {
    pub status: bool,
    pub stake_amount: u32,
    pub job_ad_id: String,
    pub authority: Pubkey
}
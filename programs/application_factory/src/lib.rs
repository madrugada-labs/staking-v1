use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token::{InitializeMint, Token, Mint};

declare_id!("2Q3jVyyE5nfU3nCZKTuemtHFdxXBcq6QtjWDR387TeJQ");

const JOB_APPLICATION_SEED: &'static [u8] = b"job_application";

// pub fn reset_data()

#[program]
pub mod application_factory {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        job_ad_id: String,
        mint_bump: u8,
        bump: u8,
    ) -> Result<()> {
        let details = &mut ctx.accounts.base_account;

        let job_ad_id_bytes_1 = &job_ad_id.as_bytes()[..18];
        let job_ad_id_bytes_2 = &job_ad_id.as_bytes()[18..];

        details.reset(job_ad_id.clone(), ctx.accounts.authority.key());

        // let new_token_mint = Keypair::new();
        let decimals = 9;

        //initialize a new token mint

        let bump_vector = mint_bump.to_le_bytes();

        // let inner = vec![
        //     JOB_APPLICATION_SEED,
        //     job_ad_id_bytes_1,
        //     job_ad_id_bytes_2,
        //     ctx.accounts.authority.key.as_ref(),
        //     bump_vector.as_ref(),
        // ];
        // let outer = vec![inner.as_slice()];

        // let mint_span: u64 = 82;
        // let lamports = Rent::get()?.minimum_balance(usize::try_from(mint_span).unwrap());
        // solana_program::program::invoke_signed_unchecked(
        //     &solana_program::system_instruction::create_account(
        //         &ctx.accounts.authority.key(),
        //         &ctx.accounts.mint_account.key(),
        //         lamports,
        //         mint_span,
        //         &ctx.accounts.token_program.key(),
        //     ),
        //     &[
        //         ctx.accounts.authority.to_account_info(),
        //         ctx.accounts.mint_account.to_account_info(),
        //         ctx.accounts.system_program.to_account_info(),
        //     ],
        //     outer.as_slice(),
        // )?;

        // Below is the actual instruction that we are going to send to the Token program.
        let transfer_instruction = InitializeMint {
            mint: ctx.accounts.mint_account.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            // outer.as_slice(), //signer PDA
        );

        // The `?` at the end will cause the function to return early in case of an error.
        // This pattern is common in Rust.
        anchor_spl::token::initialize_mint(
            cpi_ctx,
            decimals,
            &ctx.accounts.base_account.key(),
            None,
        )?;

        Ok(())
    }

    // pub fn update_status(ctx: Context<UpdateStatus>,bump: u8,  job_status: bool) -> Result<()> {

    //     let details = &mut ctx.accounts.base_account;

    //     details.status = job_status;

    //     Ok(())
    // }
}

#[derive(Accounts)]
#[instruction(job_ad_id: String, mint_bump: u8)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, seeds = [JOB_APPLICATION_SEED, job_ad_id.as_bytes()[..18].as_ref(), job_ad_id.as_bytes()[18..].as_ref(), authority.key().as_ref()], bump, space = 1 + 4 + 32 + 40 + 8)]
    pub base_account: Account<'info, JobApplication>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: ...
    #[account(init, 
        seeds = [JOB_APPLICATION_SEED, job_ad_id.as_bytes()[..18].as_ref(), job_ad_id.as_bytes()[18..].as_ref()], bump, payer = authority,
        space = Mint::LEN,
    )]
    pub mint_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

// #[derive(Account)]
// #[instruction(bump: u8)]
// pub struct UpdateStatus<'info> {
//     #[account(mut, seeds = [JOB_APPLICATION_SEED, job_ad_id.as_bytes()[..18].as_ref(), job_ad_id.as_bytes()[18..].as_ref(), authority.key().as_ref()], bump = bump)]
//     pub base_account: Account<'info, JobApplication>,
// }

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum JobStatus {
    Selected,
    Rejected,
    Pending,
}

trait SeedFormat {
    fn to_seed_format(self) -> String;
}

#[account]
pub struct JobApplication {
    pub status: JobStatus, // 1
    pub stake_amount: u32, // 4
    pub job_ad_id: String, // 40
    pub authority: Pubkey, // 32
}

impl<'info> JobApplication {
    pub fn reset(&mut self, job_ad_id: String, authority: Pubkey) {
        self.job_ad_id = job_ad_id;
        self.stake_amount = 0;
        self.status = JobStatus::Pending;
        self.authority = authority;
    }
}

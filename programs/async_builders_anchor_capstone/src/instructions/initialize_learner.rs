use anchor_lang::prelude::*;

use crate::state::LearnerProfile;

#[derive(Accounts)]
pub struct InitializeLearner<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + LearnerProfile::INIT_SPACE,
        seeds = [b"learner", user.key().as_ref()],
        bump,
    )]
    pub learner_profile: Account<'info, LearnerProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeLearner<'info> {
    pub fn initialize_learner(&mut self, bumps: &InitializeLearnerBumps) -> Result<()> {
        let clock = Clock::get()?;

        self.learner_profile.set_inner(LearnerProfile {
            user: self.user.key(),
            created_at_timestamp: clock.unix_timestamp,
            total_topics_completed: 0,
            total_quizzes_attempted: 0,
            bump: bumps.learner_profile,
        });

        msg!("inQrio: Learner profile initialized for {}", self.user.key());
        Ok(())
    }
}

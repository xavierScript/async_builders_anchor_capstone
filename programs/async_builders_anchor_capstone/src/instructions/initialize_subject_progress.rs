use anchor_lang::prelude::*;

use crate::state::{LearnerProfile, SubjectProgress};

#[derive(Accounts)]
#[instruction(subject_id: u32)]
pub struct InitializeSubjectProgress<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"learner", user.key().as_ref()],
        bump = learner_profile.bump,
        has_one = user,
    )]
    pub learner_profile: Account<'info, LearnerProfile>,

    #[account(
        init,
        payer = user,
        space = SubjectProgress::DISCRIMINATOR.len() + SubjectProgress::INIT_SPACE,
        seeds = [b"subject", user.key().as_ref(), subject_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub subject_progress: Account<'info, SubjectProgress>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeSubjectProgress<'info> {
    pub fn initialize_subject_progress(
        &mut self,
        subject_id: u32,
        bumps: &InitializeSubjectProgressBumps,
    ) -> Result<()> {
        let clock = Clock::get()?;

        self.subject_progress.set_inner(SubjectProgress {
            user: self.user.key(),
            subject_id,
            topics_completed: 0,
            quizzes_completed: 0,
            last_activity_timestamp: clock.unix_timestamp,
            bump: bumps.subject_progress,
        });

        msg!(
            "inQrio: Subject progress initialized — user={}, subject_id={}",
            self.user.key(),
            subject_id
        );
        Ok(())
    }
}

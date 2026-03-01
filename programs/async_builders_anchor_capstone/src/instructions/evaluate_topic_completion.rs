use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::{LearnerProfile, SubjectProgress, TopicCompletion};

#[derive(Accounts)]
#[instruction(subject_id: u32, topic_id: u32)]
pub struct EvaluateTopicCompletion<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"learner", user.key().as_ref()],
        bump = learner_profile.bump,
        has_one = user,
    )]
    pub learner_profile: Account<'info, LearnerProfile>,

    #[account(
        mut,
        seeds = [b"subject", user.key().as_ref(), subject_id.to_le_bytes().as_ref()],
        bump = subject_progress.bump,
        has_one = user,
    )]
    pub subject_progress: Account<'info, SubjectProgress>,

    #[account(
        init,
        payer = user,
        space = TopicCompletion::DISCRIMINATOR.len() + TopicCompletion::INIT_SPACE,
        seeds = [b"topic", user.key().as_ref(), topic_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub topic_completion: Account<'info, TopicCompletion>,

    pub system_program: Program<'info, System>,
}

impl<'info> EvaluateTopicCompletion<'info> {
    pub fn evaluate_topic_completion(
        &mut self,
        subject_id: u32,
        topic_id: u32,
        mastery_level: u8,
        bumps: &EvaluateTopicCompletionBumps,
    ) -> Result<()> {
        require!(mastery_level <= 2, ErrorCode::InvalidMasteryLevel);

        let clock = Clock::get()?;

        // Populate topic completion PDA
        self.topic_completion.set_inner(TopicCompletion {
            user: self.user.key(),
            subject_id,
            topic_id,
            mastery_level,
            completed_at: clock.unix_timestamp,
            bump: bumps.topic_completion,
        });

        // Update learner profile
        self.learner_profile.total_topics_completed = self
            .learner_profile
            .total_topics_completed
            .checked_add(1)
            .unwrap();

        // Update subject progress
        self.subject_progress.topics_completed = self
            .subject_progress
            .topics_completed
            .checked_add(1)
            .unwrap();
        self.subject_progress.last_activity_timestamp = clock.unix_timestamp;

        msg!(
            "inQrio: Topic completed — topic_id={}, mastery={}",
            topic_id,
            mastery_level
        );
        Ok(())
    }
}

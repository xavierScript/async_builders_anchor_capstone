use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::{LearnerProfile, QuizAttempt, SubjectProgress};

#[derive(Accounts)]
#[instruction(subject_id: u32, topic_id: u32, quiz_id: u32)]
pub struct RecordQuizAttempt<'info> {
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
        space = QuizAttempt::DISCRIMINATOR.len() + QuizAttempt::INIT_SPACE,
        seeds = [b"quiz", user.key().as_ref(), quiz_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub quiz_attempt: Account<'info, QuizAttempt>,

    pub system_program: Program<'info, System>,
}

impl<'info> RecordQuizAttempt<'info> {
    pub fn record_quiz_attempt(
        &mut self,
        subject_id: u32,
        topic_id: u32,
        quiz_id: u32,
        score: u8,
        time_taken: u32,
        bumps: &RecordQuizAttemptBumps,
    ) -> Result<()> {
        require!(score <= 100, ErrorCode::InvalidScore);

        let clock = Clock::get()?;

        // Derive score band on-chain: 0 = Fail (<50), 1 = Pass (50-79), 2 = Outstanding (80+)
        let score_band: u8 = if score >= 80 {
            2 // Outstanding
        } else if score >= 50 {
            1 // Pass
        } else {
            0 // Fail
        };

        // Populate quiz attempt PDA
        self.quiz_attempt.set_inner(QuizAttempt {
            user: self.user.key(),
            subject_id,
            topic_id,
            quiz_id,
            score,
            score_band,
            time_taken,
            bump: bumps.quiz_attempt,
        });

        // Update learner profile counters
        self.learner_profile.total_quizzes_attempted = self
            .learner_profile
            .total_quizzes_attempted
            .checked_add(1)
            .unwrap();

        // Update subject progress counters
        self.subject_progress.quizzes_completed = self
            .subject_progress
            .quizzes_completed
            .checked_add(1)
            .unwrap();
        self.subject_progress.last_activity_timestamp = clock.unix_timestamp;

        msg!(
            "inQrio: Quiz attempt recorded — quiz_id={}, score={}, band={}",
            quiz_id,
            score,
            score_band
        );
        Ok(())
    }
}

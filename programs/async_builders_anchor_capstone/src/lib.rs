use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("DE4Q3C4boDBeMobfbaSxETX9EmW4984HbZP9rKfDbMPQ");

#[program]
pub mod async_builders_anchor_capstone {
    use super::*;

    // ─── Learning & Assessment Instructions ────────────────────────────

    /// Create a new learner profile for the signing wallet.
    pub fn initialize_learner(ctx: Context<InitializeLearner>) -> Result<()> {
        ctx.accounts.initialize_learner(&ctx.bumps)
    }

    /// Initialize subject-level progress tracking for a learner.
    pub fn initialize_subject_progress(
        ctx: Context<InitializeSubjectProgress>,
        subject_id: u32,
    ) -> Result<()> {
        ctx.accounts.initialize_subject_progress(subject_id, &ctx.bumps)
    }

    /// Record a quiz attempt with on-chain score-band derivation.
    pub fn record_quiz_attempt(
        ctx: Context<RecordQuizAttempt>,
        subject_id: u32,
        topic_id: u32,
        quiz_id: u32,
        score: u8,
        time_taken: u32,
    ) -> Result<()> {
        ctx.accounts.record_quiz_attempt(subject_id, topic_id, quiz_id, score, time_taken, &ctx.bumps)
    }

    /// Mark a topic as completed with a mastery level.
    pub fn evaluate_topic_completion(
        ctx: Context<EvaluateTopicCompletion>,
        subject_id: u32,
        topic_id: u32,
        mastery_level: u8,
    ) -> Result<()> {
        ctx.accounts.evaluate_topic_completion(subject_id, topic_id, mastery_level, &ctx.bumps)
    }

    // ─── Tournament Management Instructions ────────────────────────────

    /// Admin creates a new tournament.
    pub fn create_tournament(
        ctx: Context<CreateTournament>,
        tournament_id: u32,
        subject_id: u32,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        ctx.accounts.create_tournament(tournament_id, subject_id, start_time, end_time, &ctx.bumps)
    }

    /// Student registers as a tournament participant.
    pub fn register_participant(
        ctx: Context<RegisterParticipant>,
        tournament_id: u32,
    ) -> Result<()> {
        ctx.accounts.register_participant(tournament_id, &ctx.bumps)
    }

    /// Participant submits their tournament score.
    pub fn submit_tournament_score(
        ctx: Context<SubmitTournamentScore>,
        tournament_id: u32,
        score: u32,
    ) -> Result<()> {
        ctx.accounts.submit_tournament_score(tournament_id, score)
    }

    /// Admin updates the tournament status (Pending → Active → Ended).
    pub fn update_tournament_status(
        ctx: Context<UpdateTournamentStatus>,
        new_status: u8,
    ) -> Result<()> {
        ctx.accounts.update_tournament_status(new_status)
    }
}

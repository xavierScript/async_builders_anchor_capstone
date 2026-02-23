use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("This topic has already been completed by the learner.")]
    TopicAlreadyCompleted,

    #[msg("Quiz score is below the passing threshold.")]
    QuizScoreBelowThreshold,

    #[msg("Tournament is not currently active.")]
    TournamentNotActive,

    #[msg("Tournament has already ended.")]
    TournamentAlreadyEnded,

    #[msg("Only the tournament admin can perform this action.")]
    UnauthorizedAdmin,

    #[msg("Score value is out of the valid range (0-100).")]
    InvalidScore,

    #[msg("Mastery level must be 0 (Basic), 1 (Solid), or 2 (Strong).")]
    InvalidMasteryLevel,

    #[msg("Tournament has reached its maximum number of participants.")]
    TournamentFull,

    #[msg("Participant is already registered for this tournament.")]
    AlreadyRegistered,

    #[msg("Start time must be before end time.")]
    InvalidTimestamp,

    #[msg("Invalid tournament status transition.")]
    InvalidStatusTransition,
}

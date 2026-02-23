use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LearnerProfile {
    pub user: Pubkey,                  // 32
    pub created_at_timestamp: i64,     // 8
    pub total_topics_completed: u32,   // 4
    pub total_quizzes_attempted: u32,  // 4
    pub bump: u8,                      // 1
}

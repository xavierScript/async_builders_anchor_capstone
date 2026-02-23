use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SubjectProgress {
    pub user: Pubkey,                    // 32
    pub subject_id: u32,                 // 4
    pub topics_completed: u32,           // 4
    pub quizzes_completed: u32,          // 4
    pub last_activity_timestamp: i64,    // 8
    pub bump: u8,                        // 1
}

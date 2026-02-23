use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct QuizAttempt {
    pub user: Pubkey,       // 32
    pub subject_id: u32,    // 4
    pub topic_id: u32,      // 4
    pub quiz_id: u32,       // 4
    pub score: u8,          // 1
    pub score_band: u8,     // 1  (0 = Fail, 1 = Pass, 2 = Outstanding)
    pub time_taken: u32,    // 4  (seconds)
    pub bump: u8,           // 1
}

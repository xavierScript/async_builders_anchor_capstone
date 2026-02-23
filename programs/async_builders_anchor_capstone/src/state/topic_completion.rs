use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TopicCompletion {
    pub user: Pubkey,       // 32
    pub subject_id: u32,    // 4
    pub topic_id: u32,      // 4
    pub mastery_level: u8,  // 1  (0 = Basic, 1 = Solid, 2 = Strong)
    pub completed_at: i64,  // 8
    pub bump: u8,           // 1
}

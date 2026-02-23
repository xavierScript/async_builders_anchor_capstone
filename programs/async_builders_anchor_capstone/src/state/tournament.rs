use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Tournament {
    pub admin: Pubkey,                 // 32
    pub tournament_id: u32,            // 4
    pub subject_id: u32,              // 4
    pub start_time: i64,              // 8
    pub end_time: i64,                // 8
    pub number_of_participants: u32,  // 4
    pub status: u8,                   // 1  (0 = Pending, 1 = Active, 2 = Ended)
    pub bump: u8,                     // 1
}

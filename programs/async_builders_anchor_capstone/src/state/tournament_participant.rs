use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TournamentParticipant {
    pub user: Pubkey,          // 32
    pub tournament_id: u32,    // 4
    pub entry_number: u32,     // 4
    pub score: u32,            // 4
    pub rank: u32,             // 4
    pub bump: u8,              // 1
}

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PlatformConfig {
    pub admin: Pubkey,             // 32
    pub pass_threshold: u8,        // 1   (default 50)
    pub outstanding_threshold: u8, // 1   (default 80)
    pub bump: u8,                  // 1
}

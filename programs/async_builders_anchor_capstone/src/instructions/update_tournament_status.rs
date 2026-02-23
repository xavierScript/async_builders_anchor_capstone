use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::Tournament;

#[derive(Accounts)]
pub struct UpdateTournamentStatus<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        has_one = admin @ ErrorCode::UnauthorizedAdmin,
    )]
    pub tournament: Account<'info, Tournament>,
}

impl<'info> UpdateTournamentStatus<'info> {
    pub fn update_tournament_status(&mut self, new_status: u8) -> Result<()> {
        // Validate status transitions: Pending(0) → Active(1) → Ended(2)
        let valid_transition = match (self.tournament.status, new_status) {
            (0, 1) => true, // Pending → Active
            (1, 2) => true, // Active  → Ended
            _ => false,
        };
        require!(valid_transition, ErrorCode::InvalidStatusTransition);

        self.tournament.status = new_status;

        msg!(
            "inQrio: Tournament {} status updated to {}",
            self.tournament.tournament_id,
            new_status
        );
        Ok(())
    }
}

use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::Tournament;

#[derive(Accounts)]
#[instruction(tournament_id: u32)]
pub struct CreateTournament<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Tournament::DISCRIMINATOR.len() + Tournament::INIT_SPACE,
        seeds = [b"tournament", tournament_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub tournament: Account<'info, Tournament>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateTournament<'info> {
    pub fn create_tournament(
        &mut self,
        tournament_id: u32,
        subject_id: u32,
        start_time: i64,
        end_time: i64,
        bumps: &CreateTournamentBumps,
    ) -> Result<()> {
        require!(start_time < end_time, ErrorCode::InvalidTimestamp);

        self.tournament.set_inner(Tournament {
            admin: self.admin.key(),
            tournament_id,
            subject_id,
            start_time,
            end_time,
            number_of_participants: 0,
            status: 0, // Pending
            bump: bumps.tournament,
        });

        msg!(
            "inQrio: Tournament created — id={}, subject={}, admin={}",
            tournament_id,
            subject_id,
            self.admin.key()
        );
        Ok(())
    }
}

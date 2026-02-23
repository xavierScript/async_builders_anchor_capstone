use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::{Tournament, TournamentParticipant};

#[derive(Accounts)]
#[instruction(tournament_id: u32)]
pub struct RegisterParticipant<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"tournament", tournament_id.to_le_bytes().as_ref()],
        bump = tournament.bump,
        constraint = tournament.tournament_id == tournament_id,
    )]
    pub tournament: Account<'info, Tournament>,

    #[account(
        init,
        payer = user,
        space = 8 + TournamentParticipant::INIT_SPACE,
        seeds = [b"tournament_participant", user.key().as_ref(), tournament_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub tournament_participant: Account<'info, TournamentParticipant>,

    pub system_program: Program<'info, System>,
}

impl<'info> RegisterParticipant<'info> {
    pub fn register_participant(
        &mut self,
        tournament_id: u32,
        bumps: &RegisterParticipantBumps,
    ) -> Result<()> {
        // Tournament must be Pending (0) or Active (1) to accept registrations
        require!(
            self.tournament.status == 0 || self.tournament.status == 1,
            ErrorCode::TournamentAlreadyEnded
        );

        // Increment participant count and assign entry number
        self.tournament.number_of_participants = self
            .tournament
            .number_of_participants
            .checked_add(1)
            .unwrap();

        self.tournament_participant.set_inner(TournamentParticipant {
            user: self.user.key(),
            tournament_id,
            entry_number: self.tournament.number_of_participants,
            score: 0,
            rank: 0,
            bump: bumps.tournament_participant,
        });

        msg!(
            "inQrio: Participant registered — tournament={}, user={}, entry#={}",
            tournament_id,
            self.user.key(),
            self.tournament.number_of_participants
        );
        Ok(())
    }
}

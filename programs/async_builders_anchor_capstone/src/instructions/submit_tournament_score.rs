use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::{Tournament, TournamentParticipant};

#[derive(Accounts)]
#[instruction(tournament_id: u32)]
pub struct SubmitTournamentScore<'info> {
    pub user: Signer<'info>,

    #[account(
        seeds = [b"tournament", tournament_id.to_le_bytes().as_ref()],
        bump = tournament.bump,
        constraint = tournament.tournament_id == tournament_id,
    )]
    pub tournament: Account<'info, Tournament>,

    #[account(
        mut,
        seeds = [b"tournament_participant", user.key().as_ref(), tournament_id.to_le_bytes().as_ref()],
        bump = tournament_participant.bump,
        has_one = user,
    )]
    pub tournament_participant: Account<'info, TournamentParticipant>,
}

impl<'info> SubmitTournamentScore<'info> {
    pub fn submit_tournament_score(
        &mut self,
        tournament_id: u32,
        score: u32,
    ) -> Result<()> {
        // Tournament must be Active (1) to accept score submissions
        require!(self.tournament.status == 1, ErrorCode::TournamentNotActive);

        self.tournament_participant.score = score;

        msg!(
            "inQrio: Score submitted — tournament={}, user={}, score={}",
            tournament_id,
            self.tournament_participant.user,
            score
        );
        Ok(())
    }
}

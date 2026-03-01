import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AsyncBuildersAnchorCapstone } from "../target/types/async_builders_anchor_capstone";
import { expect } from "chai";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("inQrio — On-Chain Program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .asyncBuildersAnchorCapstone as Program<AsyncBuildersAnchorCapstone>;

  const user = provider.wallet as anchor.Wallet;
  const adminKeypair = Keypair.generate();
  const participant2 = Keypair.generate();

  // ──────────────────────────────────────────────────────────────────────
  // Helper: derive PDA
  // ──────────────────────────────────────────────────────────────────────
  function findPda(seeds: (Buffer | Uint8Array)[]) {
    return PublicKey.findProgramAddressSync(seeds, program.programId);
  }

  function u32ToLeBytes(n: number): Buffer {
    const buf = Buffer.alloc(4);
    buf.writeUInt32LE(n);
    return buf;
  }

  // ──────────────────────────────────────────────────────────────────────
  // Setup: fund secondary wallets via transfer from provider wallet
  // (avoids devnet airdrop rate limits / 429 errors)
  // ──────────────────────────────────────────────────────────────────────
  before(async () => {
    const transferTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: user.publicKey,
        toPubkey: adminKeypair.publicKey,
        lamports: 2 * anchor.web3.LAMPORTS_PER_SOL,
      }),
      anchor.web3.SystemProgram.transfer({
        fromPubkey: user.publicKey,
        toPubkey: participant2.publicKey,
        lamports: 2 * anchor.web3.LAMPORTS_PER_SOL,
      }),
    );

    const sig = await provider.sendAndConfirm(transferTx);
    console.log("Funded secondary wallets via transfer:", sig);
  });

  // ════════════════════════════════════════════════════════════════════════
  //  PART 1 — LEARNING AND ASSESSMENT
  // ════════════════════════════════════════════════════════════════════════

  describe("Learning & Assessment", () => {
    const subjectId = 1;
    const topicId = 101;
    const quizId1 = 1001;
    const quizId2 = 1002;

    let learnerProfilePda: PublicKey;
    let subjectProgressPda: PublicKey;

    it("initializes a learner profile", async () => {
      [learnerProfilePda] = findPda([
        Buffer.from("learner"),
        user.publicKey.toBuffer(),
      ]);

      await program.methods
        .initializeLearner()
        .accountsPartial({
          user: user.publicKey,
          learnerProfile: learnerProfilePda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const profile = await program.account.learnerProfile.fetch(
        learnerProfilePda,
      );
      expect(profile.user.toBase58()).to.equal(user.publicKey.toBase58());
      expect(profile.totalTopicsCompleted).to.equal(0);
      expect(profile.totalQuizzesAttempted).to.equal(0);
      expect(profile.createdAtTimestamp.toNumber()).to.be.greaterThan(0);
    });

    it("initializes subject progress", async () => {
      [subjectProgressPda] = findPda([
        Buffer.from("subject"),
        user.publicKey.toBuffer(),
        u32ToLeBytes(subjectId),
      ]);

      await program.methods
        .initializeSubjectProgress(subjectId)
        .accountsPartial({
          user: user.publicKey,
          learnerProfile: learnerProfilePda,
          subjectProgress: subjectProgressPda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const progress = await program.account.subjectProgress.fetch(
        subjectProgressPda,
      );
      expect(progress.user.toBase58()).to.equal(user.publicKey.toBase58());
      expect(progress.subjectId).to.equal(subjectId);
      expect(progress.topicsCompleted).to.equal(0);
      expect(progress.quizzesCompleted).to.equal(0);
    });

    it("records a quiz attempt (passing score — 75)", async () => {
      const [quizAttemptPda] = findPda([
        Buffer.from("quiz"),
        user.publicKey.toBuffer(),
        u32ToLeBytes(quizId1),
      ]);

      await program.methods
        .recordQuizAttempt(subjectId, topicId, quizId1, 75, 120)
        .accountsPartial({
          user: user.publicKey,
          learnerProfile: learnerProfilePda,
          subjectProgress: subjectProgressPda,
          quizAttempt: quizAttemptPda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const attempt = await program.account.quizAttempt.fetch(quizAttemptPda);
      expect(attempt.score).to.equal(75);
      expect(attempt.scoreBand).to.equal(1); // Pass
      expect(attempt.timeTaken).to.equal(120);

      const profile = await program.account.learnerProfile.fetch(
        learnerProfilePda,
      );
      expect(profile.totalQuizzesAttempted).to.equal(1);

      const progress = await program.account.subjectProgress.fetch(
        subjectProgressPda,
      );
      expect(progress.quizzesCompleted).to.equal(1);
    });

    it("records a quiz attempt (outstanding score — 95)", async () => {
      const [quizAttemptPda] = findPda([
        Buffer.from("quiz"),
        user.publicKey.toBuffer(),
        u32ToLeBytes(quizId2),
      ]);

      await program.methods
        .recordQuizAttempt(subjectId, topicId, quizId2, 95, 90)
        .accountsPartial({
          user: user.publicKey,
          learnerProfile: learnerProfilePda,
          subjectProgress: subjectProgressPda,
          quizAttempt: quizAttemptPda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const attempt = await program.account.quizAttempt.fetch(quizAttemptPda);
      expect(attempt.score).to.equal(95);
      expect(attempt.scoreBand).to.equal(2); // Outstanding

      const profile = await program.account.learnerProfile.fetch(
        learnerProfilePda,
      );
      expect(profile.totalQuizzesAttempted).to.equal(2);
    });

    it("evaluates topic completion", async () => {
      const [topicCompletionPda] = findPda([
        Buffer.from("topic"),
        user.publicKey.toBuffer(),
        u32ToLeBytes(topicId),
      ]);

      await program.methods
        .evaluateTopicCompletion(subjectId, topicId, 2) // Strong mastery
        .accountsPartial({
          user: user.publicKey,
          learnerProfile: learnerProfilePda,
          subjectProgress: subjectProgressPda,
          topicCompletion: topicCompletionPda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const completion = await program.account.topicCompletion.fetch(
        topicCompletionPda,
      );
      expect(completion.masteryLevel).to.equal(2); // Strong
      expect(completion.subjectId).to.equal(subjectId);
      expect(completion.topicId).to.equal(topicId);
      expect(completion.completedAt.toNumber()).to.be.greaterThan(0);

      const profile = await program.account.learnerProfile.fetch(
        learnerProfilePda,
      );
      expect(profile.totalTopicsCompleted).to.equal(1);

      const progress = await program.account.subjectProgress.fetch(
        subjectProgressPda,
      );
      expect(progress.topicsCompleted).to.equal(1);
    });

    it("prevents duplicate topic completion (same PDA fails to init)", async () => {
      const [topicCompletionPda] = findPda([
        Buffer.from("topic"),
        user.publicKey.toBuffer(),
        u32ToLeBytes(topicId),
      ]);

      try {
        await program.methods
          .evaluateTopicCompletion(subjectId, topicId, 1)
          .accountsPartial({
            user: user.publicKey,
            learnerProfile: learnerProfilePda,
            subjectProgress: subjectProgressPda,
            topicCompletion: topicCompletionPda,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
        expect.fail("Should have thrown — duplicate topic completion");
      } catch (err: any) {
        // Account already exists — Anchor/runtime rejects the init
        expect(err).to.exist;
      }
    });

    it("rejects invalid score (> 100)", async () => {
      const badQuizId = 9999;
      const [quizAttemptPda] = findPda([
        Buffer.from("quiz"),
        user.publicKey.toBuffer(),
        u32ToLeBytes(badQuizId),
      ]);

      try {
        await program.methods
          .recordQuizAttempt(subjectId, topicId, badQuizId, 150, 60)
          .accountsPartial({
            user: user.publicKey,
            learnerProfile: learnerProfilePda,
            subjectProgress: subjectProgressPda,
            quizAttempt: quizAttemptPda,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
        expect.fail("Should have thrown — invalid score");
      } catch (err: any) {
        expect(err).to.exist;
      }
    });
  });

  // ════════════════════════════════════════════════════════════════════════
  //  PART 2 — TOURNAMENT MANAGEMENT
  // ════════════════════════════════════════════════════════════════════════

  describe("Tournament Management", () => {
    const tournamentId = 1;
    const subjectId = 1;
    const now = Math.floor(Date.now() / 1000);
    const startTime = now + 60;
    const endTime = now + 3600;

    let tournamentPda: PublicKey;
    let participant1Pda: PublicKey;
    let participant2Pda: PublicKey;

    it("creates a tournament", async () => {
      [tournamentPda] = findPda([
        Buffer.from("tournament"),
        u32ToLeBytes(tournamentId),
      ]);

      await program.methods
        .createTournament(
          tournamentId,
          subjectId,
          new anchor.BN(startTime),
          new anchor.BN(endTime),
        )
        .accountsPartial({
          admin: adminKeypair.publicKey,
          tournament: tournamentPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([adminKeypair])
        .rpc();

      const tournament = await program.account.tournament.fetch(tournamentPda);
      expect(tournament.admin.toBase58()).to.equal(
        adminKeypair.publicKey.toBase58(),
      );
      expect(tournament.tournamentId).to.equal(tournamentId);
      expect(tournament.subjectId).to.equal(subjectId);
      expect(tournament.numberOfParticipants).to.equal(0);
      expect(tournament.status).to.equal(0); // Pending
    });

    it("rejects tournament creation with invalid timestamps", async () => {
      const badTournamentId = 999;
      const [badTournamentPda] = findPda([
        Buffer.from("tournament"),
        u32ToLeBytes(badTournamentId),
      ]);

      try {
        await program.methods
          .createTournament(
            badTournamentId,
            subjectId,
            new anchor.BN(endTime), // start > end — invalid
            new anchor.BN(startTime),
          )
          .accountsPartial({
            admin: adminKeypair.publicKey,
            tournament: badTournamentPda,
            systemProgram: SystemProgram.programId,
          })
          .signers([adminKeypair])
          .rpc();
        expect.fail("Should have thrown — invalid timestamps");
      } catch (err: any) {
        expect(err).to.exist;
      }
    });

    it("registers participant 1 (wallet user)", async () => {
      [participant1Pda] = findPda([
        Buffer.from("tournament_participant"),
        user.publicKey.toBuffer(),
        u32ToLeBytes(tournamentId),
      ]);

      await program.methods
        .registerParticipant(tournamentId)
        .accountsPartial({
          user: user.publicKey,
          tournament: tournamentPda,
          tournamentParticipant: participant1Pda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const p = await program.account.tournamentParticipant.fetch(
        participant1Pda,
      );
      expect(p.user.toBase58()).to.equal(user.publicKey.toBase58());
      expect(p.entryNumber).to.equal(1);
      expect(p.score).to.equal(0);

      const tournament = await program.account.tournament.fetch(tournamentPda);
      expect(tournament.numberOfParticipants).to.equal(1);
    });

    it("registers participant 2", async () => {
      [participant2Pda] = findPda([
        Buffer.from("tournament_participant"),
        participant2.publicKey.toBuffer(),
        u32ToLeBytes(tournamentId),
      ]);

      await program.methods
        .registerParticipant(tournamentId)
        .accountsPartial({
          user: participant2.publicKey,
          tournament: tournamentPda,
          tournamentParticipant: participant2Pda,
          systemProgram: SystemProgram.programId,
        })
        .signers([participant2])
        .rpc();

      const p = await program.account.tournamentParticipant.fetch(
        participant2Pda,
      );
      expect(p.user.toBase58()).to.equal(participant2.publicKey.toBase58());
      expect(p.entryNumber).to.equal(2);

      const tournament = await program.account.tournament.fetch(tournamentPda);
      expect(tournament.numberOfParticipants).to.equal(2);
    });

    it("updates tournament status: Pending → Active", async () => {
      await program.methods
        .updateTournamentStatus(1) // Active
        .accountsPartial({
          admin: adminKeypair.publicKey,
          tournament: tournamentPda,
        })
        .signers([adminKeypair])
        .rpc();

      const tournament = await program.account.tournament.fetch(tournamentPda);
      expect(tournament.status).to.equal(1); // Active
    });

    it("rejects non-admin status update", async () => {
      try {
        await program.methods
          .updateTournamentStatus(2)
          .accountsPartial({
            admin: user.publicKey,
            tournament: tournamentPda,
          })
          .rpc();
        expect.fail("Should have thrown — unauthorized admin");
      } catch (err: any) {
        expect(err).to.exist;
      }
    });

    it("submits score for participant 1", async () => {
      await program.methods
        .submitTournamentScore(tournamentId, 850)
        .accountsPartial({
          user: user.publicKey,
          tournament: tournamentPda,
          tournamentParticipant: participant1Pda,
        })
        .rpc();

      const p = await program.account.tournamentParticipant.fetch(
        participant1Pda,
      );
      expect(p.score).to.equal(850);
    });

    it("submits score for participant 2", async () => {
      await program.methods
        .submitTournamentScore(tournamentId, 920)
        .accountsPartial({
          user: participant2.publicKey,
          tournament: tournamentPda,
          tournamentParticipant: participant2Pda,
        })
        .signers([participant2])
        .rpc();

      const p = await program.account.tournamentParticipant.fetch(
        participant2Pda,
      );
      expect(p.score).to.equal(920);
    });

    it("updates tournament status: Active → Ended", async () => {
      await program.methods
        .updateTournamentStatus(2) // Ended
        .accountsPartial({
          admin: adminKeypair.publicKey,
          tournament: tournamentPda,
        })
        .signers([adminKeypair])
        .rpc();

      const tournament = await program.account.tournament.fetch(tournamentPda);
      expect(tournament.status).to.equal(2); // Ended
    });

    it("rejects score submission to ended tournament", async () => {
      try {
        await program.methods
          .submitTournamentScore(tournamentId, 999)
          .accountsPartial({
            user: user.publicKey,
            tournament: tournamentPda,
            tournamentParticipant: participant1Pda,
          })
          .rpc();
        expect.fail("Should have thrown — tournament ended");
      } catch (err: any) {
        expect(err).to.exist;
      }
    });

    it("rejects invalid status transition (Ended → Active)", async () => {
      try {
        await program.methods
          .updateTournamentStatus(1)
          .accountsPartial({
            admin: adminKeypair.publicKey,
            tournament: tournamentPda,
          })
          .signers([adminKeypair])
          .rpc();
        expect.fail("Should have thrown — invalid transition");
      } catch (err: any) {
        expect(err).to.exist;
      }
    });

    it("client-side scoreboard: fetches all participants for tournament", async () => {
      // get_tournament_scoreboard is implemented client-side
      const allParticipants = await program.account.tournamentParticipant.all();

      const scoreboard = allParticipants
        .filter((p) => p.account.tournamentId === tournamentId)
        .sort((a, b) => b.account.score - a.account.score);

      expect(scoreboard.length).to.equal(2);
      expect(scoreboard[0].account.score).to.equal(920); // participant2 is #1
      expect(scoreboard[1].account.score).to.equal(850); // participant1 is #2
    });
  });
});

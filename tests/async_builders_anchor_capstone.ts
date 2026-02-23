import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AsyncBuildersAnchorCapstone } from "../target/types/async_builders_anchor_capstone";

describe("async_builders_anchor_capstone", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.asyncBuildersAnchorCapstone as Program<AsyncBuildersAnchorCapstone>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

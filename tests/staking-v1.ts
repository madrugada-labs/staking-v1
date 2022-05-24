import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { JobFactory } from "../target/types/job_factory";

describe("staking-v1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.JobFactory as Program<JobFactory>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StakingV1 } from "../target/types/staking_v1";

describe("staking-v1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StakingV1 as Program<StakingV1>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

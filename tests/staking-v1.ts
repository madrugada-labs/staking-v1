import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { JobFactory } from "../target/types/job_factory";
import {ApplicationFactory} from "../target/types/application_factory";
import {v4 as uuidv4} from "uuid";
const spl = require("@solana/spl-token");

const assert = require("assert");

describe("staking-v1", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const jobProgram = anchor.workspace.JobFactory as Program<JobFactory>;
  const applicationProgram = anchor.workspace.ApplicationFactory as Program<ApplicationFactory>;

  let alice = anchor.web3.Keypair.generate(); // HR 
  let bob = anchor.web3.Keypair.generate(); // Applicant
  let cas = anchor.web3.Keypair.generate(); // Stakeholder
  let dan = anchor.web3.Keypair.generate(); // Stakeholder
  let tokenMint = anchor.web3.Keypair.generate(); // Token Mint

  it("Funds all users", async() => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        alice.publicKey,
        10000000000
      ),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        bob.publicKey,
        10000000000
      ),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        cas.publicKey,
        10000000000
      ),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        dan.publicKey,
        10000000000
      ),
      "confirmed"
    );

    const aliceUserBalance = await provider.connection.getBalance(alice.publicKey);
    const bobUserBalance = await provider.connection.getBalance(bob.publicKey);
    const casUserBalance = await provider.connection.getBalance(cas.publicKey);
    const danUserBalance = await provider.connection.getBalance(dan.publicKey);

    assert.strictEqual(10000000000, aliceUserBalance);
    assert.strictEqual(10000000000, bobUserBalance);
    assert.strictEqual(10000000000, casUserBalance);
    assert.strictEqual(10000000000, danUserBalance);


  })

  const jobAdId = uuidv4();

  it("Initializing Job Factory", async () => {
    // Add your test here.

    const maxAmountPerApplication = 100000;

    const [jobFactoryPDA, jobFactoryBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("jobfactory"), Buffer.from(jobAdId.substring(0, 18)), Buffer.from(jobAdId.substring(18, 36))],
      jobProgram.programId
    );

    const tx = await jobProgram.methods.initialize(jobAdId, maxAmountPerApplication).accounts({
      baseAccount: jobFactoryPDA,
      authority: alice.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([alice]).rpc();

    console.log("Your transaction signature", tx);

    const jobFactoryState = await jobProgram.account.jobStakingParameter.fetch(jobFactoryPDA);

    assert.strictEqual(jobAdId, jobFactoryState.jobAdId);
    assert.strictEqual(alice.publicKey.toBase58(), jobFactoryState.authority.toBase58());
    assert.strictEqual(maxAmountPerApplication, jobFactoryState.maxAmountPerApplication);

  });

  it("Initializing Application Factory", async () => {

    const [applicationFactoryPDA, applicationFactoryBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("job_application"), Buffer.from(jobAdId.substring(0,18)), Buffer.from(jobAdId.substring(18,36)), bob.publicKey.toBuffer()],
      applicationProgram.programId
    );

    const [mintPDA, mintBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("job_application")],
      applicationProgram.programId
    )

    const tx = await applicationProgram.methods.initialize(jobAdId, mintBump, applicationFactoryBump).accounts({
      baseAccount: applicationFactoryPDA,
      authority: bob.publicKey,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      mintAccount: mintPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([bob]).rpc();

    console.log("Your transaction signature", tx);

    let newMint = await provider.connection.getAccountInfo(mintPDA);
    console.log(newMint)

    const tokenAddress = new anchor.web3.PublicKey("Bigvd91m6dugF4kf5scVXFjYzNDwExgqQ59SJJzeTri9");

    console.log(mintPDA.toBase58())

    const tokenMint = await spl.createMint(
      provider.connection,
      bob,
      bob.publicKey,
      null,
      6
    );

    const bobTokenWallet = await spl.createAccount(
      provider.connection,
      bob,
      mintPDA,
      bob.publicKey
    );

    let _bobTokenWallet = await spl.getAccount(
      provider.connection,
      bobTokenWallet
    );

    console.log(_bobTokenWallet)

    const jobApplicationState = await applicationProgram.account.jobApplication.fetch(applicationFactoryPDA);

    console.log(jobApplicationState.status);

  })

});

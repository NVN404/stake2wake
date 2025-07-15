import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stake2wake } from "../target/types/stake2wake";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { assert, use } from "chai";

describe("stake2wake", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()

  anchor.setProvider(provider);

  const program = anchor.workspace.Stake2wake as Program<Stake2wake>;

  const admin = provider.wallet;
  const user = anchor.web3.Keypair.generate();

  let bonkMint: anchor.web3.PublicKey;
  let bonkAta: anchor.web3.PublicKey;
  let treasuryPda: anchor.web3.PublicKey;
  let treasuryAta: anchor.web3.PublicKey;
  let treasuryBump: number;

  before(async () => {
    bonkMint = await createMint(
      provider.connection,
      admin.payer,
      admin.publicKey,
      null,
      6
    );

    bonkAta = getAssociatedTokenAddressSync(
      bonkMint,
      admin.publicKey,
      true
    );

    [treasuryPda, treasuryBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), admin.publicKey.toBuffer()],
      program.programId
    );

    treasuryAta = getAssociatedTokenAddressSync(
      bonkMint,
      treasuryPda,
      true
    );
  });

  it("Should initialize the treasury!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accountsPartial({
      authority: admin.publicKey,
      bonkMint: bonkMint,
      treasury: treasuryPda,
      treasuryAta: treasuryAta,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID
    }).signers([]).rpc();
    console.log("Your transaction signature", tx);

    const treasuryAccount = await program.account.treasury.fetch(treasuryPda);
    console.log("treasuryAccount", treasuryAccount);

    assert.equal(treasuryAccount.authority.toBase58(), admin.publicKey.toBase58());
    assert.equal(treasuryAccount.bonkMint.toBase58(), bonkMint.toBase58());
    assert.equal(treasuryAccount.treasuryAta.toBase58(), treasuryAta.toBase58());
    assert.equal(treasuryAccount.bump, treasuryBump);
    assert.equal(treasuryAccount.totalCollected.toNumber(), 0);
  });

  it("Should fail if the non-admin tries to initialize the treasury", async () => {
    let error = false;
    try {
      const tx = await program.methods.initialize().accountsPartial({
        authority: user.publicKey,
        bonkMint: bonkMint,
        treasury: treasuryPda,
        treasuryAta: treasuryAta,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID
      }).signers([user]).rpc();
      console.log("Your transaction signature", tx);

    } catch (err) {
      error = true;
      console.log("Expected error:", err.error?.errorMessage || err.message);
    }
    // other than admin no one has the access to initialize the treasury
    assert.isTrue(error, "Expected the transaction to fail, but it succeeded");
  })


  it("starts a challenge", async () => {
    // Add your test logic here.
  });

  it("fails if insufficient balance", async () => {
    // Add your test logic here.
  });

  it("checks in correctly", async () => {
    // Add your test logic here.
  });

  it("fails to check twice", async () => {
    // Add your test logic here.
  });

  it("fails outside wakeup time", async () => {
    // Add your test logic here.
  });

  it("cancels challenge with full refund", async () => {
    // Add your test logic here.
  });

  it("cancels early with 20% penalty", async () => {
    // Add your test logic here.
  });

  it("withdraws from treasury", async () => {
    // Add your test logic here.
  });

  it("fails withdraw for non-admin", async () => {
    // Add your test logic here.
  });
});


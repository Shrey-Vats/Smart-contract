import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { expect } from "chai";
import { BN } from "bn.js";

describe("excrow", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.excrow as Program<Escrow>;

  let from = provider.wallet;
  let to = anchor.web3.Keypair.generate();

  let escrowPda: anchor.web3.PublicKey;

  const AMOUNT = new BN(1_000_000_000);
  const TIME = new BN(0.01);

  before(async () => {
    let Connection = provider.connection;
    await Connection.requestAirdrop(
      from.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 10
    );
    await Connection.requestAirdrop(
      to.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 10
    );

    escrowPda = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        from.publicKey.toBuffer(),
        to.publicKey.toBuffer(),
      ],
      program.programId
    )[0];
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = program.methods.initialize(AMOUNT).accounts({
      to: to.publicKey,
    }).rpc();

    const escrow = await program.account.escrow.fetch(escrowPda);

    // check to, from, and amount equal or not
    expect(escrow.from.toBase58()).to.equal(to.publicKey.toBase58());
    expect(escrow.from.toBase58()).to.equal(from.publicKey.toBase58());
    expect(escrow.amount).to.equal(AMOUNT);
  });

  it("deposite", async () => {
    program.methods.deposit(TIME).accounts({AMOUNT})
  })
});

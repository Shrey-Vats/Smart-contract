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
    const tx = program.methods
      .initialize(AMOUNT)
      .accounts({
        to: to.publicKey,
        from: from.publicKey,
      })
      .signers([from.payer])
      .rpc();

    const escrow = await program.account.escrow.fetch(escrowPda);

    console.log(tx);
    // check to, from, and amount equal or not
    expect(escrow.to.toBase58()).to.equal(to.publicKey.toBase58());
    expect(escrow.from.toBase58()).to.equal(from.publicKey.toBase58());
    expect(escrow.amount.eq(AMOUNT)).to.be.true;
  });

  it("Depositing ", async () => {
    const connection = provider.connection;

    const fromBalnceBefore = await connection.getBalance(from.publicKey);
    const pdaBalanceBefore = await connection.getBalance(escrowPda);

    const tx = program.methods
      .deposit(TIME)
      .accounts({
        from: from.publicKey,
        escrow_account: escrowPda,
      })
      .signers([from.payer]).rpc();
    console.log(tx);

    const escrow = await program.account.escrow.fetch(escrowPda);

    const fromBalnce = await connection.getBalance(from.publicKey);
    const pdaBalance = await connection.getBalance(escrowPda);

    expect(fromBalnce).to.lessThan(fromBalnceBefore);
    expect(pdaBalance).to.greaterThan(pdaBalanceBefore);
  });

  it("Released", async () => {
    const connection = provider.connection;

    const userBalanceBefore = await connection.getBalance(to.publicKey);
    const pdaBalanceBefore = await connection.getBalance(escrowPda)

    const tx = program.methods.released().accounts({
      to: to.publicKey,
    }).rpc();
    console.log(tx);

    //pda destory

    const userBalanceAfter = await connection.getBalance(to.publicKey);
    const pdaBalanceAfter = await connection.getBalance(escrowPda);

    expect(userBalanceAfter).to.greaterThan(userBalanceBefore);
    expect(pdaBalanceAfter).to.lessThan(pdaBalanceBefore);
  })

  it("testing the refund", async () => {
    let connection = provider.connection;

    program.methods.initialize(new anchor.BN(anchor.web3.LAMPORTS_PER_SOL)).accounts({
      from: from.publicKey,
      to: to.publicKey
    }).signers([from.payer]).rpc();


    program.methods.deposit(new anchor.BN(1)).accounts({
      // seeing no suggestion. I think because their not needed any one 
    }).rpc();

    let userBalanceBefore = await connection.getBalance(from.publicKey);
    program.methods.refund().accounts({
      // seeing no suggestion. I think because their not needed any one 
    }).rpc();
    // pda close 

    let userBalanceAfter = await connection.getBalance(from.publicKey);

    expect(userBalanceAfter).to.greaterThan(userBalanceBefore);
  })

  it("testing the extract", async () => {
    let connection = provider.connection;

    program.methods.initialize(new anchor.BN(anchor.web3.LAMPORTS_PER_SOL)).accounts({
      from: from.publicKey,
      to: to.publicKey
    }).signers([from.payer]).rpc();

    program.methods.deposit(new anchor.BN(0.00001157407)).accounts({}).rpc();

    new Promise(r => setTimeout(r, 2000));

    const userBalanceBefore = await connection.getBalance(to.publicKey);

    program.methods.extract().accounts({
      receiver: to.publicKey
    }).rpc();

    const userBalanceAfter = await connection.getBalance(to.publicKey);


    expect(userBalanceAfter).to.greaterThan(userBalanceBefore);

  })
});

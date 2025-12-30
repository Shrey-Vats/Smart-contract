import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleWallet } from "../target/types/simple_wallet";
import { BN } from "bn.js";
import { expect } from "chai";

describe("simple_wallet", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.simpleWallet as Program<SimpleWallet>;

  let authority = provider.wallet;
  let attacker = anchor.web3.Keypair.generate();

  let walletPDA: anchor.web3.PublicKey;

  before(async () => {
    const connection = provider.connection;

    //attacker
    await connection.requestAirdrop(
      attacker.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );

    //authority
    await connection.requestAirdrop(
      authority.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );

    [walletPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), authority.publicKey.toBuffer()],
      program.programId
    );
  });

  it("initializes counter PDA", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        user: authority.publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Add amount", async () => {
    let connection = provider.connection;
    let depositAmount = anchor.web3.LAMPORTS_PER_SOL;

    let BeforeUserBalance = await connection.getBalance(authority.publicKey);
    let BeforeVaultAmount = await connection.getBalance(walletPDA);

    const tx = await program.methods
      .deposit(new anchor.BN(depositAmount))
      .accounts({ user: authority.publicKey })
      .rpc();

    let AfterUserBalance = await connection.getBalance(authority.publicKey);
    let AfterVaultAmount = await connection.getBalance(walletPDA);

    expect(BeforeUserBalance).to.be.greaterThan(AfterUserBalance);
    expect(BeforeVaultAmount).to.be.lessThan(AfterVaultAmount);
  });

  it("Withraw amount", async () => {
    let connection = provider.connection;
    let withdrawAmount = anchor.web3.LAMPORTS_PER_SOL;

    let BeforeUser = await connection.getBalance(authority.publicKey);

    const tx = await program.methods
      .withdraw(new anchor.BN(withdrawAmount))
      .accounts({ user: authority.publicKey })
      .rpc();

    let AfterUser = await connection.getBalance(authority.publicKey);

    expect(BeforeUser).to.be.lessThan(AfterUser);
  });

  it("checks", async () => {
    // Is unknow user able to get money from account
    const connection = provider.connection;
    let withdrawAmount = anchor.web3.LAMPORTS_PER_SOL;

    const tx = await program.methods
      .withdraw(new anchor.BN(withdrawAmount))
      .accounts({ user: attacker.publicKey })
      .rpc();

    // withdraw more than balance

    const tx2 = await program.methods
      .withdraw(new anchor.BN(withdrawAmount * 10))
      .accounts({ user: authority.publicKey })
      .rpc();
  });
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { expect } from "chai";

describe("pda counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;
  const user = provider.wallet;

  let counterPda: anchor.web3.PublicKey;

  it("creates a PDA counter", async () => {
    [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods.initialize().rpc();

    const account = await program.account.counter.fetch(counterPda);
    expect(account.count.toNumber()).to.equal(0);
  });

  it("increment", async () => {
    await program.methods.increment().rpc();

    const account = await program.account.counter.fetch(counterPda);
    expect(account.count.toNumber()).to.equal(1);
  })

  it("attack", async () => {
    const attacker = anchor.web3.Keypair.generate();

    try {
      await program.methods.decrement().accounts({user: attacker.publicKey }).signers([attacker]).rpc();
      throw new Error("Should have failed my dear brother");
      
    } catch (error) {
      console.error("Error done")
    }

  })
});

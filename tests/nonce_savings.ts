import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NonceSavings } from "../target/types/nonce_savings";
import { BN } from "bn.js";

describe("nonce_savings", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.NonceSavings as Program<NonceSavings>;
  console.log(program);

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
    const tx = program.methods.initializeSavings("First Savings", new BN(6), new BN(100));
    console.log("Initialiazed Completely",tx)
    console.log(tx.accounts,tx.transaction);
  });
});

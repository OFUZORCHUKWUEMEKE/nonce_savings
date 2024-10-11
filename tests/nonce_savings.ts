import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NonceSavings } from "../target/types/nonce_savings";
import { BN } from "bn.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from '@solana/spl-token';
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js';
import { assert, expect } from "chai";


describe("nonce_savings", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);
  const program = anchor.workspace.NonceSavings as Program<NonceSavings>;

  const user = anchor.web3.Keypair.generate();

  // let vault :anchor.web3.PublicKey;
  let savingsAccount1: anchor.web3.PublicKey;

  let savingsAccount2: anchor.web3.PublicKey;

  let usdc_mint: anchor.web3.PublicKey;

  let user_ata: anchor.web3.PublicKey;

  const wallet = provider.wallet as NodeWallet;
  // usdc_mint = new publicKey()

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block
    })
    return signature;
  }

  function generateShortRandomSeed(): string {
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const length = Math.floor(Math.random() * 11) + 1; // Random length between 1 and 11
    let result = '';
    for (let i = 0; i < length; i++) {
      result += characters.charAt(Math.floor(Math.random() * characters.length));
    }
    return result;
  }

  function toDecimalAmount(rawAmount: bigint, decimals: number = 6): number {
    return Number(rawAmount) / Math.pow(10, decimals);
  }

  it("airdrop sol", async () => {
    const airdrop = await provider.connection.requestAirdrop(user.publicKey, 20 * anchor.web3.LAMPORTS_PER_SOL).then(confirm);
    console.log("\nAirdropped 20 sol to user", airdrop);
    usdc_mint = await createMint(provider.connection, user, user.publicKey, user.publicKey, 6);

    user_ata = (await getOrCreateAssociatedTokenAccount(provider.connection, user, usdc_mint, user.publicKey)).address;

    console.log("user ata is", user_ata.toBase58());

    // Mint 10,000 USDC (accounting for 6 decimals)
    await mintTo(
      provider.connection,
      user,
      usdc_mint,
      user_ata,
      user.publicKey,
      10_000_000_000 // 10,000 USDC with 6 decimals
    );
    const account = await getAccount(provider.connection, user_ata);
    const decimalAmount = toDecimalAmount(account.amount, 6)

    console.log(`account balance of ${user_ata} is ${decimalAmount} `);
  });


  it("Initializing  a SOL savings account", async () => {
    const randomSeed = generateShortRandomSeed();
    const name = "Test Savings";
    const amount = new anchor.BN(1000000); // 1 SOL in lamports
    const duration = new anchor.BN(604800); // 1 week in seconds
    const typeOfSavings = { timeLockedSavings: {} }; // Assuming an enum with fixedTerm as an option
    // const usdPrice = 50.5; // Assuming current SOL price in USD

    savingsAccount1 = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("savings"), user.publicKey.toBuffer(), Buffer.from(randomSeed)],
      program.programId
    )[0];

    await program.methods.initializesol(randomSeed, name, duration, typeOfSavings, null).accountsPartial({
      user: user.publicKey,
      savingsAccount: savingsAccount1,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([user]).rpc();
    const account = await program.account.savingsAccount.fetch(savingsAccount1);
    // console.log(account);
    console.log("sol account number", account.solBalance.toNumber())
    console.log("usdc account number", account.usdcBalance.toNumber())
    console.log("account user", account.user.toString());
    expect(account.randomSeed).to.equal(randomSeed);
    expect(account.name).to.equal(name);
    expect(account.user.toString()).to.equal(user.publicKey.toString());
    expect(account.solBalance.toNumber()).to.equal(0);
    expect(account.usdcBalance.toNumber()).to.equal(0);
  });


  it("Initialize a USDC account", async () => {
    const randomSeed = generateShortRandomSeed();
    const name = "Test USDC Savings";
    const amount = new anchor.BN(1000000); // 1 SOL in lamports
    const duration = new anchor.BN(604800); // 1 week in seconds
    const typeOfSavings = { timeLockedSavings: {} }; // Assuming an enum with fixedTerm as an option

    savingsAccount2 = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("savings"), user.publicKey.toBuffer(), Buffer.from(randomSeed)], program.programId)[0];

    const vault = await getAssociatedTokenAddressSync(usdc_mint, savingsAccount2, true);

    const tx = await program.methods.initializeusdcsavings(randomSeed, name, duration, typeOfSavings, null).accountsPartial({
      user: user.publicKey,
      savingsAccount: savingsAccount2,
      userAta: user_ata,
      vaultAccount: vault,
      usdcMint: usdc_mint,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    }).signers([user]).rpc().then(confirm);

    console.log("vault balance is ", (await provider.connection.getTokenAccountBalance(vault)).value.amount)
  });

  it("Deposit Sol", async () => {
    const savingsAccount = await program.account.savingsAccount.fetch(savingsAccount1);

    const tx = await program.methods.depositSol(new anchor.BN(1000000)).accountsPartial({
      user: user.publicKey,
      savingsAccount: savingsAccount1,
      systemProgram: anchor.web3.SystemProgram.programId
    })

    const newSavingsAccount = await provider.connection.getBalance(savingsAccount1);

    console.log(newSavingsAccount);
    // expect(savingsAccount.solBalance).to.equals(1);

  })
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PredictionMarket } from "../target/types/prediction_market";
import { Keypair, SystemProgram, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";
import { Commitment } from "@solana/web3.js"

//* swithcboard sol/usd devnet price data feed ID = "8g6zZtZFLJCRBm85rZbMws3ce2oqzzDKEGBj9wQGp1kY"

const commitment: Commitment = "confirmed";

describe("capstone-project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;;

  const program = anchor.workspace.PriceBetting as Program<PredictionMarket>;

  // setup admin, betCreator, betTaker keys
  const [ admin, maker, betTaker ] = Array(3).fill(null).map(() => anchor.web3.Keypair.generate());
  console.log("Admin wallet: ", admin.publicKey.toBase58());
  console.log("maker wallet: ", maker.publicKey.toBase58());
  console.log("opponent wallet: ", betTaker.publicKey.toBase58());

  // defining the constants
  const betSeed = new anchor.BN(Date.now());
  const tokenMint = anchor.web3.Keypair.generate().publicKey;
  const makerOdds = new anchor.BN(2);
  const opponentOdds = new anchor.BN(3);
  const pricePrediction = new anchor.BN(1000);
  const deadlineToJoin = new anchor.BN(Date.now() + 3600000); // 1 hour from now
  const startTime = new anchor.BN(Date.now() + 7200000); // 2 hours from now
  const endTime = new anchor.BN(Date.now() + 10800000); // 3 hours from now
  const amount = new anchor.BN(100000000); // 0.1 SOL
  const fees = 100; // 1%

  const [housePda, housePdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("house"), admin.publicKey.toBuffer()],
    program.programId
  );
  const [treasuryPda, treasuryPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury"), housePda.toBuffer()],
    program.programId
  );
  const [betPda, betPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bet"), maker.publicKey.toBuffer()],
    program.programId
  );
  const [vaultPoolPda, vaultPoolPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), betPda.toBuffer()],
    program.programId
  );
  const [userAccountPda, userAccountPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user_profile"), maker.publicKey.toBuffer()],
    program.programId
  );
  
  // Airdrop sol to admin, maker, betTaker
  it("Airdrop some sol", async () => {
    await Promise.all([ admin, maker, betTaker].map(async (k) => {
      return await anchor.getProvider().connection.requestAirdrop(k.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL)
    })).then(confirmTxs);
  });

  it("Initialize the protocol", async () => {
    const tx = await program.methods.initializeProtocol(fees)
      .accountsPartial({
        admin: admin.publicKey,
        house: housePda,
        treasury: treasuryPda,
      })
      .signers([admin])
      .rpc();
    
      console.log("Protocol Init Transaction Signature - ", tx);

      await confirmTx(tx);

      const initializedBetHouse = await program.account.house.fetch(housePda);

      assert.equal(initializedBetHouse.admin.toBase58(), admin.publicKey.toBase58());
      assert.equal(initializedBetHouse.protoclFees, fees);
  })

  // Start creating the bet 
  //TODO: feedInjector should be added to check the real-time data in lib.rs, bet.rs, create_bet.rs and resolve_bet.rs
  it("Create a bet", async () => {
    const makerBalanceBefore = await connection.getBalance(maker.publicKey);

    const tx = await program.methods.createBet(
      betSeed, 
      tokenMint,
      makerOdds,
      opponentOdds,
      pricePrediction,
      deadlineToJoin,
      startTime,
      endTime,
      amount).accountsPartial({
        maker: maker.publicKey,
        bet: betPda,
        vaultPool: vaultPoolPda,
        userAccount: userAccountPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc();

      console.log("Bet creation Transaction Signature - ", tx);

      await confirmTx(tx);

      // Fetch the bet account and assert its data
      const betAccount = await program.account.bet.fetch(betPda);
      const vaultBalance = await connection.getBalance(vaultPoolPda);
      const makerBalanceAfter = await connection.getBalance(maker.publicKey);

      assert.equal(vaultBalance, amount.toNumber());
      assert.isAtMost(makerBalanceAfter, makerBalanceBefore - amount.toNumber());

      assert.ok(betAccount.maker.equals(maker.publicKey));
      assert.ok(betAccount.tokenMint.equals(tokenMint));
      assert.ok(betAccount.odds.makerOdds.eq(makerOdds));
      assert.ok(betAccount.odds.opponentOdds.eq(opponentOdds));
      assert.ok(betAccount.pricePrediction.eq(pricePrediction));
      assert.ok(betAccount.deadlineToJoin.eq(deadlineToJoin));
      assert.ok(betAccount.startTime.eq(startTime));
      assert.ok(betAccount.endTime.eq(endTime));
      assert.ok(betAccount.makerDeposit.eq(amount));
      assert.equal(betAccount.status, { findingOpponent: {} });
  })

  // Cancelling the created bet for testing 
  it("Cancel the bet", async () => {
    const vaultBalanceBefore = await connection.getBalance(vaultPoolPda);
    const makerBalanceBefore = await connection.getBalance(maker.publicKey);

    // Fetch the bet account before cancellation
    const betAccountBefore = await program.account.bet.fetch(betPda);
    assert.equal(betAccountBefore.status.findingOpponent, {}, "Bet status should be 'findingOpponent' before cancellation");

    const tx = await program.methods.cancelBet(betSeed)
      .accountsPartial({
        maker: maker.publicKey,
        bet: betPda,
        vaultPool: vaultPoolPda,
        userAccount: userAccountPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc();
    
      console.log("Canceling bet Transaction Signature - ", tx);

      await confirmTx(tx);
      // Fetch the bet account after cancellation
      const betAccountAfter = await program.account.bet.fetch(betPda);
    
      // Check if the bet status is updated or not
      assert.equal(betAccountAfter.status.completed, {}, "Bet status should be 'completed' after cancellation");

      // Check if the funds have been returned to the maker
      const vaultBalanceAfter = await connection.getBalance(vaultPoolPda);
      const makerBalanceAfter = await connection.getBalance(maker.publicKey);
      assert.equal(vaultBalanceAfter, 0);
      assert.equal(makerBalanceAfter, 100 * anchor.web3.LAMPORTS_PER_SOL);
  })

  // Creating another bet after cancellation of the first bet
  //TODO: feedInjector should be added to check the real-time data in lib.rs, bet.rs, create_bet.rs and resolve_bet.rs
  it("Create a bet", async () => {
    const makerBalanceBefore = await connection.getBalance(maker.publicKey);

    const tx = await program.methods.createBet(
      betSeed, 
      tokenMint,
      makerOdds,
      opponentOdds,
      pricePrediction,
      deadlineToJoin,
      startTime,
      endTime,
      amount).accountsPartial({
        maker: maker.publicKey,
        bet: betPda,
        vaultPool: vaultPoolPda,
        userAccount: userAccountPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc();

      console.log("Bet creation Transaction Signature - ", tx);

      await confirmTx(tx);

      // Fetch the bet account and assert its data
      const betAccount = await program.account.bet.fetch(betPda);
      const vaultBalance = await connection.getBalance(vaultPoolPda);
      const makerBalanceAfter = await connection.getBalance(maker.publicKey);

      assert.equal(vaultBalance, amount.toNumber());
      assert.isAtMost(makerBalanceAfter, makerBalanceBefore - amount.toNumber());

      assert.ok(betAccount.maker.equals(maker.publicKey));
      assert.ok(betAccount.tokenMint.equals(tokenMint));
      assert.ok(betAccount.odds.makerOdds.eq(makerOdds));
      assert.ok(betAccount.odds.opponentOdds.eq(opponentOdds));
      assert.ok(betAccount.pricePrediction.eq(pricePrediction));
      assert.ok(betAccount.deadlineToJoin.eq(deadlineToJoin));
      assert.ok(betAccount.startTime.eq(startTime));
      assert.ok(betAccount.endTime.eq(endTime));
      assert.ok(betAccount.makerDeposit.eq(amount));
      assert.equal(betAccount.status, { findingOpponent: {} });
  })
  
  // Accepting the second created bet after cancelling the first bet
  it("Accepting the bet", async () => {
    const takerBalanceBefore = await connection.getBalance(betTaker.publicKey);
    const vaultBalanceBefore = await connection.getBalance(vaultPoolPda);

    const tx = await program.methods.acceptBet(betSeed).accountsPartial({
      opponent: betTaker.publicKey,
      maker: maker.publicKey,
      bet: betPda,
      vaultPool: vaultPoolPda,
      userAccount: userAccountPda,
      systemProgram: SystemProgram.programId, 
    })
    .signers([betTaker])
    .rpc();

    console.log("Accepting bet Transaction Signature - ", tx);

    await confirmTx(tx);

    const vaultBalanceAfter = await connection.getBalance(vaultPoolPda);
    const takerBalanceAfter = await connection.getBalance(betTaker.publicKey);
    const treasuryBalanceAfter = await connection.getBalance(treasuryPda);
    const betAccount = await program.account.bet.fetch(betPda);
    
    assert.equal(betAccount.opponent.toBase58(), betTaker.publicKey.toBase58());
    assert.equal(betAccount.status.waitingToStart != undefined, true);

    const totalBetAmount = amount.toNumber() * 2;
    const feesAmount = totalBetAmount * (fees / 10000);
    assert.equal(vaultBalanceAfter, vaultBalanceBefore + amount.toNumber() - feesAmount);
    assert.isAtMost(takerBalanceAfter, takerBalanceBefore - amount.toNumber());
    assert.equal(treasuryBalanceAfter, feesAmount);
  })

  // Simulate time passing
  it("Simulate time passing", async () => {
    await new Promise(resolve => setTimeout(resolve, 5000)); // Wait for 5 seconds
  });
  //TODO: feedInjector should be added to check the real-time data in lib.rs, bet.rs, create_bet.rs and resolve_bet.rs
  // Checking the winner after some time has passed
  it("Checking the winner", async () => {
  const tx = await program.methods.checkWinner(betSeed)
    .accountsPartial({
      signer: admin.publicKey,
      maker: maker.publicKey,
      opponent: betTaker.publicKey,
      bet: betPda,
    })
    .signers([admin])
    .rpc();
    const betAccount = await program.account.bet.fetch(betPda);
  })
});

// Helpers
const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    commitment
  )
}
const confirmTxs = async (signatures: string[]) => {
  await Promise.all(signatures.map(confirmTx))
}

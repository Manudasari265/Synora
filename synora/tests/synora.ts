import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PredictionMarket } from "../target/types/prediction_market";
import { Keypair, SystemProgram, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";

//* swithcboard sol/usd devnet price data feed ID = "8g6zZtZFLJCRBm85rZbMws3ce2oqzzDKEGBj9wQGp1kY"

const commitment: any = "confirmed";

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

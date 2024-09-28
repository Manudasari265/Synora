import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PredictionMarket } from "../target/types/prediction_market";
import { Keypair, SystemProgram, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";

//* swithcboard sol/usd devnet price data feed ID = "8g6zZtZFLJCRBm85rZbMws3ce2oqzzDKEGBj9wQGp1kY"

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
  
});

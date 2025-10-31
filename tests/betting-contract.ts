import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BettingContract } from "../target/types/betting_contract";
import { expect } from "chai";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";
import * as fs from "fs";
import * as path from "path";

describe("betting-contract", () => {
  // Configure the client
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.BettingContract as Program<BettingContract>;
  const provider = anchor.AnchorProvider.env();

  // Bank account - This will be your pre-funded account that distributes funds
  let bankKeypair: anchor.web3.Keypair;

  // Test accounts
  let admin: anchor.web3.Keypair;
  let moderator: anchor.web3.Keypair;
  let user1: anchor.web3.Keypair;
  let user2: anchor.web3.Keypair;
  let user3: anchor.web3.Keypair;

  // Test constants
  const STREAM_ID = "test-stream-" + Date.now();
  const BET_AMOUNT_1_SOL = new anchor.BN(LAMPORTS_PER_SOL);
  const BET_AMOUNT_2_SOL = new anchor.BN(2 * LAMPORTS_PER_SOL);
  const BET_AMOUNT_HALF_SOL = new anchor.BN(0.5 * LAMPORTS_PER_SOL);

  // PDAs
  let bettingPoolPda: PublicKey;
  let user1BetPda: PublicKey;
  let user2BetPda: PublicKey;
  let user3BetPda: PublicKey;

  /**
   * Transfer SOL from bank account to a recipient
   */
  async function transferFromBank(to: PublicKey, amount: number): Promise<void> {
    try {
      const tx = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: bankKeypair.publicKey,
          toPubkey: to,
          lamports: amount,
        })
      );
      
      await provider.sendAndConfirm(tx, [bankKeypair]);
      console.log(`✅ Transferred ${amount / LAMPORTS_PER_SOL} SOL to ${to.toBase58()}`);
    } catch (error) {
      console.error(`❌ Failed to transfer to ${to.toBase58()}:`, error.message);
      throw error;
    }
  }

  /**
   * Load or create the bank keypair
   */
  function loadBankKeypair(): anchor.web3.Keypair {
    const bankPath = path.join(__dirname, 'keypairs', 'bank.json');
    
    try {
      // Try to load existing bank keypair
      const bankJson = JSON.parse(fs.readFileSync(bankPath, 'utf-8'));
      const keypair = anchor.web3.Keypair.fromSecretKey(Buffer.from(bankJson));
      console.log("📦 Loaded existing bank keypair");
      return keypair;
    } catch (error) {
      // If doesn't exist, create a new one
      console.log("📦 Creating new bank keypair...");
      const keypair = anchor.web3.Keypair.generate();
      
      // Create directory if it doesn't exist
      const dir = path.dirname(bankPath);
      if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
      }
      
      // Save the keypair
      fs.writeFileSync(
        bankPath,
        JSON.stringify(Array.from(keypair.secretKey))
      );
      
      console.log("💾 Saved bank keypair to:", bankPath);
      console.log("🏦 Bank address:", keypair.publicKey.toBase58());
      console.log("\n⚠️  IMPORTANT: Fund this bank account first!");
      console.log(`Run: solana airdrop 10 ${keypair.publicKey.toBase58()} --url devnet\n`);
      
      return keypair;
    }
  }

  before(async () => {
    // Load the bank account
    bankKeypair = loadBankKeypair();
    
    // Check bank balance
    const bankBalance = await provider.connection.getBalance(bankKeypair.publicKey);
    console.log(`🏦 Bank balance: ${bankBalance / LAMPORTS_PER_SOL} SOL`);
    
    if (bankBalance < 10 * LAMPORTS_PER_SOL) {
      console.error("\n❌ ERROR: Bank account has insufficient funds!");
      console.error(`Current balance: ${bankBalance / LAMPORTS_PER_SOL} SOL`);
      console.error(`Required: At least 10 SOL`);
      console.error(`\nFund the bank account:`);
      console.error(`solana airdrop 10 ${bankKeypair.publicKey.toBase58()} --url devnet`);
      console.error(`\nOr transfer from your wallet:`);
      console.error(`solana transfer ${bankKeypair.publicKey.toBase58()} 10 --url devnet\n`);
      throw new Error("Insufficient bank funds");
    }

    // Generate test accounts
    admin = anchor.web3.Keypair.generate();
    moderator = anchor.web3.Keypair.generate();
    user1 = anchor.web3.Keypair.generate();
    user2 = anchor.web3.Keypair.generate();
    user3 = anchor.web3.Keypair.generate();

    console.log("\n=== Generated Test Accounts ===");
    console.log("Admin:", admin.publicKey.toBase58());
    console.log("Moderator:", moderator.publicKey.toBase58());
    console.log("User1:", user1.publicKey.toBase58());
    console.log("User2:", user2.publicKey.toBase58());
    console.log("User3:", user3.publicKey.toBase58());
    console.log("================================\n");

    // Fund all test accounts from bank
    console.log("💸 Funding test accounts from bank...");
    await transferFromBank(admin.publicKey, 3 * LAMPORTS_PER_SOL);
    await transferFromBank(moderator.publicKey, 1 * LAMPORTS_PER_SOL);
    await transferFromBank(user1.publicKey, 2 * LAMPORTS_PER_SOL);
    await transferFromBank(user2.publicKey, 3 * LAMPORTS_PER_SOL);
    await transferFromBank(user3.publicKey, 2 * LAMPORTS_PER_SOL);

    // Wait a bit for transactions to confirm
    await new Promise(resolve => setTimeout(resolve, 2000));

    // Verify balances
    const adminBalance = await provider.connection.getBalance(admin.publicKey);
    const user1Balance = await provider.connection.getBalance(user1.publicKey);
    console.log(`✅ Admin funded: ${adminBalance / LAMPORTS_PER_SOL} SOL`);
    console.log(`✅ User1 funded: ${user1Balance / LAMPORTS_PER_SOL} SOL\n`);

    // Derive PDAs
    [bettingPoolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("betting_pool"), Buffer.from(STREAM_ID)],
      program.programId
    );

    [user1BetPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("bet"),
        bettingPoolPda.toBuffer(),
        user1.publicKey.toBuffer(),
        Buffer.from([0, 0, 0, 0])
      ],
      program.programId
    );

    [user2BetPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("bet"),
        bettingPoolPda.toBuffer(),
        user2.publicKey.toBuffer(),
        Buffer.from([1, 0, 0, 0])
      ],
      program.programId
    );

    [user3BetPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("bet"),
        bettingPoolPda.toBuffer(),
        user3.publicKey.toBuffer(),
        Buffer.from([2, 0, 0, 0])
      ],
      program.programId
    );
  });

  describe("Initialize Betting Pool", () => {
    it("Creates a new betting pool successfully", async () => {
      // Set betting deadline to 1 hour from now
      const bettingDeadline = Math.floor(Date.now() / 1000) + (60 * 60);
      
      const tx = await program.methods
        .initialize(STREAM_ID, new anchor.BN(bettingDeadline), moderator.publicKey)
        .accountsPartial({
          bettingPool: bettingPoolPda,
          admin: admin.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      console.log("Initialize transaction signature:", tx);

      const bettingPool = await program.account.bettingPool.fetch(bettingPoolPda);
      expect(bettingPool.admin.toString()).to.equal(admin.publicKey.toString());
      expect(bettingPool.moderator.toString()).to.equal(moderator.publicKey.toString());
      expect(bettingPool.streamId).to.equal(STREAM_ID);
      expect(bettingPool.totalPool.toNumber()).to.equal(0);
      expect(bettingPool.player1Bets.toNumber()).to.equal(0);
      expect(bettingPool.player2Bets.toNumber()).to.equal(0);
      expect(bettingPool.winnerDeclared).to.be.false;
      expect(bettingPool.creatorFeeRate).to.equal(250); // 2.5%
      expect(bettingPool.platformFeeRate).to.equal(250); // 2.5%
      expect(bettingPool.isPayoutComplete).to.be.false;
    });

    it("Fails to create pool with non-admin", async () => {
      // Create a pool with user1 as admin
      const streamId2 = "test-stream-456";
      const bettingDeadline2 = Math.floor(Date.now() / 1000) + (60 * 60);
      const [poolPda2] = PublicKey.findProgramAddressSync(
        [Buffer.from("betting_pool"), Buffer.from(streamId2)],
        program.programId
      );

      // User1 creates a pool (this should succeed now)
      await program.methods
        .initialize(streamId2, new anchor.BN(bettingDeadline2), moderator.publicKey)
        .accountsPartial({
          bettingPool: poolPda2,
          admin: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      // Now try to declare winner with a different user (should fail)
      try {
        await program.methods
          .declareWinner(1)
          .accountsPartial({
            bettingPool: poolPda2,
            signer: user2.publicKey, // Wrong signer
          })
          .signers([user2])
          .rpc();
        expect.fail("Should have failed with unauthorized admin");
      } catch (error) {
        expect(error.message).to.include("UnauthorizedAdmin");
      }
    });
  });

  describe("Place Bets", () => {
    it("User1 places bet on Player 1", async () => {
      const initialBalance = await provider.connection.getBalance(user1.publicKey);

      const tx = await program.methods
        .placeBet(1, BET_AMOUNT_1_SOL)
        .accountsPartial({
          bettingPool: bettingPoolPda,
          bet: user1BetPda,
          user: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      console.log("User1 bet transaction:", tx);

      const bet = await program.account.bet.fetch(user1BetPda);
      expect(bet.user.toString()).to.equal(user1.publicKey.toString());
      expect(bet.amount.toNumber()).to.equal(LAMPORTS_PER_SOL);
      expect(bet.prediction).to.equal(1);
      expect(bet.isPaidOut).to.be.false;

      const bettingPool = await program.account.bettingPool.fetch(bettingPoolPda);
      expect(bettingPool.totalPool.toNumber()).to.equal(LAMPORTS_PER_SOL);
      expect(bettingPool.player1Bets.toNumber()).to.equal(LAMPORTS_PER_SOL);
      expect(bettingPool.player1BetCount).to.equal(1);

      const finalBalance = await provider.connection.getBalance(user1.publicKey);
      expect(finalBalance).to.be.lessThan(initialBalance - LAMPORTS_PER_SOL);
    });

    it("User2 places bet on Player 2", async () => {
      const tx = await program.methods
        .placeBet(2, BET_AMOUNT_2_SOL)
        .accountsPartial({
          bettingPool: bettingPoolPda,
          bet: user2BetPda,
          user: user2.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user2])
        .rpc();

      console.log("User2 bet transaction:", tx);

      const bettingPool = await program.account.bettingPool.fetch(bettingPoolPda);
      expect(bettingPool.totalPool.toNumber()).to.equal(3 * LAMPORTS_PER_SOL);
      expect(bettingPool.player1Bets.toNumber()).to.equal(LAMPORTS_PER_SOL);
      expect(bettingPool.player2Bets.toNumber()).to.equal(2 * LAMPORTS_PER_SOL);
      expect(bettingPool.player2BetCount).to.equal(1);
    });

    it("User3 places bet on Player 1", async () => {
      const tx = await program.methods
        .placeBet(1, BET_AMOUNT_HALF_SOL)
        .accountsPartial({
          bettingPool: bettingPoolPda,
          bet: user3BetPda,
          user: user3.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user3])
        .rpc();

      console.log("User3 bet transaction:", tx);

      const bettingPool = await program.account.bettingPool.fetch(bettingPoolPda);
      expect(bettingPool.totalPool.toNumber()).to.equal(3.5 * LAMPORTS_PER_SOL);
      expect(bettingPool.player1Bets.toNumber()).to.equal(1.5 * LAMPORTS_PER_SOL);
      expect(bettingPool.player2Bets.toNumber()).to.equal(2 * LAMPORTS_PER_SOL);
      expect(bettingPool.player1BetCount).to.equal(2);
      expect(bettingPool.player2BetCount).to.equal(1);
    });

    it("Fails to place bet with invalid prediction", async () => {
      const bettingPool = await program.account.bettingPool.fetch(bettingPoolPda);
      const currentBetCount = bettingPool.player1BetCount + bettingPool.player2BetCount;
      
      const [invalidBetPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("bet"),
          bettingPoolPda.toBuffer(),
          user1.publicKey.toBuffer(),
          Buffer.from(new anchor.BN(currentBetCount).toArrayLike(Buffer, 'le', 4))
        ],
        program.programId
      );

      try {
        await program.methods
          .placeBet(3, BET_AMOUNT_1_SOL)
          .accountsPartial({
            bettingPool: bettingPoolPda,
            bet: invalidBetPda,
            user: user1.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([user1])
          .rpc();
        expect.fail("Should have failed with invalid prediction");
      } catch (error) {
        expect(error.message).to.include("InvalidPrediction");
      }
    });
  });

  describe("Declare Winner", () => {
    it("Admin declares Player 1 as winner", async () => {
      const tx = await program.methods
        .declareWinner(1)
        .accountsPartial({
          bettingPool: bettingPoolPda,
          signer: admin.publicKey,
        })
        .signers([admin])
        .rpc();

      console.log("Declare winner transaction (admin):", tx);

      const bettingPool = await program.account.bettingPool.fetch(bettingPoolPda);
      expect(bettingPool.winnerDeclared).to.be.true;
      expect(bettingPool.winningOutcome).to.equal(1);
    });

    it("Moderator can declare winner (should fail if already declared)", async () => {
      try {
        await program.methods
          .declareWinner(2)
          .accountsPartial({
            bettingPool: bettingPoolPda,
            signer: moderator.publicKey,
          })
          .signers([moderator])
          .rpc();
        expect.fail("Should have failed with winner already declared");
      } catch (error) {
        expect(error.message).to.include("WinnerAlreadyDeclared");
      }
    });

    it("Fails to declare winner twice", async () => {
      try {
        await program.methods
          .declareWinner(2)
          .accountsPartial({
            bettingPool: bettingPoolPda,
            signer: admin.publicKey,
          })
          .signers([admin])
          .rpc();
        expect.fail("Should have failed with winner already declared");
      } catch (error) {
        expect(error.message).to.include("WinnerAlreadyDeclared");
      }
    });
  });

  describe("Payout Winners", () => {
    it("Pays out User1 (winner with 1 SOL bet)", async () => {
      const initialBalance = await provider.connection.getBalance(user1.publicKey);
      const initialPoolBalance = await provider.connection.getBalance(bettingPoolPda);

      const tx = await program.methods
        .payoutWinners()
        .accountsPartial({
          bettingPool: bettingPoolPda,
          bet: user1BetPda,
          winner: user1.publicKey,
          platformTreasury: admin.publicKey,
          payer: admin.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      console.log("User1 payout transaction:", tx);

      const bet = await program.account.bet.fetch(user1BetPda);
      expect(bet.isPaidOut).to.be.true;

      const finalBalance = await provider.connection.getBalance(user1.publicKey);
      const finalPoolBalance = await provider.connection.getBalance(bettingPoolPda);

      expect(finalBalance).to.be.greaterThan(initialBalance);
      expect(finalPoolBalance).to.be.lessThan(initialPoolBalance);
    });

    it("Pays out User3 (winner with 0.5 SOL bet)", async () => {
      const initialBalance = await provider.connection.getBalance(user3.publicKey);

      const tx = await program.methods
        .payoutWinners()
        .accountsPartial({
          bettingPool: bettingPoolPda,
          bet: user3BetPda,
          winner: user3.publicKey,
          platformTreasury: admin.publicKey,
          payer: admin.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([admin])
        .rpc();

      console.log("User3 payout transaction:", tx);

      const bet = await program.account.bet.fetch(user3BetPda);
      expect(bet.isPaidOut).to.be.true;

      const finalBalance = await provider.connection.getBalance(user3.publicKey);
      expect(finalBalance).to.be.greaterThan(initialBalance);
    });

    it("Fails to payout losing bet (User2)", async () => {
      try {
        await program.methods
          .payoutWinners()
          .accountsPartial({
            bettingPool: bettingPoolPda,
            bet: user2BetPda,
            winner: user2.publicKey,
            platformTreasury: admin.publicKey,
            payer: admin.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([admin])
          .rpc();
        expect.fail("Should have failed - bet is not a winner");
      } catch (error) {
        expect(error.message).to.include("BetNotWinner");
      }
    });

    it("Fails to payout same bet twice", async () => {
      try {
        await program.methods
          .payoutWinners()
          .accountsPartial({
            bettingPool: bettingPoolPda,
            bet: user1BetPda,
            winner: user1.publicKey,
            platformTreasury: admin.publicKey,
            payer: admin.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([admin])
          .rpc();
        expect.fail("Should have failed - bet already paid out");
      } catch (error) {
        expect(error.message).to.include("BetAlreadyPaidOut");
      }
    });
  });

  describe("Edge Cases", () => {
    it("Cannot place bet after winner declared", async () => {
      const bettingPool = await program.account.bettingPool.fetch(bettingPoolPda);
      const currentBetCount = bettingPool.player1BetCount + bettingPool.player2BetCount;
      
      const [lateBetPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("bet"),
          bettingPoolPda.toBuffer(),
          user1.publicKey.toBuffer(),
          Buffer.from(new anchor.BN(currentBetCount).toArrayLike(Buffer, 'le', 4))
        ],
        program.programId
      );

      try {
        await program.methods
          .placeBet(1, BET_AMOUNT_1_SOL)
          .accountsPartial({
            bettingPool: bettingPoolPda,
            bet: lateBetPda,
            user: user1.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([user1])
          .rpc();
        expect.fail("Should have failed - betting is closed");
      } catch (error) {
        expect(error.message).to.include("BettingClosed");
      }
    });
  });

  describe("Pool Statistics", () => {
    it("Verifies final pool statistics", async () => {
      const bettingPool = await program.account.bettingPool.fetch(bettingPoolPda);
      
      console.log("\n=== Final Pool Statistics ===");
      console.log("Stream ID:", bettingPool.streamId);
      console.log("Total Pool:", bettingPool.totalPool.toNumber() / LAMPORTS_PER_SOL, "SOL");
      console.log("Player 1 Bets:", bettingPool.player1Bets.toNumber() / LAMPORTS_PER_SOL, "SOL");
      console.log("Player 2 Bets:", bettingPool.player2Bets.toNumber() / LAMPORTS_PER_SOL, "SOL");
      console.log("Player 1 Bet Count:", bettingPool.player1BetCount);
      console.log("Player 2 Bet Count:", bettingPool.player2BetCount);
      console.log("Winner Declared:", bettingPool.winnerDeclared);
      console.log("Winning Outcome: Player", bettingPool.winningOutcome);
      console.log("Creator Fee Rate:", bettingPool.creatorFeeRate / 100, "%");
      console.log("Platform Fee Rate:", bettingPool.platformFeeRate / 100, "%");
      console.log("=============================\n");

      expect(bettingPool.totalPool.toNumber()).to.equal(3.5 * LAMPORTS_PER_SOL);
      expect(bettingPool.player1Bets.toNumber()).to.equal(1.5 * LAMPORTS_PER_SOL);
      expect(bettingPool.player2Bets.toNumber()).to.equal(2 * LAMPORTS_PER_SOL);
      expect(bettingPool.player1BetCount).to.equal(2);
      expect(bettingPool.player2BetCount).to.equal(1);
      expect(bettingPool.winnerDeclared).to.be.true;
      expect(bettingPool.winningOutcome).to.equal(1);
    });
  });
});

# 🎮 Solana Betting Contract - Complete Guide

## 📖 What Is This Project?

This is a **smart contract** (a program that runs on the blockchain) that lets people bet on gaming streams. Think of it like a betting pool where:
- People watch a game stream (like two players competing)
- They bet SOL (Solana cryptocurrency) on who will win
- When the game ends, winners get paid from the pool
- A small fee (5%) goes to the platform

---

## 🏗️ Project Structure (What Each Folder Does)

```
betting-contract/
│
├── programs/                          # The smart contract code (Rust)
│   └── betting-contract/
│       └── src/
│           ├── lib.rs                 # Main entry point - lists all functions
│           ├── error.rs               # Error messages (like "Betting is closed")
│           ├── constants.rs           # Fixed numbers (like 5% fee)
│           │
│           ├── state/                 # Data structures (how data is stored)
│           │   ├── betting_pool.rs    # Pool info (total bets, winners, etc.)
│           │   └── bet.rs             # Individual bet info (who bet, how much)
│           │
│           └── instructions/          # Actions people can do
│               ├── initialize.rs      # Create a new betting pool
│               ├── place_bet.rs       # Place a bet on Player 1 or 2
│               ├── declare_winner.rs  # Admin declares who won
│               └── payout_winners.rs  # Pay winners their prize
│
├── tests/                             # Test code (TypeScript)
│   └── betting-contract.ts            # Tests to make sure everything works
│
├── target/                            # Compiled code (generated automatically)
│   ├── deploy/                        # Deployed program files
│   └── idl/                           # Interface definition (for frontend)
│       └── betting_contract.json      # Describes all functions (IMPORTANT!)
│
├── Anchor.toml                        # Project settings
├── package.json                       # JavaScript dependencies
└── Cargo.toml                         # Rust dependencies
```

---

## 🔧 How Does The Smart Contract Work?

### 1️⃣ **Initialize** - Create a Betting Pool
**What it does:** Admin creates a new betting pool for a specific stream.

**Example:**
```
Stream: "Player1 vs Player2 - Finals"
Admin: Your wallet address
Deadline: 1 hour from now
```

**What happens:**
- A new betting pool account is created on Solana
- It stores: stream ID, admin address, betting deadline, fee rate (5%)
- It's like creating a "pot" where people can throw their money

---

### 2️⃣ **Place Bet** - Users Bet on Winner
**What it does:** Users bet SOL on who they think will win.

**Example:**
```
User1 bets: 1 SOL on Player 1
User2 bets: 2 SOL on Player 2
User3 bets: 0.5 SOL on Player 1
```

**What happens:**
- User's SOL is transferred to the betting pool
- A "Bet" account is created with:
  - Who placed it (user's wallet)
  - How much (amount in SOL)
  - Their prediction (1 or 2)
  - When it was placed (timestamp)
  - A unique bet number (index)

**Important Rules:**
- Can only bet 1 or 2 (Player 1 or Player 2)
- Must bet before deadline
- Cannot bet after winner is declared
- Cannot bet 0 SOL

--- 

### 3️⃣ **Declare Winner** - Admin Announces Result
**What it does:** After the game ends, admin declares who won.

**Example:**
```
Admin declares: Player 1 wins!
```

**What happens:**
- The betting pool is marked as "winner declared"
- The winning outcome (1 or 2) is stored
- No more bets can be placed
- Winners can now claim their prizes

**Important Rules:**
- Only the admin can do this
- Can only declare winner once
- Must be 1 or 2 (Player 1 or Player 2)

---

### 4️⃣ **Payout Winners** - Winners Claim Prize
**What it does:** Winners claim their share of the pool.

**Example:**
```
Total Pool: 3.5 SOL
Platform Fee (5%): 0.175 SOL
Prize Pool: 3.325 SOL

Player 1 Bets: 1.5 SOL (User1 + User3)
Player 2 Bets: 2 SOL (User2)

Player 1 wins!

User1 gets: (1 SOL / 1.5 SOL) × 3.325 = 2.216 SOL
User3 gets: (0.5 SOL / 1.5 SOL) × 3.325 = 1.108 SOL
User2 gets: 0 SOL (lost)
```

**Formula:**
```
Your Prize = (Your Bet / Total Winning Side Bets) × Prize Pool
```

**What happens:**
- Program calculates winner's share
- SOL is transferred from pool to winner's wallet
- Bet is marked as "paid out" (prevents double claims)

**Important Rules:**
- Only winners can claim
- Each bet can only be paid once
- Must have declared winner first

---

## 🧪 What Are Tests? (VERY IMPORTANT!)

### **Tests are NOT the actual app!**

Tests are like **practice runs** to make sure the contract works correctly BEFORE real people use it.

### What Tests Do:
1. **Create fake wallets** with fake SOL (not real money)
2. **Run all functions** to see if they work
3. **Check for errors** to catch bugs
4. **Verify results** to ensure math is correct

### Example Test Flow:
```
1. Create fake admin wallet ✅
2. Create fake user wallets (user1, user2, user3) ✅
3. Give them fake SOL (10 SOL each) ✅
4. Admin creates betting pool ✅
5. Users place bets ✅
6. Admin declares winner ✅
7. Winners get paid ✅
8. Losers cannot get paid ✅
9. Cannot bet after winner declared ✅
```

### **Tests are STATIC (Fixed/Hardcoded)**

In the test file, everything is hardcoded:
```typescript
const STREAM_ID = "test-stream-123";  // ← Fixed value
const BET_AMOUNT_1_SOL = 1 SOL;       // ← Fixed value
const admin = Keypair.generate();      // ← Fake wallet
```

This is ONLY for testing. It's not connected to:
- Real users
- Real money
- Your frontend
- Live streams

---

## 🌐 How To Make It DYNAMIC (Real Frontend Integration)

To use this contract in your real frontend app, you need to:

### **Step 1: Deploy the Contract**
```bash
# Deploy to devnet (test network)
anchor deploy --provider.cluster devnet

# Deploy to mainnet (real network with real SOL)
anchor deploy --provider.cluster mainnet
```

You'll get a **Program ID** - this is your contract's address on Solana.

---

### **Step 2: Connect Your Frontend**

In your frontend (React, Next.js, etc.), you need:

#### **A. Install Solana Wallet Adapter**
```bash
npm install @solana/wallet-adapter-react @solana/wallet-adapter-wallets
npm install @coral-xyz/anchor @solana/web3.js
```

#### **B. Load the Contract IDL**
The IDL (Interface Definition Language) file is in `target/idl/betting_contract.json`. This tells your frontend what functions exist.

```typescript
import idl from './betting_contract.json';
import { Program, AnchorProvider } from '@coral-xyz/anchor';

// Connect to wallet
const provider = new AnchorProvider(connection, wallet, {});
const program = new Program(idl, PROGRAM_ID, provider);
```

#### **C. Make Functions Dynamic**

Instead of hardcoded test values, use real data:

**Example: Create Betting Pool**
```typescript
// STATIC (Test)
const STREAM_ID = "test-stream-123";

// DYNAMIC (Frontend)
const STREAM_ID = streamData.id;  // From your database
const deadline = Date.now() + (60 * 60 * 1000);  // 1 hour from now

await program.methods
  .initialize(STREAM_ID)
  .accounts({
    bettingPool: bettingPoolPda,
    admin: wallet.publicKey,  // Real user's wallet
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

**Example: Place Bet**
```typescript
// DYNAMIC (Frontend)
const betAmount = userInput.amount * LAMPORTS_PER_SOL;  // From form
const prediction = userInput.playerChoice;  // 1 or 2

await program.methods
  .placeBet(prediction, betAmount)
  .accounts({
    bettingPool: bettingPoolPda,
    bet: betPda,
    user: wallet.publicKey,  // Real user's wallet
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

---

### **Step 3: Calculate PDAs (Program Derived Addresses)**

PDAs are special addresses created by the program. You need to calculate them correctly:

```typescript
// Betting Pool PDA
const [bettingPoolPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("betting_pool"),
    Buffer.from(streamId)  // Your stream ID
  ],
  program.programId
);

// Bet PDA (for each user)
const [betPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("bet"),
    bettingPoolPda.toBuffer(),
    wallet.publicKey.toBuffer(),
    Buffer.from(betIndex.toArrayLike(Buffer, 'le', 4))  // Current bet count
  ],
  program.programId
);
```

**Important:** You must fetch the current `betIndex` from the pool:
```typescript
const pool = await program.account.bettingPool.fetch(bettingPoolPda);
const betIndex = pool.player1BetCount + pool.player2BetCount;
```

---

### **Step 4: Real-Time Updates**

To show live data in your frontend:

```typescript
// Fetch betting pool data
const pool = await program.account.bettingPool.fetch(bettingPoolPda);

console.log("Total Pool:", pool.totalPool / LAMPORTS_PER_SOL, "SOL");
console.log("Player 1 Bets:", pool.player1Bets / LAMPORTS_PER_SOL, "SOL");
console.log("Player 2 Bets:", pool.player2Bets / LAMPORTS_PER_SOL, "SOL");
console.log("Winner Declared:", pool.winnerDeclared);

// Fetch user's bet
const bet = await program.account.bet.fetch(betPda);
console.log("Your Bet:", bet.amount / LAMPORTS_PER_SOL, "SOL");
console.log("Your Prediction:", bet.prediction === 1 ? "Player 1" : "Player 2");
console.log("Paid Out:", bet.isPaidOut);
```

---

## 🔐 Security & Important Notes

### **1. Admin Control**
- The admin who creates the pool has power to declare winners
- Make sure admin wallet is secure
- Consider multi-sig or DAO governance for production

### **2. Deadline**
- Bets can only be placed before `betting_deadline`
- Set deadline carefully (usually right before game starts)

### **3. Platform Fee**
- Currently 5% (500 basis points)
- Stored in the pool (admin can withdraw later)

### **4. Cannot Change After Deploy**
- Once deployed, the contract code cannot be changed
- Test thoroughly on devnet first!

### **5. PDA Calculations**
- Must calculate PDAs correctly in frontend
- Wrong PDA = transaction fails
- Always fetch current bet count from pool

---

## 🚀 Quick Start Commands

```bash
# Install dependencies
yarn install

# Build the contract
anchor build

# Run tests (local)
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet (CAREFUL!)
anchor deploy --provider.cluster mainnet
```

---

## 📊 Data Flow Diagram

```
USER FLOW:
1. Admin creates pool → BettingPool account created
2. User1 bets 1 SOL on Player 1 → Bet account created + SOL transferred
3. User2 bets 2 SOL on Player 2 → Bet account created + SOL transferred
4. Admin declares Player 1 wins → Pool marked as "winner declared"
5. User1 claims payout → Calculate prize → Transfer SOL to User1
6. User2 tries to claim → Error: "BetNotWinner"
```

---

## ❓ Common Questions

### **Q: Can I test without real SOL?**
Yes! Use devnet (test network) which has free test SOL.

### **Q: How do users connect their wallets?**
Use Solana wallet adapters like Phantom, Solflare in your frontend.

### **Q: What if admin declares wrong winner?**
Cannot undo! Admin must be trusted or use governance.

### **Q: Can users bet after deadline?**
No, the contract checks `betting_deadline` timestamp.

### **Q: What happens to platform fee?**
It stays in the pool. Admin can add a function to withdraw fees.

### **Q: Can same user bet multiple times?**
Yes! Each bet creates a new Bet account with different index.

---

## 🛠️ Customization Ideas

You can modify this contract to add:
- Multiple outcome options (not just 2 players)
- Different fee structures
- Time-based auto-declaration
- Refund mechanism if game cancelled
- Bet limits (min/max amounts)
- Leaderboards
- Multi-stream tournaments

---

## 📞 Need Help?

1. Read Anchor docs: https://www.anchor-lang.com/
2. Read Solana docs: https://docs.solana.com/
3. Check test file for examples
4. Test on devnet before mainnet!

---

## ⚖️ License

This is a learning project. Use responsibly and comply with gambling laws in your jurisdiction.

---

## 🎯 Summary

- **Tests = Practice runs with fake data**
- **Frontend = Real app with real users**
- **To make it dynamic = Connect frontend to deployed contract**
- **IDL file = Bridge between contract and frontend**
- **PDAs = Special addresses, calculate them correctly**
- **Always test on devnet first!**

Good luck! 🚀

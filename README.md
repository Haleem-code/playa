# �️ Playa - Solana Betting Contract

A decentralized betting platform for live gaming streams. Watch 1v1 matches, bet on your favorite player, and win instantly on Solana.

## What This Contract Does

This is a smart contract that lets users:
- Create betting pools for gaming streams
- Place bets on Player 1 or Player 2
- Declare winners after the game
- Claim payouts automatically

**No middleman. No delays. Pure Solana speed.**

---

## How It Works

### 1. Create Stream
Stream creator initializes a betting pool:
```typescript
await program.methods.initialize(streamId, bettingDeadline).rpc();
```

### 2. Users Bet
Viewers bet on who will win:
```typescript
await program.methods.placeBet(prediction, amount).rpc();
// prediction: 1 = Player 1, 2 = Player 2
```

### 3. Declare Winner
After game ends, creator declares winner:
```typescript
await program.methods.declareWinner(winningOutcome).rpc();
```

### 4. Winners Get Paid
Winners claim their share:
```typescript
await program.methods.payoutWinners().rpc();
```

---

## Fee Structure

- **Winners:** 95% of total pool
- **Creator:** 2.5% (stream creator fee)
- **Platform:** 2.5% (Playa platform fee)

---

## Program ID

```
Devnet: [Check Anchor.toml after deployment]
```

---

## Quick Start

```bash
# Install dependencies
npm install

# Build
anchor build

# Test
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

---

## Tech Stack

- **Blockchain:** Solana
- **Framework:** Anchor
- **Language:** Rust

---

Built on Solana


###**Initialize** - Create a Betting Pool
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

###  **Place Bet** - Users Bet on Winner
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

### **Declare Winner** - Admin Announces Result
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


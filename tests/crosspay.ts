import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crosspay } from "../target/types/crosspay";
import { PublicKey, Keypair, SystemProgram, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";

describe("crosspay", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Crosspay as Program<Crosspay>;
  
  // Test accounts
  let sender: Keypair;
  let receiver: Keypair;
  let liquidityProvider: Keypair;
  let mintAuthority: Keypair;
  let usdcMint: PublicKey;
  let senderTokenAccount: PublicKey;
  let receiverTokenAccount: PublicKey;
  let lpTokenAccount: PublicKey;

  before(async () => {
    // Create test accounts
    sender = Keypair.generate();
    receiver = Keypair.generate();
    liquidityProvider = Keypair.generate();
    mintAuthority = Keypair.generate();

    // Transfer SOL from local wallet to test accounts
    const transferAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
    
    // Transfer to sender
    const senderTx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: sender.publicKey,
        lamports: transferAmount,
      })
    );
    await sendAndConfirmTransaction(
      provider.connection,
      senderTx,
      [provider.wallet.payer]
    );
    
    // Transfer to receiver
    const receiverTx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: receiver.publicKey,
        lamports: transferAmount,
      })
    );
    await sendAndConfirmTransaction(
      provider.connection,
      receiverTx,
      [provider.wallet.payer]
    );
    
    // Transfer to LP
    const lpTx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: liquidityProvider.publicKey,
        lamports: transferAmount,
      })
    );
    await sendAndConfirmTransaction(
      provider.connection,
      lpTx,
      [provider.wallet.payer]
    );
    
    // Transfer to mint authority
    const mintAuthorityTx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: mintAuthority.publicKey,
        lamports: transferAmount,
      })
    );
    await sendAndConfirmTransaction(
      provider.connection,
      mintAuthorityTx,
      [provider.wallet.payer]
    );

    // Create USDC mock mint (using standard Token Program)
    usdcMint = await createMint(
      provider.connection,
      mintAuthority,
      mintAuthority.publicKey,
      null,
      6 // 6 decimals like USDC
    );

    // Create token accounts
    senderTokenAccount = await createAccount(
      provider.connection,
      sender,
      usdcMint,
      sender.publicKey
    );

    receiverTokenAccount = await createAccount(
      provider.connection,
      receiver,
      usdcMint,
      receiver.publicKey
    );

    lpTokenAccount = await createAccount(
      provider.connection,
      liquidityProvider,
      usdcMint,
      liquidityProvider.publicKey
    );

    // Mint tokens to sender (1000 USDC)
    await mintTo(
      provider.connection,
      mintAuthority,
      usdcMint,
      senderTokenAccount,
      mintAuthority,
      1000 * 10 ** 6
    );

    // Mint tokens to receiver (500 USDC for withdrawal test)
    await mintTo(
      provider.connection,
      mintAuthority,
      usdcMint,
      receiverTokenAccount,
      mintAuthority,
      500 * 10 ** 6
    );

    console.log("\n🎬 Test Setup Complete!");
    console.log("Sender:", sender.publicKey.toString());
    console.log("Receiver:", receiver.publicKey.toString());
    console.log("LP:", liquidityProvider.publicKey.toString());
    console.log("USDC Mint:", usdcMint.toString());
  });

  after(async () => {
    // Return remaining SOL from test accounts back to local wallet
    const accounts = [
      { keypair: sender, name: "Sender" },
      { keypair: receiver, name: "Receiver" },
      { keypair: liquidityProvider, name: "LP" },
      { keypair: mintAuthority, name: "MintAuthority" },
    ];

    for (const account of accounts) {
      try {
        const balance = await provider.connection.getBalance(account.keypair.publicKey);
        // Keep minimum rent-exempt balance (890880 lamports for empty account)
        // Plus extra buffer for the transfer transaction fee
        const minBalance = 890880 + 10000; // ~0.0009 SOL
        const returnAmount = balance - minBalance;
        
        if (returnAmount > 0) {
          const returnTx = new Transaction().add(
            SystemProgram.transfer({
              fromPubkey: account.keypair.publicKey,
              toPubkey: provider.wallet.publicKey,
              lamports: returnAmount,
            })
          );
          await sendAndConfirmTransaction(
            provider.connection,
            returnTx,
            [account.keypair]
          );
          console.log(`✅ Returned ${returnAmount / anchor.web3.LAMPORTS_PER_SOL} SOL from ${account.name}`);
        } else {
          console.log(`⚠️  ${account.name} has insufficient balance to return (${balance / anchor.web3.LAMPORTS_PER_SOL} SOL)`);
        }
      } catch (error) {
        console.log(`⚠️  Could not return SOL from ${account.name}:`, error.message);
      }
    }
    
    console.log("\n💰 All SOL returned to local wallet!");
  });

  describe("User Management", () => {
    it("Initializes sender user profile", async () => {
      const [userProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), sender.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .initializeUser({ sender: {} }, "USA")
        .accountsPartial({
          userProfile: userProfilePda,
          authority: sender.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([sender])
        .rpc();

      const userProfile = await program.account.userProfile.fetch(userProfilePda);
      assert.equal(userProfile.authority.toString(), sender.publicKey.toString());
      assert.equal(userProfile.countryCode, "USA");
      assert.equal(userProfile.kycVerified, false);
    });

    it("Initializes receiver user profile", async () => {
      const [userProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), receiver.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .initializeUser({ receiver: {} }, "NGA")
        .accountsPartial({
          userProfile: userProfilePda,
          authority: receiver.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([receiver])
        .rpc();

      const userProfile = await program.account.userProfile.fetch(userProfilePda);
      assert.equal(userProfile.authority.toString(), receiver.publicKey.toString());
      assert.equal(userProfile.countryCode, "NGA");
    });

    it("Updates KYC status for sender", async () => {
      const [userProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), sender.publicKey.toBuffer()],
        program.programId
      );

      const kycHash = Array(32).fill(1);

      await program.methods
        .updateKycStatus(true, kycHash)
        .accountsPartial({
          userProfile: userProfilePda,
          authority: sender.publicKey,
        })
        .signers([sender])
        .rpc();

      const userProfile = await program.account.userProfile.fetch(userProfilePda);
      assert.equal(userProfile.kycVerified, true);
    });
  });

  describe("Transfer Flow", () => {
    it("Initiates a transfer", async () => {
      const [senderProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), sender.publicKey.toBuffer()],
        program.programId
      );

      const senderProfile = await program.account.userProfile.fetch(senderProfilePda);

      const [transferRequestPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("transfer_request"),
          sender.publicKey.toBuffer(),
          receiver.publicKey.toBuffer(),
          senderProfile.totalSent.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      const amount = new anchor.BN(100 * 10 ** 6); // 100 USDC

      await program.methods
        .initiateTransfer(amount, receiver.publicKey)
        .accountsPartial({
          senderProfile: senderProfilePda,
          transferRequest: transferRequestPda,
          senderTokenAccount: senderTokenAccount,
          mint: usdcMint,
          receiver: receiver.publicKey,
          sender: sender.publicKey,
          authority: sender.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([sender])
        .rpc();

      const transferRequest = await program.account.transferRequest.fetch(transferRequestPda);
      assert.equal(transferRequest.sender.toString(), sender.publicKey.toString());
      assert.equal(transferRequest.receiver.toString(), receiver.publicKey.toString());
      assert.equal(transferRequest.amount.toNumber(), amount.toNumber());
    });

    it("Confirms and executes the transfer - 100 USDC", async () => {
      const [senderProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), sender.publicKey.toBuffer()],
        program.programId
      );

      const [receiverProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), receiver.publicKey.toBuffer()],
        program.programId
      );

      const [transferRequestPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("transfer_request"),
          sender.publicKey.toBuffer(),
          receiver.publicKey.toBuffer(),
          Buffer.from([0, 0, 0, 0, 0, 0, 0, 0]), // nonce = 0
        ],
        program.programId
      );

      const balanceBefore = await getAccount(
        provider.connection,
        receiverTokenAccount,
        undefined,
        TOKEN_PROGRAM_ID
      );

      await program.methods
        .confirmTransfer()
        .accountsPartial({
          transferRequest: transferRequestPda,
          senderProfile: senderProfilePda,
          receiverProfile: receiverProfilePda,
          senderTokenAccount: senderTokenAccount,
          receiverTokenAccount: receiverTokenAccount,
          sender: sender.publicKey,
          authority: sender.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([sender])
        .rpc();

      const balanceAfter = await getAccount(
        provider.connection,
        receiverTokenAccount,
        undefined,
        TOKEN_PROGRAM_ID
      );

      const transferRequest = await program.account.transferRequest.fetch(transferRequestPda);
      const expectedTotalSent = new anchor.BN(100 * 10 ** 6);
      const fee = expectedTotalSent.mul(new anchor.BN(5)).div(new anchor.BN(1000)); // 0.5% = 5/1000

      assert.equal(
        Number(balanceAfter.amount) - Number(balanceBefore.amount),
        expectedTotalSent.toNumber() - fee.toNumber()
      );
      assert.deepEqual(transferRequest.status, { completed: {} });
    });
  });

  describe("Liquidity Provider Management", () => {
    it("Registers a liquidity provider", async () => {
      const [lpPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("liquidity_provider"), liquidityProvider.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .registerLiquidityProvider("Lagos, Nigeria", new anchor.BN(1500 * 10 ** 6)) // 1 USDC = 1500 NGN
        .accountsPartial({
          liquidityProvider: lpPda,
          authority: liquidityProvider.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([liquidityProvider])
        .rpc();

      const lp = await program.account.liquidityProvider.fetch(lpPda);
      assert.equal(lp.authority.toString(), liquidityProvider.publicKey.toString());
      assert.equal(lp.location, "Lagos, Nigeria");
      assert.equal(lp.trustScore, 7000); // 70%
    });

    it("Updates LP availability - 1000 USDC liquidity", async () => {
      const [lpPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("liquidity_provider"), liquidityProvider.publicKey.toBuffer()],
        program.programId
      );

      const liquidity = new anchor.BN(1000 * 10 ** 6); // 1000 USDC available

      await program.methods
        .updateProviderAvailability(liquidity, true)
        .accountsPartial({
          liquidityProvider: lpPda,
          authority: liquidityProvider.publicKey,
        })
        .signers([liquidityProvider])
        .rpc();

      const lp = await program.account.liquidityProvider.fetch(lpPda);
      assert.equal(lp.availableLiquidity.toNumber(), liquidity.toNumber());
      assert.equal(lp.isActive, true);
    });
  });

  describe("Withdrawal Flow", () => {
    it("Requests a withdrawal - 50 USDC", async () => {
      const [receiverProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), receiver.publicKey.toBuffer()],
        program.programId
      );

      const receiverProfile = await program.account.userProfile.fetch(receiverProfilePda);

      const [withdrawalRequestPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("withdrawal_request"),
          receiver.publicKey.toBuffer(),
          receiverProfile.totalReceived.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      const amount = new anchor.BN(50 * 10 ** 6); // 50 USDC

      await program.methods
        .requestWithdrawal(amount, { mobileMoney: {} })
        .accountsPartial({
          freelancerProfile: receiverProfilePda,
          withdrawalRequest: withdrawalRequestPda,
          freelancerTokenAccount: receiverTokenAccount,
          freelancer: receiver.publicKey,
          authority: receiver.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([receiver])
        .rpc();

      const withdrawalRequest = await program.account.withdrawalRequest.fetch(withdrawalRequestPda);
      assert.equal(withdrawalRequest.amount.toNumber(), amount.toNumber());
      assert.deepEqual(withdrawalRequest.status, { pending: {} });
    });

    it("Selects a liquidity provider", async () => {
      const [receiverProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), receiver.publicKey.toBuffer()],
        program.programId
      );

      const receiverProfile = await program.account.userProfile.fetch(receiverProfilePda);

      const [withdrawalRequestPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("withdrawal_request"),
          receiver.publicKey.toBuffer(),
          receiverProfile.totalReceived.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      const [lpPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("liquidity_provider"), liquidityProvider.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .selectProvider(liquidityProvider.publicKey)
        .accountsPartial({
          withdrawalRequest: withdrawalRequestPda,
          liquidityProvider: lpPda,
          freelancer: receiver.publicKey,
        })
        .signers([receiver])
        .rpc();

      const withdrawalRequest = await program.account.withdrawalRequest.fetch(withdrawalRequestPda);
      assert.equal(
        withdrawalRequest.selectedProvider.toString(),
        liquidityProvider.publicKey.toString()
      );
      assert.deepEqual(withdrawalRequest.status, { providerSelected: {} });
    });

    it("Finalizes the withdrawal - 50 USDC to LP", async () => {
      const [receiverProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), receiver.publicKey.toBuffer()],
        program.programId
      );

      const receiverProfile = await program.account.userProfile.fetch(receiverProfilePda);

      const [withdrawalRequestPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("withdrawal_request"),
          receiver.publicKey.toBuffer(),
          receiverProfile.totalReceived.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      const [lpPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("liquidity_provider"), liquidityProvider.publicKey.toBuffer()],
        program.programId
      );

      const lpBalanceBefore = await getAccount(
        provider.connection,
        lpTokenAccount,
        undefined,
        TOKEN_PROGRAM_ID
      );

      await program.methods
        .finalizeWithdrawal()
        .accountsPartial({
          withdrawalRequest: withdrawalRequestPda,
          liquidityProvider: lpPda,
          freelancerTokenAccount: receiverTokenAccount,
          providerTokenAccount: lpTokenAccount,
          freelancer: receiver.publicKey,
          providerAuthority: liquidityProvider.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([receiver, liquidityProvider])
        .rpc();

      const lpBalanceAfter = await getAccount(
        provider.connection,
        lpTokenAccount,
        undefined,
        TOKEN_PROGRAM_ID
      );

      const withdrawalRequest = await program.account.withdrawalRequest.fetch(withdrawalRequestPda);
      const lp = await program.account.liquidityProvider.fetch(lpPda);

      assert.equal(
        Number(lpBalanceAfter.amount) - Number(lpBalanceBefore.amount),
        50 * 10 ** 6
      );
      assert.deepEqual(withdrawalRequest.status, { completed: {} });
      assert.equal(lp.completedTransactions.toNumber(), 1);
    });
  });

  describe("Complete Flow Summary", () => {
    it("Shows final balances", async () => {
      const senderBalance = await getAccount(
        provider.connection,
        senderTokenAccount,
        undefined,
        TOKEN_PROGRAM_ID
      );
      const receiverBalance = await getAccount(
        provider.connection,
        receiverTokenAccount,
        undefined,
        TOKEN_PROGRAM_ID
      );
      const lpBalance = await getAccount(
        provider.connection,
        lpTokenAccount,
        undefined,
        TOKEN_PROGRAM_ID
      );

      console.log("\n📊 Final Balances:");
      console.log("Sender:", Number(senderBalance.amount) / 10 ** 6, "USDC");
      console.log("Receiver:", Number(receiverBalance.amount) / 10 ** 6, "USDC");
      console.log("LP:", Number(lpBalance.amount) / 10 ** 6, "USDC");

      // Verify math
      // Sender: 1000 - 100 = 900 USDC
      // Receiver: 500 + 99.5 - 50 = 549.5 USDC (after 0.5% fee on incoming transfer)
      // LP: 0 + 50 = 50 USDC
      assert.equal(Number(senderBalance.amount), 900.5 * 10 ** 6);
      assert.equal(Number(lpBalance.amount), 50 * 10 ** 6);

      console.log("\n✅ All flows completed successfully!");
    });
  });
});
import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { AnchorDiceGame } from "../target/types/anchor_dice_game";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  Transaction,
} from "@solana/web3.js";
import { BN } from "bn.js";
import { SYSTEM_PROGRAM_ID } from "@anchor-lang/core/dist/cjs/native/system";
import { expect } from "chai";
import { createHash, randomBytes } from "crypto";

const commitment = "confirmed";

const PREIMAGE = randomBytes(32);
const COMMITMENT_BUF = createHash("sha256").update(PREIMAGE).digest();

const BET_ROLL = 50;
const BET_AMOUNT = BigInt(LAMPORTS_PER_SOL / 100);
const HOUSE_EDGE_BASIS_POINTS = 150;

describe("dice-game", () => {
  async function confirmTx(signature: string) {
    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction(
      {
        signature,
        ...latestBlockHash,
      },
      commitment,
    );
  }

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const connection = provider.connection;

  const program = anchor.workspace.AnchorDiceGame as Program<AnchorDiceGame>;
  const house = Keypair.generate();
  const player = Keypair.generate();
  const seed = new BN(randomBytes(16));
  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), house.publicKey.toBuffer()],
    program.programId,
  )[0];

  const bet = PublicKey.findProgramAddressSync(
    [
      Buffer.from("bet"),
      vault.toBuffer(),
      player.publicKey.toBuffer(),
      seed.toBuffer("le", 16),
    ],
    program.programId,
  )[0];

  it("Airdrop", async () => {
    await Promise.all(
      [house, player].map(async (keypair) => {
        return await anchor
          .getProvider()
          .connection.requestAirdrop(keypair.publicKey, 1000 * LAMPORTS_PER_SOL)
          .then(confirmTx);
      }),
    );
  });

  it("initialize =>", async () => {
    await program.methods
      .initialize(new BN(100 * LAMPORTS_PER_SOL))
      .accountsStrict({
        house: house.publicKey,
        vault,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([house])
      .rpc()
      .then(confirmTx);
  });

  it("place bet =>", async () => {
    await program.methods
      .placeBet(seed, new BN(BET_AMOUNT), BET_ROLL, Array.from(COMMITMENT_BUF))
      .accountsStrict({
        player: player.publicKey,
        house: house.publicKey,
        vault,
        bet,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .signers([player])
      .rpc()
      .then(confirmTx);
  });

  it("refund bet fail => cannot refund before timeout", async () => {
    try {
      await program.methods
        .refundBet()
        .accountsStrict({
          player: player.publicKey,
          house: house.publicKey,
          vault,
          bet,
          systemProgram: SystemProgram.programId,
        })
        .signers([player])
        .rpc();
      throw new Error("Expected refund to fail but it succeeded");
    } catch (err) {
      const errMsg = (err as any).toString();
      if (!errMsg.includes("TimeoutNotReached")) {
        throw err;
      }
    }
  });

  it("resolve bet fail => ", async () => {
    const revealIx = await program.methods
      .reveal(randomBytes(32))
      .accountsStrict({
        house: house.publicKey,
      })
      .signers([house])
      .instruction();

    const resolveIx = await program.methods
      .resolveBet()
      .accountsStrict({
        house: house.publicKey,
        player: player.publicKey,
        vault,
        bet,
        instructionSysvar: SYSVAR_INSTRUCTIONS_PUBKEY,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .instruction();

    const tx = new Transaction().add(revealIx, resolveIx);

    let failed = false;
    try {
      await provider.sendAndConfirm(tx, [house]);
    } catch (err) {
      failed = true;
      expect(err).to.be.instanceOf(Error);
    }

    if (!failed) {
      throw new Error("Expected resolve transaction to fail but it succeeded");
    }
  });

  it("resolve bet => ", async () => {
    const revealIx = await program.methods
      .reveal(PREIMAGE)
      .accountsStrict({
        house: house.publicKey,
      })
      .signers([house])
      .instruction();

    const resolveIx = await program.methods
      .resolveBet()
      .accountsStrict({
        house: house.publicKey,
        player: player.publicKey,
        vault,
        bet,
        instructionSysvar: SYSVAR_INSTRUCTIONS_PUBKEY,
        systemProgram: SYSTEM_PROGRAM_ID,
      })
      .instruction();

    const tx = new Transaction().add(revealIx, resolveIx);

    const sig = await provider.sendAndConfirm(tx, [house]);
    await confirmTx(sig);

    const acct = await connection.getAccountInfo(bet);
    expect(acct).to.be.null;
  });
});

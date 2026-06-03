import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { AnchorDiceGame } from "../target/types/anchor_dice_game";
import NodeWallet from "@anchor-lang/core/dist/cjs/nodewallet";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { SYSTEM_PROGRAM_ID } from "@anchor-lang/core/dist/cjs/native/system";
import { expect } from "chai";
import { createHash, randomBytes } from "crypto";

const commitment = "confirmed";

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

  it("inittialize =>", async () => {
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
      .placeBet(seed, new BN(BET_AMOUNT), BET_ROLL)
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
});

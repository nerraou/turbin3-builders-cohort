import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import NodeWallet from "@anchor-lang/core/dist/cjs/nodewallet";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { randomBytes } from "crypto";

const commitment = "confirmed";

describe("anchor-escrow", () => {
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

  function confirmTxs(signatures: string[]) {
    return Promise.all(signatures.map(confirmTx));
  }

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.anchorEscrow as Program<AnchorEscrow>;

  const connection = provider.connection;

  const payer = provider.wallet as NodeWallet;

  const taker = Keypair.generate();

  let mintA: PublicKey;
  let mintB: PublicKey;

  let makerAtaA: PublicKey;
  let makerAtaB: PublicKey;

  let takerAtaA: PublicKey;
  let takerAtaB: PublicKey;

  let vault: PublicKey;

  const seed = new BN(randomBytes(8));

  const escrow = PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), payer.publicKey.toBuffer(), seed.toBuffer("le", 8)],
    program.programId,
  )[0];

  it("should request airdrops for payer and taker", async () => {
    const requests = [payer, taker].map(async (k) => {
      return await connection.requestAirdrop(
        k.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL,
      );
    });

    await Promise.all(requests).then(confirmTxs);
  });
});

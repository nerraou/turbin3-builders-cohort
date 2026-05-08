import {
  address,
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createTransactionMessage,
  getSignatureFromTransaction,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "@solana/kit";
import wallet from "../../devnet-wallet.json";
import {
  findAssociatedTokenPda,
  getCreateAssociatedTokenInstructionAsync,
  getTransferCheckedInstruction,
  TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";
import { rpc, rpcSubscriptions } from "../config/rpc";
import { loadJson } from "../utils/load_json";
import { InitMintResult } from "./types";

async function transferToken() {
  try {
    const { mintAddress } = await loadJson<InitMintResult>("mint_address.json");

    const mint = address(mintAddress);
    const to = address("9KGumQHCZ17TXKeJuf2RMfNeNHBmoJS1GKJbirgiQpui");

    const signer = await createKeyPairSignerFromBytes(new Uint8Array(wallet));

    const sendAndConfirm = sendAndConfirmTransactionFactory({
      rpc,
      rpcSubscriptions,
    });

    const [fromAta] = await findAssociatedTokenPda({
      mint,
      owner: signer.address,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
    });

    console.log(`Your from ata is: ${fromAta}`);

    const [toAta] = await findAssociatedTokenPda({
      mint,
      owner: to,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
    });

    console.log(`Your toAta is: ${toAta}`);

    const createAtaIx = await getCreateAssociatedTokenInstructionAsync({
      payer: signer,
      mint,
      owner: to,
    });

    const transferTx = getTransferCheckedInstruction({
      source: fromAta,
      mint,
      destination: toAta,
      authority: signer,
      amount: 1_000_000n,
      decimals: 6,
    });

    const { value: latestBlockHash } = await rpc.getLatestBlockhash().send();

    const msg = createTransactionMessage({
      version: 0,
    });

    const msgWithPayer = setTransactionMessageFeePayerSigner(signer, msg);

    const msgWithLifeTime = setTransactionMessageLifetimeUsingBlockhash(
      latestBlockHash,
      msgWithPayer,
    );

    const txtMessage = appendTransactionMessageInstructions(
      [createAtaIx, transferTx],
      msgWithLifeTime,
    );

    const signedTx = await signTransactionMessageWithSigners(txtMessage);

    assertIsTransactionWithBlockhashLifetime(signedTx);

    const signature = getSignatureFromTransaction(signedTx);

    await sendAndConfirm(signedTx, { commitment: "confirmed" });

    console.log(`mint txId: ${signature}`);
  } catch (error) {
    console.log(error);
  }
}

transferToken();

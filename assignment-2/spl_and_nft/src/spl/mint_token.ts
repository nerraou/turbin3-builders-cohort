import {
  address,
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
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
  getMintToInstruction,
  TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";
import { rpc, rpcSubscriptions } from "../config/rpc";
import { loadJson } from "../utils/load_json";
import { InitMintResult } from "./types";
import { saveJson } from "../utils/save_json";

const TOKEN_DECIMALS = 1_000_000n;

async function mintToken() {
  try {
    const signer = await createKeyPairSignerFromBytes(new Uint8Array(wallet));

    const { mintAddress } = await loadJson<InitMintResult>("mint_address.json");

    const mint = address(mintAddress);

    const [ata] = await findAssociatedTokenPda({
      mint,
      owner: signer.address,
      tokenProgram: TOKEN_PROGRAM_ADDRESS,
    });
    console.log(`Your ata is: ${ata}`);

    const createAtaIx = await getCreateAssociatedTokenInstructionAsync({
      mint,
      owner: signer.address,
      payer: signer,
    });

    const mintToIx = getMintToInstruction({
      mint,
      token: ata,
      mintAuthority: signer,
      amount: 1n * TOKEN_DECIMALS,
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
      [createAtaIx, mintToIx],
      msgWithLifeTime,
    );

    const signedTx = await signTransactionMessageWithSigners(txtMessage);

    assertIsTransactionWithBlockhashLifetime(signedTx);

    const signature = getSignatureFromTransaction(signedTx);

    const sendAndConfirm = sendAndConfirmTransactionFactory({
      rpc,
      rpcSubscriptions,
    });

    await sendAndConfirm(signedTx, { commitment: "confirmed" });

    console.log(`mint txId: ${signature}`);

    await saveJson("ata_address.json", {
      ata,
      signature,
    });
  } catch (error) {
    console.log(error);
  }
}

mintToken();

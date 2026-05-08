import {
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createTransactionMessage,
  generateKeyPairSigner,
  getSignatureFromTransaction,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "@solana/kit";
import {
  getInitializeMintInstruction,
  getMintSize,
  TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";
import { getCreateAccountInstruction } from "@solana-program/system";
import { rpc, rpcSubscriptions } from "../config/rpc";
import { loadWallet } from "../wallet/load_wallet";
import { saveJson } from "../utils/save_json";
import { MINT_ADDRESS_FILENAME } from "../config/consts";

export async function initMint() {
  try {
    const signer = await loadWallet();

    const mint = await generateKeyPairSigner();

    const space = BigInt(getMintSize());

    const rent = await rpc.getMinimumBalanceForRentExemption(space).send();

    const { value: latestBlockHash } = await rpc.getLatestBlockhash().send();

    const sendAndConfirm = sendAndConfirmTransactionFactory({
      rpc,
      rpcSubscriptions,
    });

    const msg = createTransactionMessage({ version: 0 });
    const msgWithPayer = setTransactionMessageFeePayerSigner(signer, msg);
    const msgWithLifeTime = setTransactionMessageLifetimeUsingBlockhash(
      latestBlockHash,
      msgWithPayer,
    );

    const txtMessage = appendTransactionMessageInstructions(
      [
        getCreateAccountInstruction({
          payer: signer,
          newAccount: mint,
          lamports: rent,
          space,
          programAddress: TOKEN_PROGRAM_ADDRESS,
        }),
        getInitializeMintInstruction({
          mint: mint.address,
          decimals: 6,
          mintAuthority: signer.address,
        }),
      ],
      msgWithLifeTime,
    );

    const signedTX = await signTransactionMessageWithSigners(txtMessage);

    assertIsTransactionWithBlockhashLifetime(signedTX);

    const signature = getSignatureFromTransaction(signedTX);

    await sendAndConfirm(signedTX, { commitment: "confirmed" });

    console.info(
      `mint address: ${mint.address}, transaction signature  ${signature}`,
    );

    await saveJson(MINT_ADDRESS_FILENAME, {
      mintAddress: mint.address,
      signature,
    });
  } catch (error) {
    console.error("initMint", error);

    return null;
  }
}

initMint();

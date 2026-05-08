import { createKeyPairSignerFromBytes } from "@solana/kit";
import wallet from "../../devnet-wallet.json";

export function loadWallet() {
  return createKeyPairSignerFromBytes(new Uint8Array(wallet));
}

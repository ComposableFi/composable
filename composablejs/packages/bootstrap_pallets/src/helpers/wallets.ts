import Keyring from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import { hexToU8a } from "@polkadot/util";
import { ethers } from "ethers";
import { privateKeyFromSeed } from "@composable/bootstrap_pallets/utils";

export function getSubstrateWallets(maxWallets = 3): KeyringPair[] {
  const kr = new Keyring({ type: "sr25519" });
  const wallets: KeyringPair[] = [];

  let idx = 0;
  while (idx < maxWallets) {
    wallets.push(kr.addFromUri(`//Saad-${idx++}`));
  }

  return wallets;
}

export function getEthereumWallets(): ethers.Wallet[] {
  return [
    new ethers.Wallet(privateKeyFromSeed(1)),
    new ethers.Wallet(privateKeyFromSeed(2)),
    new ethers.Wallet(privateKeyFromSeed(3)),
    new ethers.Wallet(privateKeyFromSeed(4)),
    new ethers.Wallet(privateKeyFromSeed(5))
  ];
}

export function getSudoWallet(chain: "dali-local" | string): KeyringPair {
  const kr = new Keyring({ type: "sr25519" });
  if (chain === "dali-local") {
    return kr.addFromUri("//Alice");
  } else {
    const pk = process.env.SUDO_SEED && process.env.SUDO_SEED.length ? process.env.SUDO_SEED : undefined;
    if (!pk?.length) throw new Error("Provide a sudo key env variable");
    return kr.addFromSeed(hexToU8a(pk));
  }
}

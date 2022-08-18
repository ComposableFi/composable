import { IKeyringPair } from "@polkadot/types/types";

export const toHexString = (bytes: Uint8Array) =>
  Array.prototype.map.call(bytes, x => ("0" + (x & 0xff).toString(16)).slice(-2)).join("");

// The prefix is defined as pallet config
export const proofMessage = (account: IKeyringPair) => "picasso-" + toHexString(account.publicKey);

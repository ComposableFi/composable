import * as dotenv from "dotenv"; // see https://github.com/motdotla/dotenv#how-do-i-use-dotenv-with-import

dotenv.config();

export const chain = (): string => {
  switch (process.env.ENV) {
    case "dali":
      return "wss://dali.devnets.composablefinance.ninja/parachain/alice";
    case "dali-stage":
      return "wss://dali-cluster-fe.composablefinance.ninja";
    default:
      if ("RELAYCHAIN_URI" in process.env) {
        return process.env.RELAYCHAIN_URI!.toString();
      }

      return "ws://127.0.0.1:9988";
  }
};

export const archive = (): string => {
  if ("SUBSQUID_ARCHIVE_URI" in process.env) {
    return process.env.SUBSQUID_ARCHIVE_URI!.toString();
  }

  return "https://subsquid-archive.composablenodes.tech/graphql";
};

export const firstBlock = (): number => {
  const relayChain = chain();
  if (relayChain === "wss://rpc.composablenodes.tech") {
    // Start from a block close to this runtime upgrade from Picasso
    // https://picasso.subscan.io/extrinsic/0xc875c8916e23c119f1d4202914dd0f28304aff62e46b0d51fed9b34e0aa30d9c
    return 1_227_000;
  }
  return 0;
};

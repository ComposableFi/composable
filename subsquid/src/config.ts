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

  return "http://127.0.0.1:8888/graphql";
};

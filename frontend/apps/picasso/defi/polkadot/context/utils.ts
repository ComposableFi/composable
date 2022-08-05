import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

export const getChainId = (
  relayChain: "kusama" | "polkadot",
  parachainId: number
) => {
  const network = Object.values(SUBSTRATE_NETWORKS).findIndex(
    (network) =>
      network.relayChain === relayChain && network.parachainId === parachainId
  );
  if (network === -1) {
    return "kusama";
  }

  return Object.keys(SUBSTRATE_NETWORKS)[network];
  // return relayChain + (parachainId ? "-" + parachainId.toFixed() : "");
};

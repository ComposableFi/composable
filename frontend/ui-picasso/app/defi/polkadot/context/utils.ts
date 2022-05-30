export const getChainId = (
  relayChain: "kusama" | "polkadot",
  parachainId: number
) => {
  return relayChain + (parachainId ? "-" + parachainId.toFixed() : "");
};

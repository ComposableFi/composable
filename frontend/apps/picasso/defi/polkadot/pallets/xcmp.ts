import { SubstrateNetworkId } from "shared";

export function availableTargetNetwork(
  network: SubstrateNetworkId,
  selectedNetwork: SubstrateNetworkId
) {
  switch (selectedNetwork) {
    case "kusama":
      return network === "picasso";
    case "picasso":
      return ["kusama", "karura", "statemine"].includes(network);
    case "karura":
      return network === "picasso";
    case "statemine":
      return network === "picasso";
  }
}

import { useContext } from "react";
import { ParachainApi, ParachainContext } from "./ParachainContext";
import { getChainId } from "./utils";

export const useParachainProvider = (
  relayChain: "polkadot" | "kusama",
  parachainId: number | 0 | undefined
): ParachainApi => {
  const { parachainProviders } = useContext(ParachainContext);

  const chainId = getChainId(relayChain, parachainId ? parachainId : 0);

  if (chainId in parachainProviders) {
    return parachainProviders[chainId];
  } else {
    return {
      accounts: [],
      apiStatus: "initializing",
      chainId,
      parachainApi: undefined,
      ss58Format: 0,
    };
  }
};

export const useAllParachainProviders = () => {
  const { parachainProviders } = useContext(ParachainContext);
  return parachainProviders;
};

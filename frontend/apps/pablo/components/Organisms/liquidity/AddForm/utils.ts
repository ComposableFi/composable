import { option, readonlyArray } from "fp-ts";
import { PoolConfig } from "@/store/createPool/types";
import BigNumber from "bignumber.js";
import { flow, pipe } from "fp-ts/function";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { SubstrateNetworkId } from "shared";
import { InputConfig } from "@/components/Organisms/liquidity/AddForm/types";

export function getInputConfig(
  pool: option.Option<PoolConfig>,
  getTokenBalance: (
    tokenId: string,
    network: "kusama" | "picasso" | "karura" | "statemine"
  ) => { free: BigNumber; locked: BigNumber }
): option.Option<InputConfig[]> {
  return pipe(
    pool,
    option.map((p) => p.config.assets),
    option.map(
      flow(
        readonlyArray.fromArray,
        readonlyArray.map((asset) => ({
          asset,
          chainId: DEFAULT_NETWORK_ID as SubstrateNetworkId,
          balance: getTokenBalance(
            asset.getPicassoAssetId() as string,
            DEFAULT_NETWORK_ID
          ),
        })),
        readonlyArray.toArray
      )
    )
  );
}

export function getAssetOptions(config: InputConfig[]) {
  return [
    {
      value: "none",
      label: "Select",
      icon: undefined,
      disabled: true,
      hidden: true,
    },
    ...Object.values(config).map((item) => ({
      value: item.asset.getIdOnChain(DEFAULT_NETWORK_ID) as string,
      label: item.asset.getSymbol(),
      icon: item.asset.getIconUrl(),
    })),
  ];
}

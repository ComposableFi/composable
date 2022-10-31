import { TokenId } from "tokens";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { SubstrateNetworkId } from "../../types";
import BigNumber from "bignumber.js";

export function extractTokenByNetworkIdentifier(
  tokens: Record<TokenId, TokenMetadata>,
  network: SubstrateNetworkId,
  id: BigNumber | string | number
): TokenMetadata | null {
  let meta = null;
  for (const token in tokens) {
    if (tokens[token as TokenId].chainId[network]) {
      switch (network) {
        case "karura":
        case "kusama":
          if (
            tokens[token as TokenId].chainId[network] ===
            (id as number | string)
          ) {
            meta = tokens[token as TokenId];
          }
          break;
        case "picasso":
          if ((tokens[token as TokenId].chainId[network] as BigNumber).eq(id)) {
            meta = tokens[token as TokenId];
          }
          break;
      }
    }
  }

  return meta;
}

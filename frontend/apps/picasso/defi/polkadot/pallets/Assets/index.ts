import { TokenId } from "tokens";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { ApiPromise } from "@polkadot/api";
import { SubstrateNetworkId } from "../../types";
import BigNumber from "bignumber.js";

export const getSubstrateNetworkAssetIdentifierKey = (
  network: SubstrateNetworkId
) =>
  network === "karura"
    ? "karuraId"
    : network === "picasso"
    ? "picassoId"
    : "kusamaId";

export function extractTokenByNetworkIdentifier(
  tokens: Record<TokenId, TokenMetadata>,
  network: SubstrateNetworkId,
  id: BigNumber | string | number
): TokenMetadata | null {
  let meta = null;
  const key = getSubstrateNetworkAssetIdentifierKey(network);
  for (const token in tokens) {
    if (tokens[token as TokenId][key]) {
      switch (typeof tokens[token as TokenId][key]) {
        case "string":
        case "number":
          if (tokens[token as TokenId][key] === (id as number | string)) {
            meta = tokens[token as TokenId];
          }
          break;
        case "object":
          if (
            (tokens[token as TokenId][key] as BigNumber).eq(id as BigNumber)
          ) {
            meta = tokens[token as TokenId];
          }
          break;
      }
    }
  }

  return meta;
}

export async function listAssets(
  api: ApiPromise
): Promise<Array<{ id: BigNumber; name: string }>> {
  const list = await api.rpc.assets.listAssets();
  return list.map((asset) => ({
    name: asset.name.toUtf8(),
    id: new BigNumber(asset.id.toString()),
  }));
}

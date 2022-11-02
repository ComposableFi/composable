import { ApiPromise } from "@polkadot/api";
import { hexToAscii } from "shared";

export type HumanizedKaruraAssetMetadata = {
  name: string;
  symbol: string;
  decimals: number;
  minimalBalance: string;
};

/**
 * https://wiki.acala.network/get-started/get-started/karura-assets
 * Fetching list of karura on chain ids and storing it in FE store
 * we try to map received assets symbol(lowercased) to hardcoded token
 * ids we have in "tokens" package
 * @param api ApiPromise
 * @returns {HumanizedKaruraAssetMetadata}
 */
export async function karuraAssetsList(
  api: ApiPromise
): Promise<Array<HumanizedKaruraAssetMetadata>> {
  try {
    // @ts-ignore
    const assetMetadata =
      await api.query.assetRegistry.assetMetadatas.entries();
    return assetMetadata.map((meta) => {
      const target = meta[1].toJSON() as HumanizedKaruraAssetMetadata;

      return {
        name: hexToAscii(target.name),
        symbol: hexToAscii(target.symbol),
        decimals: target.decimals,
        minimalBalance: target.minimalBalance,
      };
    });
  } catch (err) {
    console.error("[√] ", err);
    return [] as HumanizedKaruraAssetMetadata[];
  }
}

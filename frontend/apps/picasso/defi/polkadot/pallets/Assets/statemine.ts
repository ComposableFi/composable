import { ApiPromise } from "@polkadot/api";
import { hexToString } from "@polkadot/util";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";

export type StatemineAssetMetadata = {
  name: string;
  decimals: number;
  symbol: string;
  deposit: number;
  id: string;
  existentialDeposit: BigNumber | null;
};

type StatemineAssetDetail = {
  owner: string;
  issuer: string;
  freezer: string;
  supply: number;
  deposit: number;
  minBalance: number;
  isSufficient: true;
  accounts: number;
  sufficients: number;
  approvals: number;
  isFrozen: boolean;
};

export async function statemineAssetList(
  api: ApiPromise
): Promise<StatemineAssetMetadata[]> {
  const assets = (await api.query.assets.metadata.entries()).map(
    async ([key, value]) => {
      const json = value.toJSON() as StatemineAssetMetadata;
      if (!json) return null;

      const assetDetails = (
        await api.query.assets.asset(key.args[0].toJSON())
      ).toJSON() as StatemineAssetDetail;
      let existentialDeposit = null;
      if (assetDetails?.minBalance) {
        existentialDeposit = assetDetails.minBalance;
      }

      return {
        id: String(key.args[0].toJSON()),
        name: hexToString(json.name),
        decimals: json.decimals,
        symbol: hexToString(json.symbol),
        deposit: json.deposit,
        existentialDeposit: existentialDeposit
          ? fromChainIdUnit(existentialDeposit, json.decimals)
          : null,
      } as StatemineAssetMetadata;
    }
  );

  const result = await Promise.all(assets);
  return result.filter((a) => a !== null) as StatemineAssetMetadata[];
}

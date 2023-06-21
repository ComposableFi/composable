import { DEFAULT_NETWORK_ID, fromChainUnits } from "@/defi/utils";
import { pipe } from "fp-ts/function";
import * as O from "fp-ts/Option";
import { getOraclePrice } from "@/store/oracle/slice";
import BigNumber from "bignumber.js";
import { Asset, SubstrateNetworkId } from "shared";

export function parseLockedValue(
  getTokenById: (
    assetId: string,
    network: SubstrateNetworkId
  ) => Asset | undefined,
  picaPrice: BigNumber
) {
  return function reducer(
    acc: BigNumber,
    cur: {
      assetId: string;
      amount: string;
    }
  ) {
    return pipe(
      getTokenById(cur.assetId, DEFAULT_NETWORK_ID),
      O.fromNullable,
      O.map((asset) =>
        pipe(
          asset,
          (a) => fromChainUnits(cur.amount, a.getDecimals(DEFAULT_NETWORK_ID)),
          (amount) =>
            amount.multipliedBy(
              cur.assetId === "1"
                ? picaPrice
                : getOraclePrice(asset.getSymbol(), "coingecko", "usd")
            )
        )
      ),
      O.fold(
        () => acc,
        (price) => {
          return acc.plus(price);
        }
      )
    );
  };
}

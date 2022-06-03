import { useEffect } from "react";
import useStore from "@/store/useStore";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { getAsset } from "@/defi/polkadot/Assets";
import { createPoolAccountId, isValidAssetPair } from "../../utils";
import { AssetId } from "@/defi/polkadot/types";
import { fetchBalanceByAssetId } from "../../balances/utils";

/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const selectedAccount = useSelectedAccount("picasso");
  const {
    pools: { constantProductPools, stableSwapPools },
    addLiquidity: { form, setPoolMetadata, pool },
  } = useStore();
  const { parachainApi } = useParachainApi("picasso");

  useEffect(() => {
    if (parachainApi) {
      if (
        constantProductPools.verified.length > 0 &&
        isValidAssetPair(form.baseAssetSelected, form.quoteAssetSelected)
      ) {
        const onChainBaseAssetId = getAsset(form.baseAssetSelected as AssetId)
          .supportedNetwork.picasso as number;
        const onChainQuoteAssetId = getAsset(form.quoteAssetSelected as AssetId)
          .supportedNetwork.picasso as number;

        parachainApi.query.dexRouter
          .dexRoutes(onChainBaseAssetId, onChainQuoteAssetId)
          .then((dexRoute) => {
            const dexRouteDecoded: any = dexRoute.toJSON();
            console.log("dexRouteDecoded", dexRouteDecoded);

            if (
              dexRouteDecoded &&
              dexRouteDecoded.direct &&
              dexRouteDecoded.direct.length > 0
            ) {
              const poolId = (dexRouteDecoded.direct as number[])[0];

              let pool: any = constantProductPools.verified.find(
                (i) => i.poolId === poolId
              );

              if (!pool) {
                pool = stableSwapPools.verified.find((p) => p.poolId === poolId);
              }


              if (pool) {
                let accountId = createPoolAccountId(poolId);

                let balances = [
                  fetchBalanceByAssetId(
                    parachainApi,
                    "picasso",
                    accountId,
                    onChainBaseAssetId.toString()
                  ),
                  fetchBalanceByAssetId(
                    parachainApi,
                    "picasso",
                    accountId,
                    onChainQuoteAssetId.toString()
                  ),
                ];

                Promise.all(balances).then(([baseBalance, quoteBalance]) => {
                  console.log({
                    balance: { base: baseBalance, quote: quoteBalance },
                    ...pool,
                  });

                  setPoolMetadata({
                    balance: { base: baseBalance, quote: quoteBalance },
                    ...pool,
                  });
                });
              }
            }
          });
      } else {
        setPoolMetadata({
          balance: { base: "0", quote: "0" },
          poolId: -1,
        });
      }
    }
  }, [
    parachainApi,
    form,
    constantProductPools.verified.length,
    stableSwapPools.verified.length,
  ]);

  useEffect(() => {
    if (pool.poolId !== -1 && parachainApi && selectedAccount) {
      (parachainApi.rpc as any).assets
        .balanceOf(
          parachainApi.createType("CurrencyId", pool.lpToken),
          parachainApi.createType("AccountId32", selectedAccount.address)
        )
        .then((balance: any) => {
          console.log("balance ", balance.toString());
        });
    }
  }, [pool, selectedAccount, parachainApi]);

  return null;
};

export default Updater;

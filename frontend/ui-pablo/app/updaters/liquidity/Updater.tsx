import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import _ from "lodash";
import { useEffect, useMemo } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "../constants";
import {
  fetchAndUpdatePoolLiquidity,
} from "./utils";
import { fetchBalanceByAssetId } from "../balances/utils";
// import { fetchBalanceByAssetId } from "../balances/utils";
// import { createPoolAccountId } from "@/utils/substrate";

const PICK = ["poolId", "pair", "lpToken"];
const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const {
    assets,
    pools,
    setTokenAmountInLiquidityPool,
    setTokenValueInLiquidityPool,
    setUserLpBalance,
    poolLiquidity,
  } = useStore();
  /**
   * Select pools tracking
   * liquidity
   */
  const allPools = useMemo(() => {
    return [
      ...pools.constantProductPools.unVerified.map((p) => _.pick(p, PICK)),
      ...pools.constantProductPools.verified.map((p) => _.pick(p, PICK)),
      ...pools.stableSwapPools.unVerified.map((p) => _.pick(p, PICK)),
      ...pools.stableSwapPools.verified.map((p) => _.pick(p, PICK)),
    ];
  }, [pools]);
  /**
   * For each pool, fetch its
   * base and quote token amount
   * and update it in zustand store
   * (first call)
   */
  useEffect(() => {
    if (allPools.length && parachainApi) {
      allPools.forEach((pool) => {
        if (pool.poolId && pool.pair) {
          fetchAndUpdatePoolLiquidity(
            pool as any,
            setTokenAmountInLiquidityPool,
            parachainApi
          );
        }
      });
    }
  }, [allPools.length, parachainApi]);
  /**
   * Fetch and update LP Balances within
   * zustand store
   */
  useEffect(() => {
    if (allPools.length && selectedAccount && parachainApi) {
      allPools.forEach((pool) => {
        if (pool.poolId && pool.pair && pool.lpToken) {
          fetchBalanceByAssetId(
            parachainApi,
            DEFAULT_NETWORK_ID,
            selectedAccount.address,
            pool.lpToken
          ).then((lpBalance) => {
            setUserLpBalance(pool.poolId as number, lpBalance);
          });
        }
      });
    }
  }, [parachainApi, allPools.length, selectedAccount]);
  /**
   * For each pool, update zustand
   * store with value of tokens
   * locked within them
   */
  useEffect(() => {
    if (allPools.length) {
      allPools.forEach((pool) => {
        if (pool.poolId && pool.pair) {
          const baseAssetMeta = getAssetByOnChainId(
            DEFAULT_NETWORK_ID,
            pool.pair.base
          );
          const quoteAssetMeta = getAssetByOnChainId(
            DEFAULT_NETWORK_ID,
            pool.pair.quote
          );

          if (assets[baseAssetMeta.assetId] && poolLiquidity[pool.poolId]) {
            const baseValue = new BigNumber(
              poolLiquidity[pool.poolId].tokenAmounts.baseAmount
            )
              .times(assets[baseAssetMeta.assetId].price.toString())
              .toString();
            setTokenValueInLiquidityPool(pool.poolId, {
              baseValue,
            });
          }
          if (assets[quoteAssetMeta.assetId] && poolLiquidity[pool.poolId]) {
            const quoteValue = new BigNumber(
              poolLiquidity[pool.poolId].tokenAmounts.quoteAmount
            )
              .times(assets[quoteAssetMeta.assetId].price.toString())
              .toString();

            setTokenValueInLiquidityPool(pool.poolId, {
              quoteValue,
            });
          }
        }
      });
    }
  }, [allPools.length, assets]);
  // might add following or not
  // const extrinsicCalls = useExtrinsics();
  // const trackedTransactions = useRef<string[]>([]);
  // useEffect(() => {
  //   if (
  //     parachainApi &&
  //     selectedAccount &&
  //     Object.values(extrinsicCalls).length > 0
  //   ) {
  //     const txs = Object.values(extrinsicCalls);

  //     let shouldUpdate: string | null = null;
  //     txs.forEach((tx) => {
  //       if (
  //         tx.sender === selectedAccount.address &&
  //         tx.status === "isFinalized" &&
  //         (tx.section === "dexRouter" || tx.section === "pablo") &&
  //         !trackedTransactions.current.includes(tx.hash)
  //       ) {
  //         shouldUpdate = tx.hash;
  //       }
  //     });

  //     if (shouldUpdate !== null) {
  //       const allPromises: Promise<any>[] = [];

  //       Promise.all(allPromises).then((updatedBalancesAssetList) => {
  //         trackedTransactions.current.push(shouldUpdate as string);
  //       });
  //     }
  //   }
  // }, [extrinsicCalls, parachainApi, selectedAccount]);
  // useEffect(() => {
  //   if (parachainApi && selectedAccount) {
  //     let subscription: any;
  //     (async () => {
  //       subscription = await parachainApi.query.system.events((events) => {
  //         events.forEach(e => {
  //           const isLiquidityAdded = parachainApi.events.pablo.LiquidityAdded.is(e.event);
  //           if (isLiquidityAdded) {
  //             console.log(e.event.data.toJSON())
  //           }
  //         })
  //       })
  //     })()

  //     return () => {
  //       console.log('cleaning up', subscription)
  //       subscription()
  //     }
  //   }
  // }, [parachainApi, selectedAccount])


  return null;
};

export default Updater;

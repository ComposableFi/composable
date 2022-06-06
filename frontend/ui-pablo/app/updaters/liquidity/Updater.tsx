import { Assets, getAssetByOnChainId } from "@/defi/polkadot/Assets";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import _ from "lodash";
import { useEffect, useMemo } from "react";
import {
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import { DEFAULT_NETWORK_ID } from "../constants";
import { liquidityTransactionsByAddressAndPool } from "@ui-pablo/app/updaters/pools/subsquid";
import {
  fetchAndUpdatePoolLiquidity,
  processLiquidityTransactionsByAddress,
} from "./utils";
// import { fetchBalanceByAssetId } from "../balances/utils";
// import { createPoolAccountId } from "@/utils/substrate";

const PICK = ["poolId", "pair", "lpToken"];
const Updater = () => {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const {
    assets,
    pools,
    setTokenAmountInPool,
    setTokenValueInPool,
    setUserProvidedTokenAmountInPool,
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
            setTokenAmountInPool,
            parachainApi
          );
        }
      });
    }
  }, [allPools.length, parachainApi]);
  /**
   * For each pool query the liquidity
   * (amount of base and quote tokens)
   * provided by connectedAccount
   */
  useEffect(() => {
    if (allPools.length && selectedAccount) {
      allPools.forEach((pool) => {
        if (pool.poolId && pool.pair) {
          liquidityTransactionsByAddressAndPool(
            selectedAccount.address,
            pool.poolId
          ).then((userLiqTransactions) => {
            let { base, quote } = processLiquidityTransactionsByAddress(
              userLiqTransactions.data.pabloTransactions
            );
            setUserProvidedTokenAmountInPool((pool as any).poolId, {
              baseAmount: base.toString(),
              quoteAmount: quote.toString(),
            });
          });
        }
      });
    }
  }, [allPools.length, selectedAccount]);
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
            setTokenValueInPool(pool.poolId, {
              baseValue,
            });
          }
          if (assets[quoteAssetMeta.assetId] && poolLiquidity[pool.poolId]) {
            const quoteValue = new BigNumber(
              poolLiquidity[pool.poolId].tokenAmounts.quoteAmount
            )
              .times(assets[quoteAssetMeta.assetId].price.toString())
              .toString();

            setTokenValueInPool(pool.poolId, {
              quoteValue,
            });
          }
        }
      });
    }
  }, [allPools.length, assets]);

  // const trackedTransactions = useRef<string[]>([]);
  // const extrinsicCalls = useExtrinsics();

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

  return null;
};

export default Updater;

import { useEffect } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { useParachainApi, useSelectedAccount } from "substrate-react";

import {
  StableSwapPool,
  ConstantProductPool,
  LiquidityBootstrappingPool,
} from "@/store/pools/pools.types";
import { decodeCpp, decodeLbp, decodeSsp } from "./utils";
import { fetchBalanceByAssetId } from "../balances/utils";

/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const {
    pools: {
      setPoolsList,
      liquidityBootstrappingPools,
      constantProductPools,
      stableSwapPools,
      user: {
        setUserLpBalance
      }
    },
  } = useStore();
  const { parachainApi } = useParachainApi("picasso");
  const selectedAccount = useSelectedAccount("picasso");

  useEffect(() => {
    if (parachainApi) {
      parachainApi.query.pablo.poolCount().then(async (poolCount) => {
        const _poolCount = new BigNumber(poolCount.toString());

        let poolPromises = [];
        for (let i = 0; i < _poolCount.toNumber(); i++) {
          poolPromises.push(parachainApi.query.pablo.pools(i));
        }

        const pools = await Promise.all(poolPromises);
        const currentBlock = await parachainApi.query.system.number();
        const currentBlockBN = new BigNumber(currentBlock.toString());

        let _lbpools: LiquidityBootstrappingPool[] = [],
          _cpPools: ConstantProductPool[] = [],
          _ssPools: StableSwapPool[] = [];

        pools.map((pool, index) => {
          let decodedPool: any = pool.toHuman();
          if ("LiquidityBootstrapping" in decodedPool) {
            decodedPool = decodeLbp(
              decodedPool.LiquidityBootstrapping,
              index,
              currentBlockBN
            );
            _lbpools.push(decodedPool);
          } else if ("ConstantProduct" in decodedPool) {
            decodedPool = decodeCpp(decodedPool.ConstantProduct, index);
            _cpPools.push(decodedPool);
          } else if ("StableSwap" in decodedPool) {
            decodedPool = decodeSsp((decodedPool as any).StableSwap, index);
            _ssPools.push(decodedPool);
          }
        });

        const allPools = _lbpools
          .map((p) => {
            return { pair: p.pair, id: p.poolId };
          })
          .concat(
            _cpPools.map((p) => {
              return { pair: p.pair, id: p.poolId };
            })
          ).concat(
            _ssPools.map((p) => {
              return {pair: p.pair, id: p.poolId };
            })
          );

        let permissioned = allPools.concat().map((pool) => {
          return new Promise((res, rej) => {
            parachainApi.query.dexRouter
              .dexRoutes(pool.pair.base, pool.pair.quote)
              .then((dexRoute) => {
                const response: any = dexRoute.toJSON();
                if (response && response.direct) {
                  res(pool.id);
                }
                res(null);
              })
              .catch((err) => {
                res(null);
              });
          });
        });

        Promise.all(permissioned).then((pools) => {
          pools = pools.filter((i) => i === null);

          const _lbVerifiedpools = _lbpools.filter((pool) => {
            return (
              pools.find((p) => {
                return p === pool.poolId;
              }) === undefined
            );
          });

          const _lbUnVerifiedpools = _lbpools.filter((pool) => {
            return (
              pools.find((p) => {
                return p === pool.poolId;
              }) === undefined
            );
          });

          const _cpUnVerfiedPools = _cpPools.filter((pool) => {
            return pools.find((p) => p === pool.poolId) !== undefined;
          });

          const _cpVerifiedPools = _cpPools.filter((pool) => {
            return pools.find((p) => p === pool.poolId) === undefined;
          });

          const _ssVerifiedPools = _ssPools.filter((pool) => {
            return pools.find((p) => p === pool.poolId) === undefined;
          });

          const _ssUnverifiedPools = _ssPools.filter((pool) => {
            return pools.find((p) => p === pool.poolId) !== undefined;
          });

          setPoolsList(_lbVerifiedpools, "LiquidityBootstrapping", true);
          setPoolsList(_lbUnVerifiedpools, "LiquidityBootstrapping", false);
          setPoolsList(_cpVerifiedPools, "ConstantProduct", true);
          setPoolsList(_cpUnVerfiedPools, "ConstantProduct", false);
          setPoolsList(_ssVerifiedPools, "StableSwap", true);
          setPoolsList(_ssUnverifiedPools, "StableSwap", false);
        });
      });
    }
  }, [parachainApi]);
  /**
   * Only StableSwap and ConstantProduct Pools
   * offer LP Tokens so we need to query 
   * selected account LP balances
   */
  useEffect(() => {
    if (
      parachainApi &&
      selectedAccount &&
      stableSwapPools.verified.length > 0 &&
      constantProductPools.verified.length > 0
    ) {
      const cpPools = constantProductPools.verified.concat(constantProductPools.nonVerified).map(p => ({
        poolId: p.poolId,
        lpToken: p.lpToken
      }))

      const ssPools = stableSwapPools.verified.concat(stableSwapPools.nonVerified).map(p => ({
        poolId: p.poolId,
        lpToken: p.lpToken
      }))

      cpPools.concat(ssPools).forEach((pool) => {
        fetchBalanceByAssetId(
          parachainApi,
          "picasso",
          selectedAccount.address,
          pool.lpToken
        ).then((lpBalance) => {
          console.log('Lp Token: ', pool.lpToken, ' Balance: ', lpBalance)
          setUserLpBalance(pool.poolId, lpBalance)
        });
      });
    }
  }, [
    parachainApi,
    constantProductPools.verified.length,
    stableSwapPools.verified.length,
    selectedAccount,
  ]);

  return null;
};

export default Updater;

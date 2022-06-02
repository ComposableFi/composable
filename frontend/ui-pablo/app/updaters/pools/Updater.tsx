import { useEffect } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { useParachainApi } from "substrate-react";
import {
  StableSwapPool,
  ConstantProductPool,
  LiquidityBootstrappingPool,
} from "@/store/pools/pools.types";
import { decodeCpp, decodeLbp, decodeSsp } from "./utils";
import {
  queryPabloPoolById,
} from "./subsquid";
import { OperationResult } from "urql";
import _ from "lodash";
import { DAYS } from "../constants";
import { useAllLpTokenRewardingPools } from "../../store/hooks/useAllLpTokenRewardingPools";

function isVerifiedPool(
  verifiedPoolIds: number[],
  pool: { poolId: number }
): boolean {
  return verifiedPoolIds.some((p) => p === pool.poolId);
}

/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const {
    pools: {
      setPoolsList,
    },
  } = useStore();
  const { parachainApi } = useParachainApi("picasso");
  /**
   * Populate all pools
   * from the pallet
   */
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

        const pick = ["poolId", "pair"];
        const allPoolIdsAndPairs = [
          ..._lbpools.map((p) => _.pick(p, pick)),
          ..._cpPools.map((p) => _.pick(p, pick)),
          ..._ssPools.map((p) => _.pick(p, pick)),
        ].filter((i) => !!i);

        let permissioned = allPoolIdsAndPairs.map((pool) => {
          return new Promise((res, rej) => {
            parachainApi.query.dexRouter
              .dexRoutes(pool.pair?.base, pool.pair?.quote)
              .then((dexRoute) => {
                /**
                 * refactor this later
                 */
                const response: any = dexRoute.toJSON();
                if (
                  response &&
                  response.direct &&
                  response.direct[0] === pool.poolId
                ) {
                  res(pool.poolId);
                }
                res(null);
              })
              .catch((err) => {
                res(null);
              });
          });
        });

        Promise.all(permissioned).then((pools) => {
          const verifiedPoolIds = pools.filter((i) => i !== null) as number[];

          const _lbUnVerifiedpools = _lbpools.filter(
            (pool) => !isVerifiedPool(verifiedPoolIds, pool)
          );
          const _cpUnVerfiedPools = _cpPools.filter(
            (pool) => !isVerifiedPool(verifiedPoolIds, pool)
          );
          const _ssUnverifiedPools = _ssPools.filter(
            (pool) => !isVerifiedPool(verifiedPoolIds, pool)
          );

          const _lbVerifiedpools = _lbpools.filter((pool) =>
            isVerifiedPool(verifiedPoolIds, pool)
          );
          const _cpVerifiedPools = _cpPools.filter((pool) =>
            isVerifiedPool(verifiedPoolIds, pool)
          );
          const _ssVerifiedPools = _ssPools.filter((pool) =>
            isVerifiedPool(verifiedPoolIds, pool)
          );

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


  return null;
};

export default Updater;

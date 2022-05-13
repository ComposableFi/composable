import { useEffect } from "react";
import BigNumber from "bignumber.js";
import { LiquidityBootstrappingPool } from "@/store/pools/liquidityBootstrapping/liquidityBootstrapping.types";
import { ConstantProductPool } from "@/store/pools/constantProduct/constantProduct.types";
import useStore from "@/store/useStore";
import { percentageToNumber } from "@/utils/number";
import { getAssetById } from "@/defi/polkadot/Assets";
import { useParachainApi } from "substrate-react";
import { retrieveSpotPrice } from "../swaps/utils";

const AVERAGE_BLOCK_TIME = 20 * 1000; // Average 20 seconds block time
const DEFAULT_NETWORK_ID = "picasso";
const dummyAuctionLaunchDescrition = {
  paragraphs: [
    "PICA Protocol aims to enable developers in the Polkadot ecosystem. \
    The Picasso protocol introduces DeFi 2.0, moving the industry a massive step closer towards the latter. \
    The LBP token event will be held for 48 hours, starting from 8 February 2022, 14:00 UTC.",
    "PICA tokens purchased in the LBP are the only ones without a lockup. \
    All other parties are subject to a minimum block by block vesting of 1 year, \
    making the LBP investors the only ones able to participate in PICA or LP staking.",
  ],
};

const stringToBigNumber = (value: string): BigNumber =>
  new BigNumber(value.replaceAll(",", ""));

const decodeLbp = (
  poolItem: any,
  poolIndex: number,
  currentBlock: BigNumber
): LiquidityBootstrappingPool => {
  const startBlock = stringToBigNumber(poolItem.sale.start as string);
  const endBlock = stringToBigNumber(poolItem.sale.end as string);

  const start = currentBlock.gt(startBlock)
    ? Date.now() - startBlock.toNumber() * AVERAGE_BLOCK_TIME
    : Date.now() + startBlock.toNumber() * AVERAGE_BLOCK_TIME;
  const end = currentBlock.gt(endBlock)
    ? Date.now() - endBlock.toNumber() * AVERAGE_BLOCK_TIME
    : Date.now() + endBlock.toNumber() * AVERAGE_BLOCK_TIME;
  const duration = Math.round((end - start) / (1000 * 60 * 24 * 24));

  const baseAssetId = Number(
    (poolItem.pair.base as string).replaceAll(",", "")
  );
  const quoteAssetId = Number(
    (poolItem.pair.quote as string).replaceAll(",", "")
  );

  const baseAsset = getAssetById("picasso", baseAssetId);
  const quoteAsset = getAssetById("picasso", quoteAssetId);
  let poolId = `${baseAsset?.symbol.toLowerCase()}-${quoteAsset?.symbol.toLowerCase()}-${poolIndex}`;
  const icon = baseAsset ? baseAsset.icon : quoteAsset ? quoteAsset.icon : "-";

  return {
    id: poolId,
    poolId: poolIndex,
    networkId: DEFAULT_NETWORK_ID,
    icon,
    owner: poolItem.owner,
    pair: {
      base: baseAssetId,
      quote: quoteAssetId,
    },
    sale: {
      start,
      end,
      duration,
      initialWeight: percentageToNumber(poolItem.sale.initialWeight),
      finalWeight: percentageToNumber(poolItem.sale.finalWeight),
    },
    spotPrice: "0",
    fee: poolItem.fee.replace("%", ""),
    history: [],
    auctionDescription: dummyAuctionLaunchDescrition.paragraphs,
  } as LiquidityBootstrappingPool;
};

const decodeCpp = (
  pool: any,
  poolId: number
): ConstantProductPool => {
  return {
    poolId,
    owner: pool.owner,
    pair: {
      base: stringToBigNumber(pool.pair.base).toNumber(),
      quote: stringToBigNumber(pool.pair.quote).toNumber(),
    },
    lpToken: stringToBigNumber(pool.lpToken).toString(),
    fee: pool.fee.replace("%", ""),
    ownerFee: pool.ownerFee,
  };
};

/**
 * Updates zustand store with all pools from pablo pallet
 * @returns null
 */
const Updater = () => {
  const {
    putLBPSpotPrice,
    liquidityBootstrappingPools,
    putLBPList,
    putConstantProductPools,
  } = useStore();
  const { parachainApi } = useParachainApi("picasso");

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
          _cpPools: ConstantProductPool[] = [];

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
            decodedPool = decodeCpp(
              decodedPool.ConstantProduct,
              index
            );
            _cpPools.push(decodedPool);
          }
        });

        const allPools = _lbpools.map(p => { return { pair: p.pair, id: p.poolId }}).concat(_cpPools.map(p => { return { pair: p.pair, id: p.poolId }}))
        let permissioned = allPools.concat().map((pool) => {
          return new Promise((res, rej) => {
            parachainApi.query.dexRouter.dexRoutes(pool.pair.base, pool.pair.quote).then((dexRoute) => {
              const response: any = dexRoute.toJSON();
              if (response && response.direct) {
                res(pool.id)
              }
              res(null)
            }).catch(err => {
              res(null)
            })
          })
        });


        Promise.all(permissioned).then(pools => {
          pools = pools.filter(i => !!i);

          _lbpools = _lbpools.filter((pool) => {
            return pools.find(p => {
              return p === pool.poolId
            }) === undefined
          });

          _cpPools = _cpPools.filter((pool) => {
            return pools.find(p => p === pool.poolId) === undefined
          });

          putLBPList(_lbpools);
          putConstantProductPools(_cpPools);

        });
      });
    }
  }, [parachainApi]);

  useEffect(() => {
    if (parachainApi && liquidityBootstrappingPools.list.length > 0) {
      for (
        let pool = 0;
        pool < liquidityBootstrappingPools.list.length;
        pool++
      ) {
        retrieveSpotPrice(
          parachainApi,
          liquidityBootstrappingPools.list[pool].pair,
          liquidityBootstrappingPools.list[pool].poolId
        ).then((spotPrice) => {
          putLBPSpotPrice(spotPrice.toFixed(4), pool);
        });
      }
    }
  }, [parachainApi, liquidityBootstrappingPools.list.length]);

  return null;
};

export default Updater;

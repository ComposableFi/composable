import { AssetMetadata } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import BigNumber from "bignumber.js";
import { useEffect, useState } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { fetchBalanceByAssetId } from "../../updaters/balances/utils";
import { DEFAULT_NETWORK_ID } from "../../updaters/constants";
import { createPoolAccountId } from "../../updaters/utils";
import { DailyRewards } from "../poolStats/poolStats.types";
import { useLiquidityPoolsList } from "./useLiquidityPoolsList";

export const useLiquidityPoolsWithOpenPositions = (): {
    poolId: number;
    baseAsset: AssetMetadata;
    quoteAsset: AssetMetadata;
    volume: BigNumber;
    apr: BigNumber;
    tvl: BigNumber;
    dailyRewards: DailyRewards[];
    lpAssetId: string
  }[] => {
    const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
    const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
    const allPools = useLiquidityPoolsList();
    const [openPoisitionPoolIds, setOpenPositionsPoolIds] = useState<number[]>([]);
  
    useEffect(() => {
      if (parachainApi && selectedAccount && allPools.length) {
        allPools.map(i => {
          fetchBalanceByAssetId(
            parachainApi,
            DEFAULT_NETWORK_ID,
            selectedAccount.address,
            i.lpAssetId
          ).then((balance) => {
            if (new BigNumber(balance).gt(0)) {
              setOpenPositionsPoolIds([... openPoisitionPoolIds, i.poolId])
            } else {
              setOpenPositionsPoolIds(openPoisitionPoolIds.filter(poolId => poolId !== i.poolId))
            }
          })
        })
      } else {
        setOpenPositionsPoolIds([]);
      }
    }, [selectedAccount, allPools, allPools])

    return allPools.filter(pool => openPoisitionPoolIds.includes(pool.poolId))
  };
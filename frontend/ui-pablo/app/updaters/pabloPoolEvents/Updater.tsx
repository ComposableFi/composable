import { useEffect } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";
import useStore from "@/store/useStore";
import { getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { DEFAULT_NETWORK_ID } from "../constants";
import BigNumber from "bignumber.js";

let userEventsSubscription: any;
const Updater = () => {
  const { parachainApi } = useParachainApi("picasso");
  const selectedAccount = useSelectedAccount("picasso");
  const allLpRewardingPools = useAllLpTokenRewardingPools();
  const {
      updateUserProvidedTokenAmountInLiquidityPool
  } = useStore();

  useEffect(() => {
    if (parachainApi && selectedAccount && allLpRewardingPools.length) {
        console.log('Adding Pool Events User Listener for: ', selectedAccount.address, ' Pools: ', allLpRewardingPools.length)
    //   parachainApi.query.system
    //     .events((blockEvents) => {
    //       const liquidityEvents = blockEvents.filter(
    //         (blockevent) =>
    //           parachainApi.events.pablo.LiquidityAdded.is(blockevent.event) ||
    //           parachainApi.events.pablo.LiquidityRemoved.is(blockevent.event)
    //       ).filter(blockevent => selectedAccount.address === (blockevent.event.data.toJSON() as any)[0]);
          /**
           * Account id who added liquidity.
           * who: T::AccountId,
           * Pool id to which liquidity added.
           * pool_id: T::PoolId,
           * Amount of base asset deposited.
           * base_amount: T::Balance,
           * Amount of quote asset deposited.
           * quote_amount: T::Balance,
           * Amount of minted lp.
           * minted_lp: T::Balance,
           */
          /**
           * Account id who removed liquidity.
           * who: T::AccountId,
           * Pool id to which liquidity added.
           * pool_id: T::PoolId,
           * Amount of base asset removed from pool.
           * base_amount: T::Balance,
           * Amount of quote asset removed from pool.
           * quote_amount: T::Balance,
           * Updated lp token supply.
           * total_issuance: T::Balance,
           */
        //   liquidityEvents.map((i) => {
        //     let eventType = "LIQUIDITY_ADDED";
        //     if (parachainApi.events.pablo.LiquidityRemoved.is(i.event)) {
        //         eventType = "LIQUIDITY_REMOVED";
        //     }

        //     const data: string[] = i.event.data.toJSON() as string[];
        //     const poolId = Number(data[1]);
        //     const pool = allLpRewardingPools.find(p => p.poolId === poolId);

        //     if (pool) {
        //         const baseAsset = getAssetByOnChainId(DEFAULT_NETWORK_ID, pool.pair.base);
        //         const quoteAsset = getAssetByOnChainId(DEFAULT_NETWORK_ID, pool.pair.base);
        //         const baseDecimals = new BigNumber(10).pow(baseAsset.decimals)
        //         const quoteDecimals = new BigNumber(10).pow(baseAsset.decimals)

        //         if (baseAsset) {
        //             let amount = new BigNumber(data[2]).div(baseDecimals);
        //             if (eventType === "LIQUIDITY_REMOVED") amount = amount.times(-1)
        //             updateUserProvidedTokenAmountInLiquidityPool(
        //                 poolId,
        //                 {
        //                     baseAmount: amount.toString()
        //                 }
        //             )
        //         }
        //         if (quoteAsset) {
        //             let amount = new BigNumber(data[2]).div(quoteDecimals);
        //             if (eventType === "LIQUIDITY_REMOVED") amount = amount.times(-1)
        //             updateUserProvidedTokenAmountInLiquidityPool(
        //                 poolId,
        //                 {
        //                     quoteAmount: amount.toString()
        //                 }
        //             )
        //         }
        //     }

        //   });
        // })
        // .then((subscription) => {
        //   userEventsSubscription = subscription;
        // });

      return function () {
        if (userEventsSubscription) {
          console.log("Clearing Subscription");
          userEventsSubscription();
        }
      };
    }
  }, [parachainApi, selectedAccount, allLpRewardingPools.length]);

  return null;
};

export default Updater;

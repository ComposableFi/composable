import { useEffect } from "react";
import useStore from "@/store/useStore";
import { useParachainApi } from "substrate-react";
import { AssetId } from "@/defi/polkadot/types";
import { Assets, getAssetOnChainId } from "@/defi/polkadot/Assets";
import BigNumber from "bignumber.js";
import { LiquidityPoolType } from "@/store/swaps/swaps.types";
import { LiquidityBootstrappingPool } from "@/store/pools/liquidityBootstrapping/liquidityBootstrapping.types";
import { ConstantProductPool } from "@/store/pools/constantProduct/constantProduct.types";
import { retrieveAssetBalance } from "../balances/Updater";
import {
  subsquidClient,
  subsquidLiquidityPool24HourOldQuoteAssetPrice,
  subsquidLiquidityPoolChartPricesQuery,
} from "@/subsquid";
import { createChartSeries, retrieveSpotPrice } from "./utils";

function transformSwapSubsquidTx(
  subsquidSwapTxs: {
    baseAssetId: string;
    quoteAssetId: string;
    receivedTimestamp: string;
    spotPrice: string;
  }[],
  selectedQuote: number
): {
  baseAssetId: number;
  quoteAssetId: number;
  receivedTimestamp: number;
  spotPrice: string;
}[] {
  return subsquidSwapTxs.map((tx) => {
    let spotPrice = new BigNumber(tx.spotPrice);
    if (Number(tx.quoteAssetId) !== selectedQuote) {
      spotPrice = new BigNumber(1).div(spotPrice)
    }

    return {
      baseAssetId: Number(tx.baseAssetId),
      quoteAssetId: Number(tx.quoteAssetId),
      receivedTimestamp: Number(tx.receivedTimestamp),
      spotPrice: spotPrice.toString(),
    };
  });
}

/* See task https://app.clickup.com/t/2u9un3m
 * how to create AccountId for derived Accounts
 * within a pallet
 */
export function createPoolAccountId(poolId: number): string {
  enum PalletIds {
    PalletsId = "0x6d6f646c",
    Pablo = "70616c6c5f706162",
  }

  return (
    PalletIds.PalletsId.toString() +
    PalletIds.Pablo.toString() +
    Number(poolId).toString(16) +
    new Array(50).fill("0").join("")
  ).substring(0, 66);
}

function isValidAssetPairFromUi(
  assetId1: AssetId | "none",
  assetId2: AssetId | "none"
) {
  return assetId1 !== "none" && assetId2 !== "none";
}
/**
 * This updater is used for
 * Swaps page, we update values needed
 * by swaps page here to complete UX of
 * swaps page
 * @returns null
 */
const Updater = () => {
  const {
    assets,
    swaps,
    swapsChart,
    setDexRouteSwaps,
    setPoolConstantsSwaps,
    setUserAccountBalanceSwaps,
    constantProductPools,
    setPoolVariablesSwaps,
    liquidityBootstrappingPools,
    putSwapsChartSeries,
    put24HourOldPrice
  } = useStore();
  const { parachainApi } = useParachainApi("picasso");
  /**
   * Triggered when user changes first
   * token from token list dropdown on
   * swaps page
   * Updates with balance from zustand store
   */
  useEffect(() => {
    const { ui } = swaps;
    if (ui.quoteAssetSelected === "none") {
      setUserAccountBalanceSwaps("quote", "0");
    } else {
      const balance = assets[ui.quoteAssetSelected as AssetId].balance.picasso;
      setUserAccountBalanceSwaps("quote", balance);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swaps.ui, assets]);
  /**
   * Triggered when user changes second
   * token from token list dropdown on
   * swaps page
   * Updates with balance from zustand store
   */
  useEffect(() => {
    const { ui } = swaps;
    if (ui.baseAssetSelected === "none") {
      setUserAccountBalanceSwaps("base", "0");
    } else {
      const balance = assets[ui.baseAssetSelected as AssetId].balance.picasso;
      setUserAccountBalanceSwaps("base", balance);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [swaps.ui, assets]);
  /**
   * This hook is triggered when all
   * pools are fetched from the pablo pallet
   *
   * It is responsible for two things
   * 1. Fetch and check permissioned route for
   *    selected asset pair in UI
   * 2. Update swaps zustand store with pool constants
   *    fetched from the chain
   */
  useEffect(() => {
    const { ui } = swaps;
    if (isValidAssetPairFromUi(ui.baseAssetSelected, ui.quoteAssetSelected)) {
      if (
        parachainApi &&
        (liquidityBootstrappingPools.list.length ||
          constantProductPools.list.length)
      ) {
        // Convert to on chain Asset Ids from string Asset Ids
        const _baseAssetId =
          Assets[ui.baseAssetSelected as AssetId].supportedNetwork.picasso;
        const _quoteAssetId =
          Assets[ui.quoteAssetSelected as AssetId].supportedNetwork.picasso;
        // Fetch a permissioned route, dexRoutes do not
        // support inverted routes so we query both
        // possibilites
        const routePromises = [
          parachainApi.query.dexRouter.dexRoutes(_baseAssetId, _quoteAssetId),
          parachainApi.query.dexRouter.dexRoutes(_quoteAssetId, _baseAssetId),
        ];

        Promise.all(routePromises).then(
          ([baseToQuoteRoute, quoteToBaseRoute]) => {
            let baseToQuoteRouteJSON = baseToQuoteRoute.toJSON();
            let quoteToBaseRouteJSON = quoteToBaseRoute.toJSON();

            let dexRoute: any = null;
            if (!!baseToQuoteRouteJSON) dexRoute = baseToQuoteRouteJSON;
            if (!!quoteToBaseRouteJSON) dexRoute = quoteToBaseRouteJSON;

            if (dexRoute === null) {
              // Clear Data here as
              // no permissioned route
              // was found
              setDexRouteSwaps([]);
              setPoolConstantsSwaps({
                poolAccountId: "",
                poolIndex: -1,
                lbpConstants: undefined,
                poolType: "none",
                fee: "0",
                pair: { quote: -1, base: -1 },
              });
            } else if (dexRoute.direct) {
              // found a route, involves a
              // single pool which is why "direct"
              let poolType: LiquidityPoolType | "none" = "none";
              let pair = { quote: -1, base: -1 };
              let poolId = dexRoute.direct[0];
              poolId = poolId.toString();

              let pool:
                | LiquidityBootstrappingPool
                | ConstantProductPool
                | undefined = liquidityBootstrappingPools.list.find(
                (pool) => pool.poolId.toString() === poolId
              );

              if (!pool) {
                pool = constantProductPools.list.find(
                  (pool) => pool.poolId.toString() === poolId
                );
              }
              if (pool && (pool as LiquidityBootstrappingPool).sale) {
                poolType = "Balancer";
                pair = pool.pair;
              }
              // change the line below to better check
              // pools type
              if (pool && !(pool as any).sale) {
                poolType = "Uniswap";
                pair = pool.pair;
              }

              if (pool) {
                let poolAccountId = createPoolAccountId(poolId as number);

                let lbp = undefined;
                let fee = new BigNumber(pool.fee);
                if ((pool as LiquidityBootstrappingPool).sale) {
                  lbp = {
                    start: (pool as LiquidityBootstrappingPool).sale.start,
                    end: (pool as LiquidityBootstrappingPool).sale.end,
                    initialWeight: (
                      pool as LiquidityBootstrappingPool
                    ).sale.initialWeight.toString(),
                    finalWeight: (
                      pool as LiquidityBootstrappingPool
                    ).sale.finalWeight.toString(),
                  };
                }
                if ((pool as ConstantProductPool).ownerFee) {
                  fee.plus((pool as ConstantProductPool).ownerFee);
                }

                let poolConstants = {
                  poolAccountId: poolAccountId,
                  poolIndex: Number(poolId),
                  fee: fee.toString(),
                  lbpConstants: lbp,
                  poolType,
                  pair,
                };

                setDexRouteSwaps([Number(poolId)]);
                setPoolConstantsSwaps(poolConstants);
              }
            } else {
              // Some future logic
            }
          }
        );
      }
    }
  }, [
    swaps.ui,
    parachainApi,
    liquidityBootstrappingPools.list.length,
    constantProductPools.list.length,
  ]);

  useEffect(() => {
    if (swaps.poolConstants.poolIndex !== -1) {
      const { poolAccountId, poolIndex, pair } = swaps.poolConstants;
      const { baseAssetSelected, quoteAssetSelected } = swaps.ui;

      if (
        isValidAssetPairFromUi(baseAssetSelected, quoteAssetSelected) &&
        parachainApi
      ) {
        const _baseAssetId =
          Assets[baseAssetSelected as AssetId].supportedNetwork.picasso;
        const _quoteAssetId =
          Assets[quoteAssetSelected as AssetId].supportedNetwork.picasso;

        const isReversedTrade = pair.base === _baseAssetId;

        if (_baseAssetId && _quoteAssetId) {
          let promises = [
            retrieveAssetBalance(
              parachainApi,
              "picasso",
              poolAccountId,
              _baseAssetId.toString()
            ),
            retrieveAssetBalance(
              parachainApi,
              "picasso",
              poolAccountId,
              _quoteAssetId.toString()
            ),
            retrieveSpotPrice(parachainApi, swaps.poolConstants.pair, swaps.poolConstants.poolIndex),
            subsquidClient
              .query(
                subsquidLiquidityPool24HourOldQuoteAssetPrice(
                  swaps.poolConstants.poolIndex,
                  _quoteAssetId,
                  1
                )
              )
              .toPromise(),
          ];

          Promise.all(promises).then(
            ([
              baseAssetBalance,
              quoteAssetBalance,
              spotPrice,
              priceChangeResponse,
            ]) => {
              if (isReversedTrade) {
                spotPrice = new BigNumber(1).div(spotPrice as BigNumber);
              }

              setPoolVariablesSwaps({
                spotPrice: (spotPrice as BigNumber).toFixed(4),
                baseAssetReserve: baseAssetBalance as string,
                quoteAssetReserve: quoteAssetBalance as string,
              });

              if ((priceChangeResponse as any).data && (priceChangeResponse as any).data.pabloTransactions) {
                let pc = new BigNumber(0);
                if ((priceChangeResponse as any).data.pabloTransactions[0]) {
                  pc = new BigNumber((priceChangeResponse as any).data.pabloTransactions[0].spotPrice)
                }
                put24HourOldPrice(pc.toString())
              }
            }
          );
        }
      }
    } else {
      setPoolVariablesSwaps({
        spotPrice: "0",
        baseAssetReserve: "0",
        quoteAssetReserve: "0",
      });
    }
  }, [swaps.ui, swaps.poolConstants.poolIndex, parachainApi]);

  useEffect(() => {
    const { ui, poolConstants } = swaps;
    if (
      parachainApi &&
      poolConstants.poolIndex !== -1 &&
      isValidAssetPairFromUi(ui.baseAssetSelected, ui.quoteAssetSelected)
    ) {
      const _selectedQuoteAssetId = getAssetOnChainId(
        "picasso",
        ui.quoteAssetSelected as AssetId
      );

      if (_selectedQuoteAssetId) {
        subsquidClient
          .query(
            subsquidLiquidityPoolChartPricesQuery(
              poolConstants.poolIndex,
              "SWAP",
              250
            )
          )
          .toPromise()
          .then((response) => {
            if (
              response.data &&
              response.data.pabloTransactions &&
              response.data.pabloTransactions.length
            ) {
              const txs = transformSwapSubsquidTx(
                response.data.pabloTransactions,
                _selectedQuoteAssetId
              );

              putSwapsChartSeries(
                createChartSeries(txs, swapsChart.selectedRange)
              );
            } else {
              putSwapsChartSeries([]);
            }
          });
      } else {
        putSwapsChartSeries([]);
      }
    } else {
      putSwapsChartSeries([]);
    }
  }, [
    swaps.ui,
    swaps.poolConstants.poolIndex,
    parachainApi,
    swapsChart.selectedRange,
  ]);

  return null;
};

export default Updater;

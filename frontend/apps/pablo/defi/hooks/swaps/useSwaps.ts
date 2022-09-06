import { Option } from "@/components/types";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import {
  calculator,
  DEFAULT_NETWORK_ID,
  fetchSpotPrice,
  isValidAssetPair,
  stableSwapCalculator,
} from "@/defi/utils";
import { useAppSelector } from "@/hooks/store";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import { usePrevious } from "@/hooks/usePrevious";
import { MockedAsset } from "@/store/assets/assets.types";
import { useAssetBalance, useUSDPriceByAssetId } from "@/store/assets/hooks";
import { useLiquidityByPool } from "@/store/hooks/useLiquidityByPool";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useSnackbar } from "notistack";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useParachainApi } from "substrate-react";
import { useAsset } from "../assets/useAsset";
import { useFilteredAssetListDropdownOptions } from "../assets/useFilteredAssetListDropdownOptions";
import { usePriceImpact } from "./usePriceImpact";

export function useSwaps(): {
  balance1: BigNumber;
  balance2: BigNumber;
  changeAsset: (side: "base" | "quote", asset: string | "none") => void;
  selectedAssetOneId: string | "none";
  selectedAssetTwoId: string | "none";
  selectedAssetOne: MockedAsset | undefined;
  selectedAssetTwo: MockedAsset | undefined;
  assetListOne: Option[];
  assetListTwo: Option[];
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
  dexRoute: BigNumber | null;
  onChangeTokenAmount: (
    sideChange: "base" | "quote",
    amount: BigNumber
  ) => void;
  flipAssetSelection: () => void;
  updateSpotPrice: () => void;
  pabloPool: ConstantProductPool | StableSwapPool | undefined;
  minimumReceived: BigNumber;
  slippageAmount: BigNumber;
  feeCharged: BigNumber;
  spotPrice: BigNumber;
  valid: boolean;
  percentageToSwap: number;
  asset1PriceUsd: BigNumber;
  asset2PriceUsd: BigNumber;
  setAssetOneInputValid: (validity: boolean) => void;
  setAssetTwoInputValid: (validity: boolean) => void;
  assetOneInputValid: boolean;
  assetTwoInputValid: boolean;
  isProcessing: boolean;
  priceImpact: BigNumber;
} {
  const slippage = useAppSelector(
    (state) => state.settings.transactionSettings.tolerance
  );
  const previousSlippage = usePrevious(slippage);

  const [assetOneInputValid, setAssetOneInputValid] = useState(true);
  const [assetTwoInputValid, setAssetTwoInputValid] = useState(true);
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { enqueueSnackbar } = useSnackbar();

  const {
    swaps,
    pools: { constantProductPools, stableSwapPools },
  } = useStore();
  const {
    tokenAmounts: { assetOneAmount, assetTwoAmount },
    setTokenAmounts,
    spotPrice,
    selectedAssets,
    selectedPool,
    setSpotPrice,
    setSelectedAsset,
    setSelectedPool,
    resetSwaps,
    flipAssetSelection,
  } = swaps;

  useEffect(() => {
    return () => {
      resetSwaps();
    };
  }, [resetSwaps]);

  const { selectedAssetOneId, selectedAssetTwoId } = useMemo(() => {
    return {
      selectedAssetOneId: selectedAssets.quote,
      selectedAssetTwoId: selectedAssets.base,
    };
  }, [selectedAssets]);

  const setSelectedAssetOne = (id: string | "none") => {
    setSelectedAsset(id, "quote");
  };

  const setSelectedAssetTwo = (id: string | "none") => {
    setSelectedAsset(id, "base");
  };

  const asset1PriceUsd = useUSDPriceByAssetId(selectedAssetOneId);
  const asset2PriceUsd = useUSDPriceByAssetId(selectedAssetTwoId);

  const selectedAssetOne = useAsset(selectedAssetOneId);
  const selectedAssetTwo = useAsset(selectedAssetTwoId);

  const assetListOne = useFilteredAssetListDropdownOptions(selectedAssetTwoId);
  const assetListTwo = useFilteredAssetListDropdownOptions(selectedAssetOneId);

  const balance1 = useAssetBalance(DEFAULT_NETWORK_ID, selectedAssetOneId);
  const balance2 = useAssetBalance(DEFAULT_NETWORK_ID, selectedAssetTwoId);

  const fetchDexRoute = useCallback(async (): Promise<BigNumber | null> => {
    if (
      parachainApi &&
      isValidAssetPair(selectedAssetOneId, selectedAssetTwoId)
    ) {
      const routePromises = [
        parachainApi.query.dexRouter.dexRoutes(
          selectedAssetOneId,
          selectedAssetTwoId
        ),
        parachainApi.query.dexRouter.dexRoutes(
          selectedAssetTwoId,
          selectedAssetOneId
        ),
      ];
      const dexRoutes = await Promise.all(routePromises);
      const [straightRouteResponse, inverseRouteResponse] = dexRoutes;
      let straightRoute = straightRouteResponse.toJSON();
      let inverseRoute = inverseRouteResponse.toJSON();

      let dexRoute: any = null;
      if (!!straightRoute) dexRoute = straightRoute;
      if (!!inverseRoute) dexRoute = inverseRoute;

      if (dexRoute && dexRoute.direct) {
        return new BigNumber(dexRoute.direct[0]);
      } else {
        return dexRoute;
      }
    }
    return null;
  }, [selectedAssetOneId, selectedAssetTwoId, parachainApi]);

  const [dexRoute, setDexRoute] = useState<BigNumber | null>(null);
  useAsyncEffect(async () => {
    const dexRoute = await fetchDexRoute();
    if (selectedPool && dexRoute) {
      if (selectedPool.poolId === dexRoute.toNumber()) {
        // no need to set the route again if it's the same
        return;
      }
      setDexRoute(dexRoute);
    }
    setDexRoute(dexRoute);
  }, [fetchDexRoute, selectedPool]);

  useEffect(() => {
    if (!dexRoute) {
      return setSelectedPool(undefined);
    }

    const verifiedConstantProductPools = constantProductPools.verified;
    const verifiedStableSwapPools = stableSwapPools.verified;

    let lpToTrade: StableSwapPool | ConstantProductPool | undefined = undefined;
    lpToTrade = verifiedConstantProductPools.find(
      (i) => i.poolId === dexRoute.toNumber()
    );
    if (!lpToTrade)
      lpToTrade = verifiedStableSwapPools.find(
        (i) => i.poolId === dexRoute.toNumber()
      );

    setSelectedPool(lpToTrade);
  }, [dexRoute, constantProductPools, stableSwapPools, setSelectedPool]);

  const updateSpotPrice = useCallback(async () => {
    if (parachainApi && selectedPool) {
      const { base, quote } = selectedPool.pair;
      const isInverse = selectedAssetOneId === base.toString();
      let pair = { base: base.toString(), quote: quote.toString() };
      const spotPrice = await fetchSpotPrice(
        parachainApi,
        pair,
        selectedPool.poolId
      );
      if (isInverse) {
        setSpotPrice(new BigNumber(1).div(spotPrice).dp(4));
      } else {
        setSpotPrice(spotPrice.dp(4));
      }
    } else {
      setSpotPrice(new BigNumber(0));
    }
  }, [parachainApi, selectedPool, selectedAssetOneId, setSpotPrice]);

  useEffect(() => {
    if (selectedPool) {
      updateSpotPrice();
    }
  }, [selectedPool, updateSpotPrice]);

  const [minimumReceived, setMinimumReceived] = useState(new BigNumber(0));
  const [slippageAmount, setSlippageAmount] = useState(new BigNumber(0));
  const [feeCharged, setFeeCharged] = useState(new BigNumber(0));

  const resetTokenAmounts = useCallback(() => {
    setTokenAmounts({
      assetOneAmount: new BigNumber(0),
      assetTwoAmount: new BigNumber(0),
    });
  }, [setTokenAmounts]);

  const [isProcessing, setIsProcessing] = useState(false);

  const unsetProcessingDelayed = () => {
    setTimeout(() => {
      setIsProcessing(false);
    }, 500);
  };

  const onChangeTokenAmount = async (
    changedSide: "base" | "quote",
    amount: BigNumber
  ) => {
    try {
      setIsProcessing(true);
      if (
        parachainApi &&
        selectedPool &&
        isValidAssetPair(selectedAssetOneId, selectedAssetTwoId)
      ) {
        const spotPrice = await fetchSpotPrice(
          parachainApi,
          { base: selectedAssetTwoId, quote: selectedAssetOneId },
          selectedPool.poolId
        );

        const { feeRate } = selectedPool.feeConfig;
        let feePercentage = new BigNumber(feeRate).toNumber();

        const { minReceive, tokenOutAmount, feeChargedAmount, slippageAmount } =
          "baseWeight" in selectedPool
            ? calculator(
                changedSide,
                amount,
                spotPrice,
                slippage,
                feePercentage
              )
            : stableSwapCalculator(
                changedSide,
                amount,
                spotPrice,
                slippage,
                feePercentage
              );

        if (changedSide === "base" && tokenOutAmount.gt(balance1)) {
          throw new Error("Insufficient balance.");
        }

        setTokenAmounts({
          assetOneAmount: changedSide === "base" ? tokenOutAmount : amount,
          assetTwoAmount: changedSide === "quote" ? tokenOutAmount : amount,
        });
        setMinimumReceived(minReceive);
        setFeeCharged(feeChargedAmount);
        setSlippageAmount(slippageAmount);
      } else {
        throw new Error("Pool not found.");
      }
    } catch (err: any) {
      resetTokenAmounts();
      console.error(err.message);
      enqueueSnackbar(err.message);
    } finally {
      unsetProcessingDelayed();
    }
  };

  const {
    tokenAmounts: { baseAmount, quoteAmount },
  } = useLiquidityByPool(selectedPool);
  let poolQuoteBalance = selectedPool
    ? selectedPool.pair.quote.toString() === selectedAssetOneId
      ? quoteAmount
      : baseAmount
    : new BigNumber(0);
  let poolBaseBalance = selectedPool
    ? selectedPool.pair.quote.toString() === selectedAssetOneId
      ? baseAmount
      : quoteAmount
    : new BigNumber(0);
  const priceImpact = usePriceImpact({
    tokenInAmount: assetOneAmount,
    tokenOutAmount: assetTwoAmount,
    isConstantProductPool: selectedPool ? "baseWeight" in selectedPool : false,
    baseWeight:
      selectedPool && "baseWeight" in selectedPool
        ? new BigNumber(selectedPool.baseWeight)
        : new BigNumber(0),
    quoteBalance: poolQuoteBalance,
    baseBalance: poolBaseBalance,
    amplificationCoefficient: selectedPool && "amplificationCoefficient" in selectedPool ? new BigNumber(selectedPool.amplificationCoefficient) : new BigNumber(0)
  });

  /**
   * Effect to update minimum received when
   * there is a change in slippage
   */
  useEffect(() => {
    if (parachainApi && selectedPool) {
      if (previousSlippage != slippage) {
        if (minimumReceived.gt(0)) {
          const { feeRate } = selectedPool.feeConfig;
          let feePercentage = new BigNumber(feeRate).toNumber();

          const { minReceive } =
            "baseWeight" in selectedPool
              ? calculator(
                  "quote",
                  assetOneAmount,
                  spotPrice,
                  slippage,
                  feePercentage
                )
              : stableSwapCalculator(
                  "quote",
                  assetOneAmount,
                  spotPrice,
                  slippage,
                  feePercentage
                );
          setMinimumReceived(minReceive);
        }
      }
    }
    return;
  }, [
    spotPrice,
    selectedPool,
    balance1,
    previousSlippage,
    minimumReceived,
    feeCharged,
    slippageAmount,
    slippage,
    parachainApi,
    assetOneAmount,
    assetTwoAmount,
  ]);

  const flipAssets = () => {
    setIsProcessing(true);
    flipAssetSelection();
    unsetProcessingDelayed();
  };

  const changeAsset = (
    changedSide: "quote" | "base",
    tokenId: string | "none"
  ) => {
    setIsProcessing(true);
    changedSide === "quote"
      ? setSelectedAssetOne(tokenId)
      : setSelectedAssetTwo(tokenId);
    resetTokenAmounts();
    unsetProcessingDelayed();
  };

  const valid =
    dexRoute !== null &&
    assetOneInputValid &&
    assetTwoInputValid &&
    !!selectedPool;

  const percentageToSwap = 50;

  return {
    balance1,
    balance2,
    changeAsset,
    selectedAssetOneId,
    selectedAssetTwoId,
    selectedAssetOne,
    selectedAssetTwo,
    assetListOne,
    assetListTwo,
    onChangeTokenAmount,
    updateSpotPrice,
    assetOneAmount,
    assetTwoAmount,
    dexRoute,
    pabloPool: selectedPool,
    minimumReceived,
    slippageAmount,
    feeCharged,
    valid,
    spotPrice,
    asset1PriceUsd,
    asset2PriceUsd,
    setAssetOneInputValid,
    setAssetTwoInputValid,
    assetOneInputValid,
    assetTwoInputValid,
    flipAssetSelection: flipAssets,
    percentageToSwap,
    isProcessing,
    priceImpact
  };
}

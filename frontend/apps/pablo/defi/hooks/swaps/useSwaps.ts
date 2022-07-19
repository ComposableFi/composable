import { Option } from "@/components/types";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import {
  DEFAULT_NETWORK_ID,
  fetchSpotPrice,
  isValidAssetPair,
  uniswapCalculator,
} from "@/defi/utils";
import { useAppSelector } from "@/hooks/store";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import { MockedAsset } from "@/store/assets/assets.types";
import { useAssetBalance, useUSDPriceByAssetId } from "@/store/assets/hooks";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useSnackbar } from "notistack";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useParachainApi } from "substrate-react";
import { useAsset } from "../assets/useAsset";
import { useFilteredAssetListDropdownOptions } from "../assets/useFilteredAssetListDropdownOptions";

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
  asset1PriceUsd: BigNumber;
  asset2PriceUsd: BigNumber;
  setAssetOneInputValid: (validity: boolean) => void;
  setAssetTwoInputValid: (validity: boolean) => void;
  assetOneInputValid: boolean;
  assetTwoInputValid: boolean;
} {
  const slippage = useAppSelector(
    (state) => state.settings.transactionSettings.tolerance
  );

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
    if (!dexRoute) { return setSelectedPool(undefined); }

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
            assetOneAmount:  new BigNumber(0),
            assetTwoAmount: new BigNumber(0)
        })
}, [setTokenAmounts])

  const onChangeTokenAmount = async (
    changedSide: "base" | "quote",
    amount: BigNumber
  ) => {
    if (
      parachainApi &&
      selectedPool &&
      isValidAssetPair(selectedAssetOneId, selectedAssetTwoId)
    ) {
      const { base, quote } = selectedPool.pair;
      const { feeRate } = selectedPool.feeConfig;
      let feePercentage = new BigNumber(feeRate).toNumber();
      const isInverse = selectedAssetOneId === base.toString();
      let pair = { base: base.toString(), quote: quote.toString() };

      const oneBaseInQuote = await fetchSpotPrice(
        parachainApi,
        pair,
        selectedPool.poolId
      );
      const { minReceive, tokenOutAmount, feeChargedAmount, slippageAmount } =
        uniswapCalculator(
          changedSide,
          isInverse,
          amount,
          oneBaseInQuote,
          slippage,
          feePercentage
        );

      setTokenAmounts({
        assetOneAmount: changedSide === "base" ? amount : tokenOutAmount,
        assetTwoAmount: changedSide === "quote" ? amount : tokenOutAmount,
      });
      setMinimumReceived(minReceive);
      setFeeCharged(feeChargedAmount);
      setSlippageAmount(slippageAmount);
      return {
        minReceive,
        tokenOutAmount,
        feeCharged,
        slippageAmount,
      };
    } else {
        resetTokenAmounts();
      console.error(`Registered Pool not found`);
      enqueueSnackbar(`Registered Pool not found`);
      return {
        minReceive: new BigNumber(0),
        tokenOutAmount: new BigNumber(0),
        feeCharged: new BigNumber(0),
        slippageAmount: new BigNumber(0),
      };
    }
  };

  const changeAsset = (
    changedSide: "quote" | "base",
    tokenId: string | "none"
  ) => {
    changedSide === "quote"
      ? setSelectedAssetOne(tokenId)
      : setSelectedAssetTwo(tokenId);
    resetTokenAmounts();
  };

  const valid =
    dexRoute !== null &&
    assetOneInputValid &&
    assetTwoInputValid &&
    !!selectedPool;

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
    flipAssetSelection,
  };
}

import { Asset, DualAssetConstantProduct } from "shared";
import { Option } from "@/components/types";
import {
  calculator,
  DEFAULT_NETWORK_ID,
} from "@/defi/utils";
import { usePrevious } from "@/hooks/usePrevious";
import { useAppSettingsSlice } from "@/store/appSettings/slice";
import { useAssetBalance, useAssetIdOraclePrice } from "@/defi/hooks";
import { Dispatch, SetStateAction, useCallback, useEffect, useMemo, useState } from "react";
import { useParachainApi } from "substrate-react";
import { useAsset } from "../assets/useAsset";
import { useFilteredAssetListDropdownOptions } from "../assets/useFilteredAssetListDropdownOptions";
import { usePriceImpact } from "./usePriceImpact";
import { useLiquidity } from "../useLiquidity";
import { usePoolsSlice } from "@/store/pools/pools.slice";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

export function useSwaps({ selectedAccount }: { selectedAccount?: InjectedAccountWithMeta; }): {
  balance1: BigNumber;
  balance2: BigNumber;
  changeAsset: (side: "base" | "quote", asset: string | "none") => void;
  selectedAssetOneId: string | "none";
  selectedAssetTwoId: string | "none";
  selectedAssetOne: Asset | undefined;
  selectedAssetTwo: Asset | undefined;
  assetListOne: Option[];
  assetListTwo: Option[];
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
  onChangeTokenAmount: (
    amount: BigNumber
  ) => void;
  flipAssetSelection: () => void;
  updateSpotPrice: () => void;
  inputMode: 1 | 2,
  setInputMode: Dispatch<SetStateAction<1 | 2>>;
  pabloPool: DualAssetConstantProduct | undefined;
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
  priceImpact: BigNumber;
} {
  const slippage = useAppSettingsSlice().transactionSettings.tolerance;
  const previousSlippage = usePrevious(slippage);

  const [inputMode, setInputMode] = useState<1 | 2>(1);
  const [assetOneInputValid, setAssetOneInputValid] = useState(true);
  const [assetTwoInputValid, setAssetTwoInputValid] = useState(true);
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const {
    swaps,
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

  const selectedAssetOne = useAsset(selectedAssetOneId);
  const selectedAssetTwo = useAsset(selectedAssetTwoId);
  const asset1PriceUsd = useAssetIdOraclePrice(selectedAssetOneId);
  const asset2PriceUsd = useAssetIdOraclePrice(selectedAssetTwoId);
  const balance1 = useAssetBalance(selectedAssetOne, "picasso");
  const balance2 = useAssetBalance(selectedAssetTwo, "picasso");
  const assetListOne = useFilteredAssetListDropdownOptions(selectedAssetOneId);
  const assetListTwo = useFilteredAssetListDropdownOptions(selectedAssetTwoId);

  const updateSpotPrice = useCallback(async () => {
    if (selectedPool) {
      const pair = Object.keys(selectedPool.getAssets().assets);
      const base = pair[0];
      const isInverse = selectedAssetOneId === base;

      const spotPrice = await selectedPool.getSpotPrice(new BigNumber(selectedAssetOneId));
      if (isInverse) {
        setSpotPrice(new BigNumber(1).div(spotPrice));
      } else {
        setSpotPrice(spotPrice);
      }

    } else {
      setSpotPrice(new BigNumber(0));
    }
  }, [selectedPool, selectedAssetOneId, setSpotPrice]);

  useEffect(() => {
    if (selectedPool) {
      updateSpotPrice();
    }
  }, [selectedPool, updateSpotPrice]);

  const { liquidityPools } = usePoolsSlice();
  useEffect(() => {
    if (liquidityPools.length > 0) {
      const pool = liquidityPools.find((_constantPool) => {
        try {
          const pair = Object.keys(_constantPool.getAssets().assets);
          const pairBase = pair[0].toString();
          const pairQuote = pair[1].toString();
  
          return (
            pairBase === selectedAssetOneId && pairQuote === selectedAssetTwoId ||
            pairBase === selectedAssetTwoId && pairQuote === selectedAssetOneId
          )
        } catch (err: any) {
          console.error('[useSwaps] Liquidity Pool not found. ', err.message);
        }
      });
      if (pool) {
        setSelectedPool(pool);
      }
    }
  }, [liquidityPools, selectedAssetOneId, selectedAssetTwoId, setSelectedPool, setSpotPrice])

  const [minimumReceived, setMinimumReceived] = useState(new BigNumber(0));
  const [slippageAmount, setSlippageAmount] = useState(new BigNumber(0));
  const [feeCharged, setFeeCharged] = useState(new BigNumber(0));

  const resetTokenAmounts = useCallback(() => {
    setTokenAmounts({
      assetOneAmount: new BigNumber(0),
      assetTwoAmount: new BigNumber(0),
    });
  }, [setTokenAmounts]);

  const onChangeTokenAmount = (
    amount: BigNumber
  ) => {
    if (selectedPool && amount.gt(0)) {
      const feeRate = selectedPool.getFeeConfig().getFeeRate();
      let feePercentage = feeRate.toNumber();
      const { minReceive, tokenOutAmount, feeChargedAmount, slippageAmount } = calculator(
        inputMode === 1 ? "quote" : "base",
        amount,
        spotPrice,
        slippage,
        feePercentage
      )
      setTokenAmounts({
        assetOneAmount: inputMode === 2 ? tokenOutAmount : amount,
        assetTwoAmount: inputMode === 1 ? tokenOutAmount : amount,
      });
      setMinimumReceived(minReceive);
      setFeeCharged(feeChargedAmount);
      setSlippageAmount(slippageAmount);
    }
  };

  const { baseAmount, quoteAmount } = useLiquidity(selectedPool);
  const poolAssets = selectedPool ? Object.keys(selectedPool.getAssets().assets) : null;
  let poolQuoteBalance = poolAssets ? 
    poolAssets?.[0] === selectedAssetOneId
      ? quoteAmount
      : baseAmount
    : new BigNumber(0);
  let poolBaseBalance = poolAssets
    ? poolAssets?.[1] === selectedAssetOneId
      ? baseAmount
      : quoteAmount
    : new BigNumber(0);

  const priceImpact = usePriceImpact({
    tokenInAmount: assetOneAmount,
    tokenOutAmount: assetTwoAmount,
    isConstantProductPool: selectedPool ? "baseWeight" in selectedPool : false,
    // needs update later
    baseWeight: new BigNumber(0),
    quoteBalance: poolQuoteBalance,
    baseBalance: poolBaseBalance,
    // amplificationCoefficient: selectedPool && "amplificationCoefficient" in selectedPool ? new BigNumber(selectedPool.amplificationCoefficient) : new BigNumber(0)
  });

  /**
   * Effect to update minimum received when
   * there is a change in slippage
   */
  useEffect(() => {
    if (parachainApi && selectedPool) {
      if (previousSlippage != slippage) {
        if (minimumReceived.gt(0)) {
          const feeRate = selectedPool.getFeeConfig().getFeeRate();
          let feePercentage = new BigNumber(feeRate).toNumber();

          if (selectedPool instanceof DualAssetConstantProduct) {
            const { minReceive } =
              calculator(
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
    flipAssetSelection();
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
    assetOneInputValid &&
    assetTwoInputValid &&
    !!selectedPool;

  const percentageToSwap = 50;

  return {
    inputMode,
    setInputMode,
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
    priceImpact
  };
}

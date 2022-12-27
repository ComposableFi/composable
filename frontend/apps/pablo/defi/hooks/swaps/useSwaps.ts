import { Asset } from "shared";
import { Option } from "@/components/types";
import { calculateOutGivenIn, DEFAULT_NETWORK_ID } from "@/defi/utils";
import { usePrevious } from "@/hooks/usePrevious";
import { useAppSettingsSlice } from "@/store/appSettings/slice";
import { useAssetBalance } from "@/defi/hooks";
import {
  Dispatch,
  SetStateAction,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from "react";
import { useParachainApi } from "substrate-react";
import { useAsset } from "../assets/useAsset";
import { usePriceImpact } from "./usePriceImpact";
import { useLiquidity } from "../useLiquidity";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { PoolConfig } from "@/store/pools/types";
import { getOraclePrice } from "@/store/oracle/slice";
import { usePoolSpotPrice } from "@/defi/hooks/pools/usePoolSpotPrice";

export function useSwaps({
  selectedAccount,
}: {
  selectedAccount?: InjectedAccountWithMeta;
}): {
  balance1: BigNumber;
  balance2: BigNumber;
  changeAsset: (side: "base" | "quote", asset: string | "none") => void;
  selectedAssetOneId: string | "none";
  selectedAssetTwoId: string | "none";
  selectedAssetOne: Asset | undefined;
  selectedAssetTwo: Asset | undefined;
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
  onChangeTokenAmount: (amount: BigNumber) => void;
  flipAssetSelection: () => void;
  inputMode: 1 | 2;
  setInputMode: Dispatch<SetStateAction<1 | 2>>;
  pabloPool: PoolConfig | undefined;
  minimumReceived: {
    asset: Asset | undefined;
    value: BigNumber;
  };
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
  assetList: Option[];
} {
  const slippage = useAppSettingsSlice().transactionSettings.tolerance;
  const previousSlippage = usePrevious(slippage);

  const [inputMode, setInputMode] = useState<1 | 2>(1);
  const [assetOneInputValid, setAssetOneInputValid] = useState(true);
  const [assetTwoInputValid, setAssetTwoInputValid] = useState(true);
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const {
    tokenAmounts: { assetOneAmount, assetTwoAmount },
    setTokenAmounts,
    selectedAssets,
    selectedPool,
    setSelectedAsset,
    setSelectedPool,
    resetSwaps,
    flipAssetSelection,
  } = useStore((store) => store.swaps);
  const { baseAmount, quoteAmount, baseAsset, quoteAsset } =
    useLiquidity(selectedPool);

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
  const [asset1PriceUsd, asset2PriceUsd] = useMemo(() => {
    let asset1Price = new BigNumber(0);
    let asset2Price = new BigNumber(0);
    asset1Price = selectedAssetOne
      ? getOraclePrice(selectedAssetOne.getSymbol(), "coingecko", "usd")
      : asset1Price;
    asset2Price = selectedAssetTwo
      ? getOraclePrice(selectedAssetTwo.getSymbol(), "coingecko", "usd")
      : asset2Price;

    if (asset1Price.eq(0) && selectedAssetOne?.getSymbol() === "PICA") {
      asset1Price = asset2Price.eq(0)
        ? new BigNumber(0)
        : new BigNumber(1).div(asset2Price);
    }
    if (asset2Price.eq(0) && selectedAssetTwo?.getSymbol() === "PICA") {
      asset2Price = asset1Price.eq(0)
        ? new BigNumber(0)
        : new BigNumber(1).div(asset1Price);
    }

    return [asset1Price, asset2Price];
  }, [selectedAssetOne, selectedAssetTwo]);

  const balance1 = useAssetBalance(selectedAssetOne, "picasso");
  const balance2 = useAssetBalance(selectedAssetTwo, "picasso");

  const pools = useStore((store) => store.pools.config);
  const isPoolsLoaded = useStore((store) => store.pools.isLoaded);
  const assetList = useMemo(() => {
    type InputSelectionItem = {
      [assetId in string]: Option;
    };

    if (!isPoolsLoaded) return [];

    const inputs = pools.reduce((acc, cur) => {
      const [a, b] = cur.config.assets.map(
        (asset) =>
          ({
            value: asset.getPicassoAssetId()?.toString() ?? "none",
            label: asset.getName(),
            shortLabel: asset.getSymbol(),
            icon: asset.getIconUrl(),
          } as const)
      );
      acc[a.value] = a;
      acc[b.value] = b;
      return acc;
    }, {} as InputSelectionItem);

    return Object.values(inputs);
  }, [isPoolsLoaded, pools]);

  useEffect(() => {
    if (isPoolsLoaded) {
      const pool = pools.find((pool) => {
        try {
          const [pairBase, pairQuote] = pool.config.assets.map((a) =>
            a.getPicassoAssetId()?.toString()
          );
          return (
            (pairBase === selectedAssetOneId &&
              pairQuote === selectedAssetTwoId) ||
            (pairBase === selectedAssetTwoId &&
              pairQuote === selectedAssetOneId)
          );
        } catch (err: any) {
          console.error("[useSwaps] Liquidity Pool not found. ", err.message);
        }
      });
      setSelectedPool(pool);
    }
  }, [
    isPoolsLoaded,
    pools,
    selectedAssetOneId,
    selectedAssetTwoId,
    setSelectedPool,
  ]);

  const [minimumReceived, setMinimumReceived] = useState<{
    asset: Asset | undefined;
    value: BigNumber;
  }>({
    asset: baseAsset,
    value: new BigNumber(0),
  });
  const [slippageAmount, setSlippageAmount] = useState(new BigNumber(0));
  const [feeCharged, setFeeCharged] = useState(new BigNumber(0));

  const resetTokenAmounts = useCallback(() => {
    setTokenAmounts({
      assetOneAmount: new BigNumber(0),
      assetTwoAmount: new BigNumber(0),
    });
  }, [setTokenAmounts]);

  const onChangeTokenAmount = (amount: BigNumber) => {
    if (selectedPool && amount.gt(0)) {
      const sideUpdated = inputMode === 1 ? "quote" : "base";
      const feePercentage = selectedPool.config.feeConfig.feeRate;
      let minReceive = new BigNumber(0);
      const tokenQuoteAmount =
        selectedAssetOneId === quoteAsset?.getPicassoAssetId()
          ? quoteAmount
          : baseAmount;
      const tokenBaseAmount =
        selectedAssetTwoId === baseAsset?.getPicassoAssetId()
          ? baseAmount
          : quoteAmount;

      const tokenQuoteAsset =
        selectedAssetOneId === quoteAsset?.getPicassoAssetId()
          ? quoteAsset
          : baseAsset;
      const tokenBaseAsset =
        selectedAssetTwoId === baseAsset?.getPicassoAssetId()
          ? baseAsset
          : quoteAsset;
      if (sideUpdated === "quote") {
        const toAmount = calculateOutGivenIn(
          tokenBaseAmount,
          tokenQuoteAmount,
          amount.gt(balance1) ? balance1 : amount,
          new BigNumber(5),
          new BigNumber(5)
        );

        const feeChargedAmount = toAmount.multipliedBy(feePercentage / 100);
        const slippageAmount = toAmount.multipliedBy(slippage / 100);
        minReceive = toAmount.minus(slippageAmount.plus(feeChargedAmount));
        setMinimumReceived({
          asset: tokenBaseAsset,
          value: minReceive,
        });
        setFeeCharged(feeChargedAmount);
        setSlippageAmount(slippageAmount);
        setTokenAmounts({
          assetOneAmount: amount.gt(balance1) ? balance1 : amount,
          assetTwoAmount: toAmount.minus(feeChargedAmount),
        });
      } else {
        // INPUT mode 2 = token2 change, base change
        // MinReceive for baseChange

        const fromAmount = calculateOutGivenIn(
          tokenQuoteAmount,
          tokenBaseAmount,
          amount,
          new BigNumber(5),
          new BigNumber(5)
        );

        const feeChargedAmount = fromAmount.multipliedBy(feePercentage / 100);
        const slippageAmount = fromAmount.multipliedBy(slippage / 100);
        minReceive = fromAmount.minus(slippageAmount.plus(feeChargedAmount));
        setTokenAmounts({
          assetOneAmount: fromAmount.minus(feeChargedAmount),
          assetTwoAmount: amount,
        });

        setMinimumReceived({
          value: minReceive,
          asset: tokenQuoteAsset,
        });
        setFeeCharged(feeChargedAmount);
        setSlippageAmount(slippageAmount);
      }
    }
  };

  const { spotPrice } = usePoolSpotPrice(
    selectedPool,
    selectedPool?.config.assets
  );
  const poolAssets = selectedPool
    ? Object.keys(selectedPool.config.assets)
    : null;
  let poolQuoteBalance = poolAssets
    ? poolAssets?.[0] === selectedAssetOneId
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
    baseWeight: new BigNumber(0),
    quoteBalance: poolQuoteBalance,
    baseBalance: poolBaseBalance,
  });

  /**
   * Effect to update minimum received when
   * there is a change in slippage
   */
  useEffect(() => {
    if (parachainApi && selectedPool) {
      if (previousSlippage != slippage) {
        if (minimumReceived.gt(0)) {
          if (selectedPool) {
            //
            // const  = calculateOutGivenIn(
            //   baseAmount,
            //   quoteAmount,
            //   assetOneAmount,
            //   new BigNumber(5),
            //   new BigNumber(5)
            // );
            //
            // const slippageAmount = assetOneAmount
            //   .minus(feeCharged)
            //   .multipliedBy(slippage);
            // const minReceive = assetOneAmount.minus(slippageAmount);
            //
            // setMinimumReceived(minReceive);
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
    onChainId: string | "none"
  ) => {
    changedSide === "quote"
      ? setSelectedAssetOne(onChainId)
      : setSelectedAssetTwo(onChainId);
    resetTokenAmounts();
  };

  const valid = assetOneInputValid && assetTwoInputValid && !!selectedPool;

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
    assetList,
    onChangeTokenAmount,
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
    priceImpact,
  };
}

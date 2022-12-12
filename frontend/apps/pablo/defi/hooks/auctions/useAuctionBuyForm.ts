import { Asset, PabloLiquidityBootstrappingPool } from "shared";
import { calculator, DEFAULT_NETWORK_ID } from "@/defi/utils";
import { usePrevious } from "@/hooks/usePrevious";
import { useAppSettingsSlice } from "@/store/appSettings/slice";
import { useAssetBalance } from "@/defi/hooks";
import {
  setAuctionsSpotPrice,
  useAuctionsSlice,
} from "@/store/auctions/auctions.slice";
import BigNumber from "bignumber.js";
import _ from "lodash";
import { useSnackbar } from "notistack";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  useDotSamaContext,
  usePendingExtrinsic,
  useSelectedAccount,
} from "substrate-react";
import { useAsset } from "../assets/useAsset";
import { useAuctionSpotPrice } from "./useAuctionSpotPrice";

const UPDATE_SPOT_PRICE_IN = 10_000;
const UPDATE_TRADES_IN = 30_000;
const UPDATE_STATS_IN = 30_000;

const initialTokenAmounts = {
  baseAmount: new BigNumber(0),
  quoteAmount: new BigNumber(0),
  minimumReceived: new BigNumber(0),
  slippageAmount: new BigNumber(0),
  feeCharged: new BigNumber(0),
};

export const useAuctionBuyForm = (): {
  balanceBase: BigNumber;
  balanceQuote: BigNumber;
  isValidBaseInput: boolean;
  setIsValidBaseInput: (validity: boolean) => void;
  isValidQuoteInput: boolean;
  setIsValidQuoteInput: (validity: boolean) => void;
  quoteAsset: Asset | undefined;
  baseAsset: Asset | undefined;
  minimumReceived: BigNumber;
  baseAmount: BigNumber;
  quoteAmount: BigNumber;
  feeCharged: BigNumber;
  slippageAmount: BigNumber;
  selectedAuction: PabloLiquidityBootstrappingPool | null;
  isBuyButtonDisabled: boolean;
  isPendingBuy: boolean;
  onChangeTokenAmount: (
    changedSide: "quote" | "base",
    amount: BigNumber
  ) => void;
} => {
  const { enqueueSnackbar } = useSnackbar();
  const { extensionStatus } = useDotSamaContext();
  const { activePool } = useAuctionsSlice();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const slippage = useAppSettingsSlice().transactionSettings.tolerance;
  const previousSlippage = usePrevious(slippage);

  const spotPrice = useAuctionSpotPrice(
    activePool ? (activePool.getPoolId(true) as BigNumber).toNumber() : -1
  );
  
  const pair = activePool ? Object.keys(activePool.getAssets().assets) : null;
  const baseAsset = useAsset(
    pair?.[0] ?? "-"
  );
  const quoteAsset = useAsset(
    pair?.[1] ?? "-"
  );

  const balanceBase = useAssetBalance(
    baseAsset,
    "picasso"
  );
  const balanceQuote = useAssetBalance(
    quoteAsset,
    "picasso"
  );

  const [isValidBaseInput, setIsValidBaseInput] = useState(false);
  const [isValidQuoteInput, setIsValidQuoteInput] = useState(false);

  const [tokenAmounts, setTokenAmounts] = useState(initialTokenAmounts);
  const resetTokenAmounts = useCallback(
    () => setTokenAmounts(initialTokenAmounts),
    []
  );

  const isUpdatingField = useRef(false);

  useEffect(() => {
    if (selectedAccount) {
      resetTokenAmounts();
    }
  }, [selectedAccount, resetTokenAmounts]);

  const updateSpotPrice = useCallback(() => {
    if (!activePool) return;

    activePool.getSpotPrice().then((spotPrice) => {
      setAuctionsSpotPrice(
        activePool.getPoolId() as string,
        spotPrice
      )
    })
  }, [activePool]);

  const onChangeTokenAmount = useCallback(
    (changedSide: "quote" | "base", amount: BigNumber) => {
      if (!activePool || isUpdatingField.current) return;
      if (spotPrice.eq(0)) {
        updateSpotPrice();
        return;
      }
      isUpdatingField.current = true;
      updateSpotPrice();
      const feeRate = activePool.getFeeConfig().getFeeRate();
      let feePercentage = new BigNumber(feeRate).toNumber();
      const { minReceive, tokenOutAmount, feeChargedAmount, slippageAmount } =
        calculator(changedSide, amount, spotPrice, slippage, feePercentage);

      if (changedSide === "base" && tokenOutAmount.gt(balanceQuote)) {
        enqueueSnackbar("Insufficient Quote Asset balance.", {
          variant: "error",
        });
        resetTokenAmounts();
      } else {
        setTokenAmounts({
          quoteAmount: changedSide === "base" ? tokenOutAmount : amount,
          baseAmount: changedSide === "quote" ? tokenOutAmount : amount,
          minimumReceived: minReceive,
          feeCharged: feeChargedAmount,
          slippageAmount: slippageAmount,
        });
      }
      isUpdatingField.current = false;
    },
    [
      balanceQuote,
      activePool,
      spotPrice,
      updateSpotPrice,
      enqueueSnackbar,
      resetTokenAmounts,
      slippage,
    ]
  );

  // useCallback will always receive
  // up to date deps
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const debouncedUpdater = useCallback(_.debounce(onChangeTokenAmount, 500), [
    onChangeTokenAmount,
  ]);

  useEffect(() => {
    const spotPriceInterval = setInterval(
      updateSpotPrice,
      UPDATE_SPOT_PRICE_IN
    );
    return () => {
      clearInterval(spotPriceInterval);
    };
  }, [updateSpotPrice]);

  // Will be reworked
  // const updateActiveAuctionsPoolTrades = useCallback(() => {
  //   if (activePool) {
  //     fetchAuctionTrades(activePool)
  //       .then((activePoolTradeHistory) => {
  //         setAuctionsSlice({ activePoolTradeHistory });
  //       })
  //       .catch(console.error)
  //   }
  // }, [activePool]);

  // useEffect(() => {
  //   const updateActiveAuctionsPoolTradesInterval = setInterval(
  //     updateActiveAuctionsPoolTrades,
  //     UPDATE_TRADES_IN
  //   );
  //   return () => {
  //     clearInterval(updateActiveAuctionsPoolTradesInterval);
  //   };
  // }, [updateActiveAuctionsPoolTrades]);

  // const updateActiveAuctionPoolStats = useCallback(() => {
  //   if (activePool) {
  //     fetchAndExtractAuctionStats(activePool)
  //       .then((activePoolStats) => {
  //         setAuctionsSlice({ activePoolStats });
  //       })
  //       .catch(console.error);
  //   }
  // }, [activePool]);

  // useEffect(() => {
  //   const updateActiveAuctionPoolStatsInterval = setInterval(
  //     updateActiveAuctionPoolStats,
  //     UPDATE_STATS_IN
  //   );
  //   return () => {
  //     clearInterval(updateActiveAuctionPoolStatsInterval);
  //   };
  // }, [updateActiveAuctionPoolStats]);

  const { baseAmount, quoteAmount } = tokenAmounts;
  const isPendingBuy = usePendingExtrinsic(
    "exchange",
    "dexRouter",
    selectedAccount ? selectedAccount.address : ""
  );

  const isBuyButtonDisabled = useMemo(() => {
    return (
      extensionStatus !== "connected" ||
      !isValidBaseInput ||
      !isValidQuoteInput ||
      isPendingBuy
    );
  }, [isValidBaseInput, isValidQuoteInput, extensionStatus, isPendingBuy]);

  /**
   * Effect to update minimum received when
   * there is a change in slippage
   */
  useEffect(() => {
    if (activePool) {
      if (previousSlippage != slippage) {
        const { minimumReceived } = tokenAmounts;
        if (minimumReceived.gt(0)) {
          const feeRate = activePool.getFeeConfig().getFeeRate();
          let feePercentage = new BigNumber(feeRate).toNumber();

          const { minReceive } = calculator(
            "quote",
            quoteAmount,
            spotPrice,
            slippage,
            feePercentage
          );

          setTokenAmounts((amounts) => {
            return {
              ...amounts,
              minimumReceived: minReceive,
            };
          });
        }
      }
    }
    return;
  }, [
    spotPrice,
    activePool,
    balanceQuote,
    previousSlippage,
    tokenAmounts,
    slippage,
    quoteAmount,
    baseAmount,
  ]);

  return {
    balanceBase,
    balanceQuote,
    isValidBaseInput,
    setIsValidBaseInput,
    isValidQuoteInput,
    setIsValidQuoteInput,
    quoteAsset,
    baseAsset,
    minimumReceived: tokenAmounts.minimumReceived,
    baseAmount,
    quoteAmount,
    slippageAmount: tokenAmounts.slippageAmount,
    feeCharged: tokenAmounts.feeCharged,
    isBuyButtonDisabled,
    selectedAuction: activePool,
    onChangeTokenAmount: debouncedUpdater,
    isPendingBuy,
  };
};

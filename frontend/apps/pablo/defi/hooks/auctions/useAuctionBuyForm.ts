import { fetchAuctionTrades } from "@/defi/subsquid/auctions/helpers";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { calculator, DEFAULT_NETWORK_ID, fetchSpotPrice } from "@/defi/utils";
import { fetchAndExtractAuctionStats } from "@/defi/utils/pablo/auctions";
import { useAppSelector } from "@/hooks/store";
import { usePrevious } from "@/hooks/usePrevious";
import { MockedAsset } from "@/store/assets/assets.types";
import { useAssetBalance } from "@/store/assets/hooks";
import {
  setAuctionsSlice,
  setAuctionsSpotPrice,
  useAuctionsSlice,
} from "@/store/auctions/auctions.slice";
import BigNumber from "bignumber.js";
import _ from "lodash";
import { useSnackbar } from "notistack";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  useDotSamaContext,
  useParachainApi,
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
  quoteAsset: MockedAsset | undefined;
  baseAsset: MockedAsset | undefined;
  minimumReceived: BigNumber;
  baseAmount: BigNumber;
  quoteAmount: BigNumber;
  feeCharged: BigNumber;
  slippageAmount: BigNumber;
  selectedAuction: LiquidityBootstrappingPool;
  isBuyButtonDisabled: boolean;
  isPendingBuy: boolean;
  onChangeTokenAmount: (
    changedSide: "quote" | "base",
    amount: BigNumber
  ) => void;
} => {
  const { enqueueSnackbar } = useSnackbar();
  const { extensionStatus } = useDotSamaContext();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { activePool } = useAuctionsSlice();
  const slippage = useAppSelector(
    (state) => state.settings.transactionSettings.tolerance
  );
  const previousSlippage = usePrevious(slippage);

  const spotPrice = useAuctionSpotPrice(activePool.poolId);
  const baseAsset = useAsset(activePool.pair.base.toString());
  const quoteAsset = useAsset(activePool.pair.quote.toString());

  const balanceBase = useAssetBalance(
    DEFAULT_NETWORK_ID,
    baseAsset ? baseAsset.network[DEFAULT_NETWORK_ID] : "none"
  );
  const balanceQuote = useAssetBalance(
    DEFAULT_NETWORK_ID,
    quoteAsset ? quoteAsset.network[DEFAULT_NETWORK_ID] : "none"
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
    if (!activePool || !parachainApi) return;
    const { base, quote } = activePool.pair;
    let pair = { base: base.toString(), quote: quote.toString() };
    fetchSpotPrice(parachainApi, pair, activePool.poolId)
      .then((_spotPrice) => {
        setAuctionsSpotPrice(activePool.poolId.toString(), _spotPrice);
      })
      .catch(console.error);
  }, [activePool, parachainApi]);

  const onChangeTokenAmount = useCallback(
    (changedSide: "quote" | "base", amount: BigNumber) => {
      if (!parachainApi || !activePool || isUpdatingField.current) return;
      if (spotPrice.eq(0)) {
        updateSpotPrice();
        return;
      }
      isUpdatingField.current = true;
      updateSpotPrice();
      const {
        feeConfig: { feeRate },
      } = activePool;
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
      parachainApi,
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

  const updateActiveAuctionsPoolTrades = useCallback(() => {
    if (activePool.poolId !== -1) {
      fetchAuctionTrades(activePool)
        .then((activePoolTradeHistory) => {
          setAuctionsSlice({ activePoolTradeHistory });
        })
        .catch((err) => {
          console.error(err);
        });
    }
  }, [activePool]);

  useEffect(() => {
    const updateActiveAuctionsPoolTradesInterval = setInterval(
      updateActiveAuctionsPoolTrades,
      UPDATE_TRADES_IN
    );
    return () => {
      clearInterval(updateActiveAuctionsPoolTradesInterval);
    };
  }, [updateActiveAuctionsPoolTrades]);

  const updateActiveAuctionPoolStats = useCallback(() => {
    if (parachainApi && activePool.poolId !== -1) {
      fetchAndExtractAuctionStats(parachainApi, activePool)
        .then((activePoolStats) => {
          setAuctionsSlice({ activePoolStats });
        })
        .catch((err) => {
          console.error(err.message);
        });
    }
  }, [parachainApi, activePool]);

  useEffect(() => {
    const updateActiveAuctionPoolStatsInterval = setInterval(
      updateActiveAuctionPoolStats,
      UPDATE_STATS_IN
    );
    return () => {
      clearInterval(updateActiveAuctionPoolStatsInterval);
    };
  }, [updateActiveAuctionPoolStats]);

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
    if (parachainApi && activePool) {
      if (previousSlippage != slippage) {
        const { minimumReceived } = tokenAmounts;
        if (minimumReceived.gt(0)) {
          const { feeRate } = activePool.feeConfig;
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
    parachainApi,
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

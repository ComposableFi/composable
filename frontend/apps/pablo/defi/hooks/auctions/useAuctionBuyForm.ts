import { fetchAuctionTrades } from "@/defi/subsquid/auctions/helpers";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { calculator, DEFAULT_NETWORK_ID, fetchSpotPrice } from "@/defi/utils";
import { fetchAuctions } from "@/defi/utils/pablo/auctions";
import { useAppSelector } from "@/hooks/store";
import { usePrevious } from "@/hooks/usePrevious";
import { MockedAsset } from "@/store/assets/assets.types";
import { useAssetBalance } from "@/store/assets/hooks";
import useStore from "@/store/useStore";
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
  refreshAuctionData: () => void;
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
  const {
    auctions: { activeLBP },
    putStatsActiveLBP,
    putHistoryActiveLBP,
  } = useStore();
  const slippage = useAppSelector(
    (state) => state.settings.transactionSettings.tolerance
  );
  const previousSlippage = usePrevious(slippage);

  const [spotPrice, setSpotPrice] = useState(new BigNumber(0));
  const baseAsset = useAsset(activeLBP.pair.base.toString());
  const quoteAsset = useAsset(activeLBP.pair.quote.toString());

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
  const resetTokenAmounts = useCallback(() => setTokenAmounts(initialTokenAmounts), []);

  const isUpdatingField = useRef(false);

  useEffect(() => {
    if (selectedAccount) {
      resetTokenAmounts();
    }
  }, [selectedAccount, resetTokenAmounts]);

  const updateSpotPrice = useCallback(() => {
    if (!activeLBP || !parachainApi) return;
    const { base, quote } = activeLBP.pair;
    let pair = { base: base.toString(), quote: quote.toString() };
    fetchSpotPrice(parachainApi, pair, activeLBP.poolId)
      .then(setSpotPrice)
      .catch(console.error);
  }, [activeLBP, parachainApi]);

  const onChangeTokenAmount = useCallback(
    (changedSide: "quote" | "base", amount: BigNumber) => {
      if (!parachainApi || !activeLBP || isUpdatingField.current) return;
      if (spotPrice.eq(0)) {
        updateSpotPrice();
        return;
      }
      isUpdatingField.current = true;
      updateSpotPrice();
      const {
        feeConfig: { feeRate },
      } = activeLBP;
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
      activeLBP,
      spotPrice,
      updateSpotPrice,
      enqueueSnackbar,
      resetTokenAmounts,
      slippage
    ]
  );

  // useCallback will always receive
  // up to date dependancies
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const debouncedUpdater = useCallback(_.debounce(onChangeTokenAmount, 500), [
    onChangeTokenAmount,
  ]);

  const refreshAuctionData = useCallback(async () => {
    const { poolId } = activeLBP;
    if (parachainApi && poolId !== -1) {
      const stats = await fetchAuctions(parachainApi, activeLBP);
      const trades = await fetchAuctionTrades(activeLBP);
      putStatsActiveLBP(stats);
      putHistoryActiveLBP(trades);
    }
  }, [activeLBP, putHistoryActiveLBP, putStatsActiveLBP, parachainApi]);

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
    if (parachainApi && activeLBP) {
      if (previousSlippage != slippage) {
        const { minimumReceived } = tokenAmounts;
        if (minimumReceived.gt(0)) {
          const { feeRate } = activeLBP.feeConfig;
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
    activeLBP,
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
    selectedAuction: activeLBP,
    refreshAuctionData,
    onChangeTokenAmount: debouncedUpdater,
    isPendingBuy,
  };
};

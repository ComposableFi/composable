import {
  DEFAULT_NETWORK_ID,
  isValidAssetPair,
  toChainUnits,
} from "@/defi/utils";
import BigNumber from "bignumber.js";
import { useSnackbar } from "notistack";
import { useCallback, useMemo } from "react";
import {
  useSigner,
  useExecutor,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import {
  transactionStatusSnackbarMessage,
  SNACKBAR_TYPES,
} from "../addLiquidity/useAddLiquidity";

type PabloSwapProps = {
  baseAssetId: string;
  quoteAssetId: string;
  minimumReceived: BigNumber;
  quoteAmount: BigNumber;
  swapOrigin?: "Auction" | "Swap";
};

export function usePabloSwap({
  quoteAssetId,
  baseAssetId,
  quoteAmount,
  minimumReceived,
  swapOrigin = "Swap",
}: PabloSwapProps) {
  const { enqueueSnackbar } = useSnackbar();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const signer = useSigner();
  const executor = useExecutor();

  const onTxReady = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(
          swapOrigin,
          transactionHash,
          "Initiated"
        ),
        SNACKBAR_TYPES.INFO
      );
    },
    [enqueueSnackbar, swapOrigin]
  );

  const onTxFinalized = useCallback(
    (transactionHash: string, _eventRecords: any[]) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(
          swapOrigin,
          transactionHash,
          "Finalized"
        ),
        SNACKBAR_TYPES.SUCCESS
      );
    },
    [enqueueSnackbar, swapOrigin]
  );

  const onTxError = useCallback(
    (transactionError: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(swapOrigin, transactionError, "Error"),
        SNACKBAR_TYPES.ERROR
      );
    },
    [enqueueSnackbar, swapOrigin]
  );

  const validAssetPair = useMemo(() => {
    return isValidAssetPair(baseAssetId, quoteAssetId);
  }, [baseAssetId, quoteAssetId]);

  const pair = useMemo(() => {
    return {
      base: baseAssetId,
      quote: quoteAssetId,
    };
  }, [baseAssetId, quoteAssetId]);

  const amount = useMemo(() => {
    if (!parachainApi) return null;
    return parachainApi.createType(
      "u128",
      toChainUnits(quoteAmount).toString()
    );
  }, [parachainApi, quoteAmount]);

  const minimumReceive = useMemo(() => {
    if (!parachainApi) return null;
    return parachainApi.createType(
      "u128",
      toChainUnits(minimumReceived).toString()
    );
  }, [parachainApi, minimumReceived]);

  const useSwapTx = useCallback(async (): Promise<void> => {
    try {
      if (
        !parachainApi ||
        !signer ||
        !executor ||
        !validAssetPair ||
        !selectedAccount ||
        !amount ||
        !minimumReceive
      ) {
        throw new Error("Missing dependencies.");
      }

      await executor.execute(
        parachainApi.tx.dexRouter.exchange(pair, amount, minimumReceive),
        selectedAccount.address,
        parachainApi,
        signer,
        onTxReady,
        onTxFinalized,
        onTxError
      );
    } catch (err: any) {
      onTxError(err.message);
    }
  }, [
    parachainApi,
    signer,
    executor,
    validAssetPair,
    selectedAccount,
    amount,
    minimumReceive,
    pair,
    onTxReady,
    onTxFinalized,
    onTxError,
  ]);

  return useSwapTx;
}

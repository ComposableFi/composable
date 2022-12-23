import {
  DEFAULT_NETWORK_ID,
  isValidAssetPair,
  toChainUnits,
} from "@/defi/utils";
import BigNumber from "bignumber.js";
import { useSnackbar } from "notistack";
import { useCallback, useMemo } from "react";
import {
  useExecutor,
  useParachainApi,
  useSelectedAccount,
  useSigner,
} from "substrate-react";
import {
  SNACKBAR_TYPES,
  transactionStatusSnackbarMessage,
} from "../addLiquidity/useAddLiquidity";
import { PoolConfig } from "@/store/pools/types";

type PabloSwapProps = {
  pool: PoolConfig | undefined;
  baseAssetId: string;
  quoteAssetId: string;
  minimumReceived: BigNumber;
  quoteAmount: BigNumber;
  swapOrigin?: "Auction" | "Swap";
};

export function usePabloSwap({
  pool,
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
        !minimumReceive ||
        !pool
      ) {
        throw new Error("Missing dependencies.");
      }

      const toChainQuoteAmount = toChainUnits(quoteAmount).toString();
      const toChainMinReceive = toChainUnits(minimumReceived).toString();
      await executor.execute(
        parachainApi.tx.pablo.swap(
          pool.poolId.toString(),
          {
            assetId: quoteAssetId,
            amount: toChainQuoteAmount,
          },
          {
            assetId: baseAssetId,
            amount: toChainMinReceive,
          },
          true
        ),
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
    pool,
    onTxReady,
    onTxFinalized,
    onTxError,
  ]);

  return useSwapTx;
}

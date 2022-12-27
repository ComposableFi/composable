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
import { Asset } from "shared";

type PabloSwapProps = {
  pool: PoolConfig | undefined;
  baseAsset: Asset | undefined;
  quoteAsset: Asset | undefined;
  minimumReceived: BigNumber;
  quoteAmount: BigNumber;
  swapOrigin?: "Auction" | "Swap";
};

export function usePabloSwap({
  pool,
  quoteAsset,
  baseAsset,
  quoteAmount,
  minimumReceived,
  swapOrigin = "Swap",
}: PabloSwapProps) {
  const { enqueueSnackbar } = useSnackbar();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const signer = useSigner();
  const executor = useExecutor();

  const quoteAssetId = quoteAsset?.getPicassoAssetId()?.toString();
  const baseAssetId = quoteAsset?.getPicassoAssetId()?.toString();

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
    if (!baseAssetId || !quoteAssetId) return false;
    return isValidAssetPair(baseAssetId, quoteAssetId);
  }, [baseAssetId, quoteAssetId]);
  useMemo(() => {
    return {
      base: baseAssetId,
      quote: quoteAssetId,
    };
  }, [baseAssetId, quoteAssetId]);
  const amount = useMemo(() => {
    if (!parachainApi) return null;
    return parachainApi.createType(
      "u128",
      toChainUnits(
        quoteAmount,
        quoteAsset?.getDecimals(DEFAULT_NETWORK_ID)
      ).toString()
    );
  }, [parachainApi, quoteAmount, quoteAsset]);

  const minimumReceive = useMemo(() => {
    if (!parachainApi) return null;
    return parachainApi.createType(
      "u128",
      toChainUnits(
        minimumReceived,
        baseAsset?.getDecimals(DEFAULT_NETWORK_ID)
      ).toString()
    );
  }, [parachainApi, minimumReceived, baseAsset]);

  return useCallback(async (): Promise<void> => {
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

      const toChainQuoteAmount = toChainUnits(
        quoteAmount,
        quoteAsset?.getDecimals(DEFAULT_NETWORK_ID)
      ).toString();
      const toChainMinReceive = toChainUnits(
        minimumReceived,
        baseAsset?.getDecimals(DEFAULT_NETWORK_ID)
      ).toString();

      await executor.execute(
        parachainApi.tx.pablo.swap(
          pool.poolId.toString(),
          {
            assetId: quoteAsset?.getPicassoAssetId()?.toString(),
            amount: toChainQuoteAmount,
          },
          {
            assetId: quoteAsset?.getPicassoAssetId()?.toString(),
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
    quoteAmount,
    quoteAsset,
    minimumReceived,
    baseAsset,
    onTxReady,
    onTxFinalized,
    onTxError,
  ]);
}

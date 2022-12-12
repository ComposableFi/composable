import { Executor } from "substrate-react";
import { ApiPromise } from "@polkadot/api";
import { toChainUnits } from "@/defi/utils";
import { resetAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import { useSnackbar, VariantType } from "notistack";
import { Signer } from "@polkadot/api/types";
import { useCallback, useMemo } from "react";
import { setUiState } from "@/store/ui/ui.slice";
import { DualAssetConstantProduct } from "shared";
import BigNumber from "bignumber.js";
import router from "next/router";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

const TxOrigin = "Add Liquidity";

export function transactionStatusSnackbarMessage(
  moduleName: string,
  transactionHashOrErrorMessage: string,
  status: "Initiated" | "Finalized" | "Error"
): string {
  return `${moduleName} Transaction ${status}: ${transactionHashOrErrorMessage}`;
}

/**
 * Later: move to snackbar utils
 */
export const SNACKBAR_TYPES: Record<string, { variant: VariantType }> = {
  ERROR: { variant: "error" },
  SUCCESS: { variant: "success" },
  INFO: { variant: "info" },
};

export const useAddLiquidity = ({
  selectedAccount,
  executor,
  parachainApi,
  assetOne,
  assetTwo,
  assetOneAmount,
  assetTwoAmount,
  lpReceiveAmount,
  pool,
  signer,
}: {
  selectedAccount: InjectedAccountWithMeta | undefined;
  executor: Executor | undefined;
  parachainApi: ApiPromise | undefined;
  assetOne: string | undefined;
  assetTwo: string | undefined;
  pool: DualAssetConstantProduct | undefined;
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
  lpReceiveAmount: BigNumber;
  signer: Signer | undefined;
}) => {
  const { enqueueSnackbar } = useSnackbar();

  const { baseAmount, quoteAmount } = useMemo(() => {
    if (!pool || !assetOne)
      return {
        baseAmount: undefined,
        quoteAmount: undefined,
      };

    const pair = Object.keys(pool.getAssets().assets);
    let isReversed = pair[0].toString() !== assetOne;
    return {
      baseAmount: toChainUnits(
        isReversed ? assetTwoAmount : assetOneAmount
      ).toString(),
      quoteAmount: toChainUnits(
        isReversed ? assetOneAmount : assetTwoAmount
      ).toString(),
    };
  }, [pool, assetOne, assetOneAmount, assetTwoAmount]);

  const _lpReceiveAmount = useMemo(() => {
    return toChainUnits(lpReceiveAmount).toString();
  }, [lpReceiveAmount]);

  const onTxReady = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(TxOrigin, transactionHash, "Initiated"),
        SNACKBAR_TYPES.INFO
      );
      setUiState({ isConfirmingSupplyModalOpen: true, isConfirmSupplyModalOpen: false });
    },
    [enqueueSnackbar]
  );

  const onTxFinalized = useCallback(
    (transactionHash: string, _eventRecords: any[]) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(TxOrigin, transactionHash, "Finalized"),
        SNACKBAR_TYPES.SUCCESS
      );
      resetAddLiquiditySlice();
      const poolId = pool?.getPoolId() as string;
      router.push("/pool/select/" + poolId);
      setUiState({ isConfirmingSupplyModalOpen: false });
    },
    [pool, enqueueSnackbar]
  );

  const onTxError = useCallback(
    (transactionError: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(TxOrigin, transactionError, "Error"),
        SNACKBAR_TYPES.ERROR
      );
      setUiState({ isConfirmingSupplyModalOpen: false });
    },
    [enqueueSnackbar]
  );

  return useCallback(async () => {
    try {
      if (
        !selectedAccount ||
        !parachainApi ||
        !executor ||
        !assetOne ||
        !baseAmount ||
        !quoteAmount ||
        !assetTwo ||
        !signer ||
        !pool
      ) {
        throw new Error("Missing dependencies.");
      }

      setUiState({ isConfirmingSupplyModalOpen: false });

      await executor.execute(
        parachainApi.tx.pablo.addLiquidity(
          pool.getPoolId() as string,
          baseAmount,
          quoteAmount,
          _lpReceiveAmount,
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
      enqueueSnackbar(
        transactionStatusSnackbarMessage(TxOrigin, err.message, "Error"),
        SNACKBAR_TYPES.ERROR
      );
      setUiState({ isConfirmingSupplyModalOpen: false });
    }
  }, [selectedAccount, parachainApi, executor, assetOne, baseAmount, quoteAmount, assetTwo, signer, pool, _lpReceiveAmount, onTxReady, onTxFinalized, onTxError, enqueueSnackbar]);

};

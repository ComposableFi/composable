import { Executor } from "substrate-react";
import { ApiPromise } from "@polkadot/api";
import { resetAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import { useSnackbar, VariantType } from "notistack";
import { Signer } from "@polkadot/api/types";
import { useCallback } from "react";
import { setUiState } from "@/store/ui/ui.slice";
import BigNumber from "bignumber.js";
import router from "next/router";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { toChainUnits } from "@/defi/utils";
import { getAssetTree } from "@/components/Organisms/pool/AddLiquidity/utils";
import { pipe } from "fp-ts/lib/function";
import { option } from "fp-ts";

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
  assetOneAmount,
  assetTwoAmount,
  assetInId,
  assetOutId,
  lpReceiveAmount,
  poolId,
  signer,
}: {
  selectedAccount: InjectedAccountWithMeta | undefined;
  executor: Executor | undefined;
  parachainApi: ApiPromise | undefined;
  poolId: string | undefined;
  assetInId: string | null;
  assetOutId: string | null;
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
  lpReceiveAmount: BigNumber;
  signer: Signer | undefined;
}) => {
  const { enqueueSnackbar } = useSnackbar();

  const onTxReady = useCallback(
    (transactionHash: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(
          TxOrigin,
          transactionHash,
          "Initiated"
        ),
        SNACKBAR_TYPES.INFO
      );
      setUiState({
        isConfirmingSupplyModalOpen: true,
        isConfirmSupplyModalOpen: false,
      });
    },
    [enqueueSnackbar]
  );

  const onTxFinalized = useCallback(
    (transactionHash: string, _eventRecords: any[]) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(
          TxOrigin,
          transactionHash,
          "Finalized"
        ),
        SNACKBAR_TYPES.SUCCESS
      );
      resetAddLiquiditySlice();
      router.push("/pool/select/" + poolId);
      setUiState({ isConfirmingSupplyModalOpen: false });
    },
    [poolId, enqueueSnackbar]
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
        !signer ||
        !poolId ||
        !assetInId ||
        !assetOutId
      ) {
        return () => {};
      }
      const assetTree = pipe(
        getAssetTree(
          {
            assetIdOnChain: assetInId,
            balance: assetOneAmount,
          },
          {
            assetIdOnChain: assetOutId,
            balance: assetTwoAmount,
          }
        ),
        option.toNullable
      );

      if (!assetTree) return () => {};

      setUiState({ isConfirmingSupplyModalOpen: false });

      await executor.execute(
        parachainApi.tx.pablo.addLiquidity(
          poolId,
          parachainApi.createType("BTreeMap<u128, u128>", assetTree),
          parachainApi.createType(
            "u128",
            toChainUnits(lpReceiveAmount).toString()
          ),
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
  }, [
    selectedAccount,
    parachainApi,
    executor,
    signer,
    poolId,
    assetInId,
    assetOutId,
    assetOneAmount,
    assetTwoAmount,
    lpReceiveAmount,
    onTxReady,
    onTxFinalized,
    onTxError,
    enqueueSnackbar,
  ]);
};

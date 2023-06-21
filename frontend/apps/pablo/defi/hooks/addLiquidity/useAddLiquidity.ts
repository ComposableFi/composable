import { Executor } from "substrate-react";
import { ApiPromise } from "@polkadot/api";
import { resetAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import { useSnackbar } from "notistack";
import { Signer } from "@polkadot/api/types";
import { useCallback } from "react";
import { setUiState } from "@/store/ui/ui.slice";
import BigNumber from "bignumber.js";
import router from "next/router";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { DEFAULT_NETWORK_ID, toChainUnits } from "@/defi/utils";
import { getAssetTree } from "@/components/Organisms/pool/AddLiquidity/utils";
import { pipe } from "fp-ts/lib/function";
import { option } from "fp-ts";
import { Asset, subscanExtrinsicLink } from "shared";

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
export const SNACKBAR_TYPES: Record<string, any> = {
  ERROR: { variant: "error", persist: true, isClosable: true },
  SUCCESS: { variant: "success", persist: true, isClosable: true },
  INFO: { variant: "info", persist: true, isClosable: true },
};

export const useAddLiquidity = ({
  selectedAccount,
  executor,
  parachainApi,
  assetOneAmount,
  assetTwoAmount,
  assetIn,
  assetOut,
  lpReceiveAmount,
  poolId,
  signer,
}: {
  selectedAccount: InjectedAccountWithMeta | undefined;
  executor: Executor | undefined;
  parachainApi: ApiPromise | undefined;
  poolId: string | undefined;
  assetIn: Asset | undefined;
  assetOut: Asset | undefined;
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
  lpReceiveAmount: BigNumber;
  signer: Signer | undefined;
}) => {
  const { enqueueSnackbar } = useSnackbar();

  const onTxReady = useCallback(
    (hash: string) => {
      enqueueSnackbar(`${TxOrigin}: Initiated`, {
        ...SNACKBAR_TYPES.INFO,
        ...{ url: subscanExtrinsicLink(DEFAULT_NETWORK_ID, hash) },
      });
      setUiState({
        isConfirmingSupplyModalOpen: true,
        isConfirmSupplyModalOpen: false,
      });
    },
    [enqueueSnackbar]
  );

  const onTxFinalized = useCallback(
    (hash: string, _eventRecords: any[]) => {
      enqueueSnackbar(`${TxOrigin}: Finalized`, {
        ...SNACKBAR_TYPES.SUCCESS,
        ...{ url: subscanExtrinsicLink(DEFAULT_NETWORK_ID, hash) },
      });
      resetAddLiquiditySlice();
      router.push("/pool/select/" + poolId);
      setUiState({ isConfirmingSupplyModalOpen: false });
    },
    [poolId, enqueueSnackbar]
  );

  const onTxError = useCallback(
    (transactionError: string) => {
      enqueueSnackbar(`${TxOrigin}: Error`, {
        ...SNACKBAR_TYPES.ERROR,
        ...{ description: transactionError },
      });
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
        !assetIn ||
        !assetOut
      ) {
        return () => {};
      }
      const assetTree = pipe(
        getAssetTree(
          {
            asset: assetIn,
            balance: assetOneAmount,
          },
          {
            asset: assetOut,
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
    assetIn,
    assetOut,
    assetOneAmount,
    assetTwoAmount,
    lpReceiveAmount,
    onTxReady,
    onTxFinalized,
    onTxError,
    enqueueSnackbar,
  ]);
};

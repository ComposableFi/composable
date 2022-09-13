import {
  closeConfirmingSupplyModal,
  closeConfirmSupplyModal,
  openConfirmingSupplyModal,
} from "@/stores/ui/uiSlice";
import Executor from "substrate-react/dist/extrinsics/Executor";
import BigNumber from "bignumber.js";
import router from "next/router";
import { ApiPromise } from "@polkadot/api";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import { toChainUnits } from "@/defi/utils";
import { resetAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import { useSnackbar, VariantType } from "notistack";
import { useDispatch } from "react-redux";
import { ConnectedAccount } from "substrate-react/dist/dotsama/types";
import { Signer } from "@polkadot/api/types";
import { useCallback, useMemo } from "react";

function transactionStatusSnackbarMessage(
  transactionHashOrErrorMessage: string,
  status: "Initiated" | "Finalized" | "Error"
): string {
  return `Add liquidity Transaction ${status}: ${transactionHashOrErrorMessage}`;
}

/**
 * Later: move to snackbar utils
 */
const SNACKBAR_TYPES: Record<string, { variant: VariantType }> = {
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
  selectedAccount: ConnectedAccount | undefined;
  executor: Executor | undefined;
  parachainApi: ApiPromise | undefined;
  assetOne: string | undefined;
  assetTwo: string | undefined;
  pool: ConstantProductPool | StableSwapPool | undefined;
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
  lpReceiveAmount: BigNumber;
  signer: Signer | undefined;
}) => {
  const { enqueueSnackbar } = useSnackbar();
  const dispatch = useDispatch();

  const { baseAmount, quoteAmount } = useMemo(() => {
    if (!pool || !assetOne)
      return {
        baseAmount: undefined,
        quoteAmount: undefined,
      };

    let isReversed = pool.pair.base.toString() !== assetOne;
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
        transactionStatusSnackbarMessage(transactionHash, "Initiated"),
        SNACKBAR_TYPES.INFO
      );
      dispatch(openConfirmingSupplyModal());
    },
    [enqueueSnackbar, dispatch]
  );

  const onTxFinalized = useCallback(
    (transactionHash: string, _eventRecords: any[]) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(transactionHash, "Finalized"),
        SNACKBAR_TYPES.SUCCESS
      );
      resetAddLiquiditySlice();
      router.push("/pool/select/" + pool?.poolId);
      dispatch(closeConfirmingSupplyModal());
    },
    [pool, enqueueSnackbar, dispatch]
  );

  const onTxError = useCallback(
    (transactionError: string) => {
      enqueueSnackbar(
        transactionStatusSnackbarMessage(transactionError, "Error"),
        SNACKBAR_TYPES.ERROR
      );
      dispatch(closeConfirmingSupplyModal());
    },
    [enqueueSnackbar, dispatch]
  );

  const onAddLiquidity = useCallback(async () => {
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

      dispatch(closeConfirmSupplyModal());

      await executor.execute(
        parachainApi.tx.pablo.addLiquidity(
          pool.poolId,
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
        transactionStatusSnackbarMessage(err.message, "Error"),
        SNACKBAR_TYPES.ERROR
      );
      dispatch(closeConfirmingSupplyModal());
    }
  }, [
    parachainApi,
    executor,
    signer,
    baseAmount,
    quoteAmount,
    _lpReceiveAmount,
    enqueueSnackbar,
    assetOne,
    assetTwo,
    dispatch,
    pool,
    selectedAccount,
    onTxError,
    onTxFinalized,
    onTxReady,
  ]);

  return onAddLiquidity;
};

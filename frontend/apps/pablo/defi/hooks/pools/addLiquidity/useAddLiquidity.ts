import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import { toChainUnits } from "@/defi/utils";
import { resetAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import {
  closeConfirmingSupplyModal,
  closeConfirmSupplyModal,
  openConfirmingSupplyModal,
} from "@/stores/ui/uiSlice";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import router from "next/router";
import { useSnackbar } from "notistack";
import { useDispatch } from "react-redux";
import { ConnectedAccount } from "substrate-react/dist/dotsama/types";
import Executor from "substrate-react/dist/extrinsics/Executor";
import { Signer } from "@polkadot/api/types";
import { useCallback, useMemo } from "react";

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
        `Add liquidity Transaction Initiated: ${transactionHash}`,
        { variant: "success" }
      );
      dispatch(openConfirmingSupplyModal());
    },
    [enqueueSnackbar, dispatch]
  );

  const onTxFinalized = useCallback(
    (transactionHash: string, _eventRecords: any[]) => {
      enqueueSnackbar(
        `Add liquidity Transaction Finalized: ${transactionHash}`,
        { variant: "success" }
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
        `Add liquidity Transaction Failed: ERROR: ${transactionError}`,
        { variant: "error" }
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
        throw new Error("Missing dependancies.");
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
        `Add liquidity Transaction Failed: ERROR: ${err.message}`,
        { variant: "error" }
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

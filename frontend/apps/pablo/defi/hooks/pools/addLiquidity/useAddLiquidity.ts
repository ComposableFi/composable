import { getSigner } from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
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
  signer
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
  signer: Signer | undefined
}) => {
  const { enqueueSnackbar } = useSnackbar();
  const dispatch = useDispatch();

  const onConfirmSupply = async () => {
    if (
      !selectedAccount ||
      !parachainApi ||
      !executor ||
      !assetOne ||
      !assetTwo ||
      !signer ||
      !pool
    ) {
      return;
    }

    try {
      dispatch(closeConfirmSupplyModal());

      let isReverse = pool.pair.base.toString() !== assetOne;
      const bnBase = toChainUnits(isReverse ? assetTwoAmount : assetOneAmount);
      const bnQuote = toChainUnits(isReverse ? assetOneAmount : assetTwoAmount);
      lpReceiveAmount = toChainUnits(lpReceiveAmount);

      await executor.execute(
        parachainApi.tx.pablo.addLiquidity(
          pool.poolId,
          bnBase.toString(),
          bnQuote.toString(),
          lpReceiveAmount.toString(),
          true
        ),
        selectedAccount.address,
        parachainApi,
        signer,
        (txReady: string) => {
          dispatch(openConfirmingSupplyModal());
          console.log("txReady", txReady);
        },
        (txHash: string, events) => {
          enqueueSnackbar(
            "Transaction successful. Transaction hash: " + txHash,
            { variant: "success" }
          );
          resetAddLiquiditySlice();
          router.push("/pool/select/" + pool?.poolId);
          dispatch(closeConfirmingSupplyModal());
        },
        (errorMessage: string) => {
          console.log("Tx Error:", errorMessage);
          enqueueSnackbar("Tx Error: " + errorMessage);
          dispatch(closeConfirmingSupplyModal());
        }
      );
    } catch (err: any) {
      enqueueSnackbar(err.message, { variant: "error" });
      dispatch(closeConfirmingSupplyModal());
      console.log("Tx Error:", err);
    }
  };

  return onConfirmSupply;
};

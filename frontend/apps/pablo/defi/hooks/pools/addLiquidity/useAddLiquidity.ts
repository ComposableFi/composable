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
import { ConntectedAccount } from "substrate-react/dist/dotsama/types";
import Executor from "substrate-react/dist/extrinsics/Executor";

export const useAddLiquidity = ({
  selectedAccount,
  executor,
  parachainApi,
  assetOne,
  assetTwo,
  assetOneAmount,
  assetTwoAmount,
  lpAmountExpected,
  pool,
}: {
  selectedAccount: ConntectedAccount | undefined;
  executor: Executor | undefined;
  parachainApi: ApiPromise | undefined;
  assetOne: string | undefined;
  assetTwo: string | undefined;
  pool: ConstantProductPool | StableSwapPool | undefined;
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
  lpAmountExpected: BigNumber;
}) => {
  const { enqueueSnackbar } = useSnackbar();
  const dispatch = useDispatch();

  const onConfirmSupply = async () => {
    if (
      selectedAccount &&
      executor &&
      parachainApi &&
      assetOne !== undefined &&
      assetTwo !== undefined &&
      pool
    ) {
      try {
        dispatch(closeConfirmSupplyModal());

        let isReverse = pool.pair.base.toString() !== assetOne;
        const bnBase = toChainUnits(
          isReverse ? assetTwoAmount : assetOneAmount
        );
        const bnQuote = toChainUnits(
          isReverse ? assetOneAmount : assetTwoAmount
        );

        const signer = await getSigner(APP_NAME, selectedAccount.address);

        executor
          .execute(
            parachainApi.tx.pablo.addLiquidity(
              pool.poolId,
              bnBase.toString(),
              bnQuote.toString(),
              0,
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
              enqueueSnackbar("Transaction successful. Transaction hash: " + txHash, { variant: "success" });
              resetAddLiquiditySlice();
              router.push("/pool/select/" + pool?.poolId);
              dispatch(closeConfirmingSupplyModal());
            },
            (errorMessage: string) => {
              console.log("Tx Error:", errorMessage);
              enqueueSnackbar("Tx Error: " + errorMessage);
              dispatch(closeConfirmingSupplyModal());
            }
          )
          .catch((err) => {
            enqueueSnackbar(err.message, { variant: "error" });
            dispatch(closeConfirmingSupplyModal());
            console.log("Tx Error:", err);
          });
      } catch (err: any) {
        enqueueSnackbar(err.message, { variant: "error" });
        dispatch(closeConfirmingSupplyModal());
        console.log("Tx Error:", err);
      }
    }
  };

  return onConfirmSupply;
};

import { BoxProps, Button, Typography, useTheme } from "@mui/material";
import { useRouter } from "next/router";
import { FormTitle } from "../../FormTitle";
import { TransactionSettings } from "../../TransactionSettings";
import { useSnackbar } from "notistack";
import { FC, useEffect, useMemo, useState } from "react";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { setUiState, useUiSlice } from "@/store/ui/ui.slice";
import { usePoolDetail } from "@/defi/hooks/pools/usePoolDetail";
import useStore from "@/store/useStore";
import { either, option } from "fp-ts";
import { pipe } from "fp-ts/lib/function";
import BigNumber from "bignumber.js";
import { LiquidityInput } from "../../pool/AddLiquidity/LiquidityInput";
import { PoolShare } from "@/components/Organisms/bonds/PoolShare";
import {
  getAssetOptions,
  getInputConfig,
} from "@/components/Organisms/liquidity/AddForm/utils";
import { PlusIcon } from "@/components/Organisms/liquidity/AddForm/PlusIcon";
import { useSimulateAddLiquidity } from "@/components/Organisms/pool/AddLiquidity/useSimulateAddLiquidity";
import { ConfirmSupplyModal } from "@/components/Organisms/liquidity/AddForm/ConfirmSupplyModal";
import { YourPosition } from "@/components/Organisms/liquidity/YourPosition";
import { ConfirmingSupplyModal } from "@/components/Organisms/liquidity/AddForm/ConfirmingSupplyModal";
import { useLiquidity } from "@/defi/hooks";
import {
  useExecutor,
  useParachainApi,
  useSelectedAccount,
  useSigner,
} from "substrate-react";
import {
  addLiquidityToPoolViaPablo,
  DEFAULT_NETWORK_ID,
  fromChainUnits,
} from "@/defi/utils";
import { getAssetTree } from "@/components/Organisms/pool/AddLiquidity/utils";

function amountWithRatio(
  amount: BigNumber,
  spotPrice: BigNumber,
  isReverse: boolean
) {
  return pipe(
    isReverse,
    either.fromPredicate(
      (v) => !v,
      () => new BigNumber(1)
    ),
    either.fold(
      (v) =>
        v.div(
          amount.multipliedBy(spotPrice.isZero() ? new BigNumber(1) : spotPrice)
        ),
      () =>
        amount.multipliedBy(spotPrice.isZero() ? new BigNumber(1) : spotPrice)
    )
  );
}

export const AddLiquidityForm: FC<BoxProps> = ({ ...rest }) => {
  const theme = useTheme();
  const router = useRouter();
  const { enqueueSnackbar } = useSnackbar();
  const { isConfirmSupplyModalOpen, isConfirmingSupplyModalOpen } =
    useUiSlice();
  const onBackHandler = () => {
    router.push(`/pool/select/${poolId}`);
  };
  const onSettingHandler = () => {
    setUiState({ isTransactionSettingsModalOpen: true });
  };
  const [{ amountOne, amountTwo }, setAmount] = useState<{
    amountOne: BigNumber;
    amountTwo: BigNumber;
  }>({
    amountOne: new BigNumber(0),
    amountTwo: new BigNumber(0),
  });

  const gasBalance = useStore(
    (store) => {
      const gasFeeToken = store.byog.feeItem;
      return store.substrateBalances.tokenBalances.picasso[gasFeeToken];
    },
    (a, b) => a.free.eq(b.free)
  );

  const gasFeeRatio = useStore((store) =>
    store.substrateTokens.tokens[store.byog.feeItem].getRatio()
  );

  const gasFeeTokenDecimals = useStore((store) =>
    store.substrateTokens.tokens[store.byog.feeItem].getDecimals(
      DEFAULT_NETWORK_ID
    )
  );

  const [transactionFee, setTransactionFee] = useState<BigNumber>(
    new BigNumber(0)
  );

  const handleConfirmSupplyButtonClick = () => {
    pipe(
      pool,
      option.fromNullable,
      option.fold(
        () => {
          enqueueSnackbar(
            "Liquidity pool for the selected token pair does not exist.",
            {
              variant: "error",
            }
          );
        },
        () => {
          setUiState({ isConfirmSupplyModalOpen: true });
        }
      )
    );
  };

  // Populate Dropdowns
  const { poolId } = usePoolDetail();
  const getPoolById = useStore((store) => store.pools.getPoolById);
  const pool = pipe(getPoolById(poolId), option.toNullable);
  const getTokenBalance = useStore(
    (store) => store.substrateBalances.getTokenBalance
  );
  const inputConfig = pipe(
    getInputConfig(pipe(pool, option.fromNullable), getTokenBalance),
    option.toNullable
  );
  const gasFeeToken = useStore(
    (store) => store.substrateTokens.tokens[store.byog.feeItem]
  );
  const { baseAmount, quoteAmount } = useLiquidity(pool);
  const assetOptions = getAssetOptions(inputConfig ?? []);
  const [leftConfig, rightConfig] = inputConfig ?? [];
  const leftId = (leftConfig?.asset.getPicassoAssetId() as string) || null;
  const rightId = (rightConfig?.asset.getPicassoAssetId() as string) || null;
  const [simulated, setSimulated] = useState<BigNumber>(new BigNumber(0));
  const simulate = useSimulateAddLiquidity();
  const [isInValid, setInValid] = useState<boolean>(false);
  const [isOutValid, setOutValid] = useState<boolean>(false);
  const inputValid = isInValid && isOutValid && gasBalance.free.gt(0);
  const isPoolEmpty = baseAmount.isZero() && quoteAmount.isZero();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const executor = useExecutor();
  const account = useSelectedAccount(DEFAULT_NETWORK_ID);
  const signer = useSigner();

  useEffect(() => {
    if (leftId === null || rightId === null) return;
    simulate(
      poolId,
      {
        asset: leftConfig.asset,
        balance: amountOne,
      },
      {
        asset: rightConfig.asset,
        balance: amountTwo,
      }
    ).then((simulatedValue) => {
      if (!simulatedValue.eq(simulated)) {
        setSimulated(simulatedValue);
      }
    });
  }, [
    amountOne,
    amountTwo,
    leftConfig?.asset,
    leftId,
    poolId,
    rightConfig?.asset,
    rightId,
    simulate,
    simulated,
  ]);

  useMemo(() => {
    if (
      parachainApi &&
      poolId &&
      executor &&
      account &&
      signer &&
      leftConfig &&
      rightConfig
    ) {
      try {
        const assetTree = getAssetTree(
          {
            asset: leftConfig.asset,
            balance: amountOne,
          },
          {
            asset: rightConfig.asset,
            balance: amountTwo,
          }
        );
        const paymentInfo = pipe(
          assetTree,
          option.map((assets) =>
            addLiquidityToPoolViaPablo(parachainApi, poolId, assets)
          ),
          option.map((call) =>
            executor.paymentInfo(call, account.address, signer)
          ),
          option.toNullable
        );
        let transactionFee = new BigNumber(0);
        if (paymentInfo) {
          paymentInfo.then((result) => {
            if (!gasFeeRatio) {
              transactionFee = fromChainUnits(result.partialFee.toString());
            } else {
              transactionFee = fromChainUnits(
                new BigNumber(result.partialFee.toString())
                  .multipliedBy(gasFeeRatio.n)
                  .div(gasFeeRatio.d),
                gasFeeTokenDecimals
              );
            }
            setTransactionFee(transactionFee);
          });
        }
      } catch (e) {
        console.error(e);
      }
    }
  }, [
    account,
    amountOne,
    amountTwo,
    executor,
    gasFeeRatio,
    gasFeeTokenDecimals,
    leftConfig?.asset,
    parachainApi,
    poolId,
    rightConfig?.asset,
    signer,
  ]);

  if (inputConfig === null) {
    return null;
  }

  return (
    <HighlightBox
      margin="auto"
      sx={{
        width: 550,
        [theme.breakpoints.down("sm")]: {
          width: "100%",
        },
      }}
      {...rest}
    >
      <FormTitle
        title="Add liquidity"
        onBackHandler={onBackHandler}
        onSettingHandler={onSettingHandler}
      />
      <Typography variant="subtitle1" textAlign="center" mt={4}>
        Use this tool to add tokens to the liquidity pool.
      </Typography>
      <LiquidityInput
        onValidationChange={setInValid}
        config={leftConfig}
        transactionFee={transactionFee}
        gasFeeToken={gasFeeToken}
        value={amountOne}
        onChange={(v) => {
          if (isPoolEmpty) {
            setAmount((state) => ({
              ...state,
              amountOne: v,
            }));
          } else {
            setAmount({
              amountOne: v,
              amountTwo: v.div(baseAmount).multipliedBy(quoteAmount),
            });
          }
        }}
        assetDropdownItems={assetOptions}
        label={"Token 1"}
      />

      <PlusIcon />

      <LiquidityInput
        onValidationChange={setOutValid}
        config={rightConfig}
        transactionFee={transactionFee}
        gasFeeToken={gasFeeToken}
        value={amountTwo}
        onChange={(v) => {
          if (isPoolEmpty) {
            setAmount((state) => ({
              ...state,
              amountTwo: v,
            }));
          } else {
            setAmount({
              amountOne: v.div(quoteAmount).multipliedBy(baseAmount),
              amountTwo: v, // amountTwo / Pool.amountTwo = RATIO
            });
          }
        }}
        assetDropdownItems={assetOptions}
        label={"Token 2"}
      />

      {pool ? (
        <PoolShare
          pool={pool}
          input={[leftConfig.asset, rightConfig.asset]}
          amounts={[amountOne, amountTwo]}
          simulated={simulated}
        />
      ) : null}

      <Button
        variant="contained"
        size="large"
        fullWidth
        sx={{
          mt: 2,
        }}
        disabled={!inputValid}
        onClick={handleConfirmSupplyButtonClick}
      >
        Supply
      </Button>

      {inputValid && pool ? (
        <YourPosition
          pool={pool}
          amounts={[amountOne, amountTwo]}
          noTitle={false}
          assets={inputConfig.map((config) => config.asset)}
          expectedLP={simulated}
          transactionFee={transactionFee}
          gasFeeToken={gasFeeToken}
          mt={4}
        />
      ) : null}

      {pool ? (
        <ConfirmSupplyModal
          pool={pool}
          inputConfig={inputConfig}
          expectedLP={simulated}
          share={new BigNumber(0)}
          open={isConfirmSupplyModalOpen}
          amountOne={amountOne}
          amountTwo={amountTwo}
        />
      ) : null}

      {pool ? (
        <ConfirmingSupplyModal
          pool={pool}
          inputConfig={inputConfig}
          expectedLP={simulated}
          share={new BigNumber(0)}
          open={isConfirmingSupplyModalOpen}
          amountOne={amountOne}
          amountTwo={amountTwo}
        />
      ) : null}

      <TransactionSettings showSlippageSelection={false} />
    </HighlightBox>
  );
};

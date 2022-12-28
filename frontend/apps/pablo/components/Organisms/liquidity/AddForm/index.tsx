import { BoxProps, Button, Typography, useTheme } from "@mui/material";
import { useRouter } from "next/router";
import { FormTitle } from "../../FormTitle";
import { TransactionSettings } from "../../TransactionSettings";
import { useSnackbar } from "notistack";
import { FC, useEffect, useState } from "react";
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
import { usePoolSpotPrice } from "@/defi/hooks/pools/usePoolSpotPrice";
import { useLiquidity } from "@/defi/hooks";

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
    option.fold(
      () => null,
      (ic) => ic
    )
  );
  const { spotPrice } = usePoolSpotPrice(pool, pool?.config.assets);
  const { baseAmount, quoteAmount } = useLiquidity(pool);
  const assetOptions = getAssetOptions(inputConfig ?? []);
  const [leftConfig, rightConfig] = inputConfig ?? [];
  const leftId = (leftConfig?.asset.getPicassoAssetId() as string) || null;
  const rightId = (rightConfig?.asset.getPicassoAssetId() as string) || null;
  const [simulated, setSimulated] = useState<BigNumber>(new BigNumber(0));
  const simulate = useSimulateAddLiquidity();
  const [isInValid, setInValid] = useState<boolean>(false);
  const [isOutValid, setOutValid] = useState<boolean>(false);
  const inputValid = isInValid && isOutValid;
  const isPoolEmpty = baseAmount.isZero() && quoteAmount.isZero();

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

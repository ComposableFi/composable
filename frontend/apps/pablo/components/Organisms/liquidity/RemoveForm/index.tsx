import { FormTitle, TransactionSettings, ValueSelector } from "@/components";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import {
  alpha,
  Box,
  Button,
  Divider,
  Slider,
  Typography,
  useTheme,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { useRouter } from "next/router";
import { FC, useEffect, useMemo, useState } from "react";
import { ConfirmingModal } from "./ConfirmingModal";
import { PreviewDetails } from "./PreviewDetails";
import useDebounce from "@/hooks/useDebounce";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { setUiState, useUiSlice } from "@/store/ui/ui.slice";
import { rest } from "lodash";
import { PoolConfig } from "@/store/pools/types";
import useStore from "@/store/useStore";
import { fromChainUnits, toChainUnits } from "@/defi/utils";
import { Asset } from "shared";

export const RemoveLiquidityForm: FC<{ pool: PoolConfig }> = ({ pool }) => {
  const theme = useTheme();
  const router = useRouter();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const ownedLiquidity = useStore((store) => store.ownedLiquidity.tokens);
  const ownedLiquidityBalance = ownedLiquidity[pool.config.lpToken];
  const lpBalance = useMemo(
    () => ownedLiquidityBalance?.balance.free ?? new BigNumber(0),
    [ownedLiquidityBalance?.balance.free]
  );
  const { isConfirmingModalOpen } = useUiSlice();
  const [token1, token2] = ownedLiquidity[pool.config.lpToken]?.pair ?? [
    new Asset("", "", "", "pica"),
    new Asset("", "", "", "pica"),
  ];

  const [percentage, setPercentage] = useState<number>(0);
  const [expectedRemoveAmountQuote, setExpectedRemoveAmountQuote] =
    useState<BigNumber>(new BigNumber(0));
  const [expectedRemoveAmountBase, setExpectedRemoveAmountBase] =
    useState<BigNumber>(new BigNumber(0));

  const [confirmed, setConfirmed] = useState<boolean>(false);
  const [priceOfBase, setPriceOfBase] = useState(new BigNumber(0));
  const [priceOfQuote, setPriceOfQuote] = useState(new BigNumber(0));
  const debouncedPercentage = useDebounce(percentage, 500);

  useEffect(() => {
    if (
      parachainApi &&
      debouncedPercentage > 0 &&
      lpBalance.gt(0) &&
      selectedAccount &&
      token1 &&
      token2
    ) {
      const selectedLpAmount = toChainUnits(
        lpBalance.times(debouncedPercentage / 100)
      );

      const b = pool.config.assets[0].getPicassoAssetId()?.toString() || "0";
      const q = pool.config.assets[1].getPicassoAssetId()?.toString() || "0";
      parachainApi.rpc.pablo
        .simulateRemoveLiquidity(
          selectedAccount.address,
          pool.poolId.toString(),
          selectedLpAmount.dp(0).toString(),
          {
            [b]: "0",
            [q]: "0",
          }
        )
        .then((response: any) => {
          const expectedRewards = response.toJSON().assets;
          setExpectedRemoveAmountBase(
            fromChainUnits(
              expectedRewards[b],
              token1.getDecimals(DEFAULT_NETWORK_ID)
            )
          );
          setPriceOfBase(
            new BigNumber(1).div(
              fromChainUnits(
                expectedRewards[b],
                token1.getDecimals(DEFAULT_NETWORK_ID)
              ).div(
                fromChainUnits(
                  expectedRewards[q],
                  token2.getDecimals(DEFAULT_NETWORK_ID)
                )
              )
            )
          );
          setExpectedRemoveAmountQuote(
            fromChainUnits(
              expectedRewards[q],
              token2.getDecimals(DEFAULT_NETWORK_ID)
            )
          );

          setPriceOfQuote(
            fromChainUnits(
              expectedRewards[b],
              token1.getDecimals(DEFAULT_NETWORK_ID)
            ).div(
              fromChainUnits(
                expectedRewards[q],
                token2.getDecimals(DEFAULT_NETWORK_ID)
              )
            )
          );
        })
        .catch((err: any) => {
          setExpectedRemoveAmountBase(new BigNumber(0));
          setExpectedRemoveAmountQuote(new BigNumber(0));
          console.error(err);
        });
    }
  }, [
    parachainApi,
    debouncedPercentage,
    lpBalance,
    selectedAccount,
    token1,
    token2,
    pool,
  ]);

  const onBackHandler = () => {
    router.push("/pool");
  };

  const onSettingHandler = () => {
    setUiState({ isTransactionSettingsModalOpen: true });
  };

  const onSliderChangeHandler = (_: Event, newValue: number | number[]) => {
    setPercentage(newValue as number);
  };

  const onRemoveHandler = async () => {
    setUiState({ isConfirmingModalOpen: true });
  };

  useEffect(() => {
    if (confirmed) {
      setUiState({ isConfirmingModalOpen: false });
    }
  }, [confirmed]);

  return (
    <Box
      borderRadius={1}
      margin="auto"
      sx={{
        width: 550,
        padding: theme.spacing(4),
        [theme.breakpoints.down("sm")]: {
          width: "100%",
          padding: theme.spacing(2),
        },
        background: theme.palette.gradient.secondary,
        boxShadow: `-1px -1px ${alpha(
          theme.palette.common.white,
          theme.custom.opacity.light
        )}`,
      }}
      {...rest}
    >
      <FormTitle
        title="Remove Liquidity"
        onBackHandler={onBackHandler}
        onSettingHandler={onSettingHandler}
      />

      <Box
        display="flex"
        alignItems="center"
        justifyContent="space-between"
        mt={4}
      >
        <Typography variant="body1">Amount</Typography>
        <Typography variant="body1">Detailed</Typography>
      </Box>

      <Typography variant="h5" mt={4}>
        {percentage}%
      </Typography>

      <Box mt={8}>
        <Slider
          aria-label="percentage"
          value={percentage}
          valueLabelDisplay="auto"
          onChange={confirmed ? undefined : onSliderChangeHandler}
          min={0}
          max={100}
          marks={[
            { value: 0, label: "0" },
            { value: 25 },
            { value: 50 },
            { value: 75 },
            { value: 100, label: "100" },
          ]}
        />
        <ValueSelector
          values={[25, 50, 75, 100]}
          unit="%"
          onChangeHandler={confirmed ? undefined : setPercentage}
        />
      </Box>

      <Box mt={4} textAlign="center">
        <Box
          width={56}
          height={56}
          borderRadius="50%"
          display="flex"
          border={`2px solid ${theme.palette.primary.main}`}
          justifyContent="center"
          alignItems="center"
          margin="auto"
        >
          <ExpandMoreIcon />
        </Box>
      </Box>

      {token1 && token2 && (
        <PreviewDetails
          lpToRemove={lpBalance.times(debouncedPercentage).div(100)}
          mt={4}
          token1={token1}
          token2={token2}
          expectedReceiveAmountToken1={expectedRemoveAmountBase}
          expectedReceiveAmountToken2={expectedRemoveAmountQuote}
          price1={priceOfBase}
          price2={priceOfQuote}
        />
      )}

      {!confirmed && (
        <Box
          display="flex"
          justifyContent="space-between"
          alignItems="center"
          mt={4}
          gap={2}
        >
          <Box width="100%">
            <Button
              variant="outlined"
              size="large"
              fullWidth
              disabled={!percentage || confirmed}
              onClick={onRemoveHandler}
            >
              {!percentage ? "Enter Amount" : "Remove"}
            </Button>
          </Box>
        </Box>
      )}

      {!confirmed && (
        <>
          <Box mt={6}>
            <Divider
              sx={{
                borderColor: alpha(
                  theme.palette.common.white,
                  theme.custom.opacity.main
                ),
              }}
            />
          </Box>
          {/* <YourPosition
            noTitle={false}
            noDivider
            tokenId1={tokenId1 as TokenId}
            tokenId2={tokenId2 as TokenId}
            pooledAmount1={pooledAmount1}
            pooledAmount2={pooledAmount2}
            amount={amount}
            share={share}
            mt={6}
          /> */}
        </>
      )}

      {!confirmed && token1 && token2 && (
        <ConfirmingModal
          lpBalance={lpBalance}
          percentage={new BigNumber(debouncedPercentage).div(100)}
          pool={pool}
          price1={priceOfBase}
          price2={priceOfQuote}
          baseAsset={token1}
          quoteAsset={token2}
          open={isConfirmingModalOpen}
          amount1={expectedRemoveAmountBase}
          amount2={expectedRemoveAmountQuote}
          setConfirmed={setConfirmed}
        />
      )}

      <TransactionSettings showSlippageSelection={false} />
    </Box>
  );
};

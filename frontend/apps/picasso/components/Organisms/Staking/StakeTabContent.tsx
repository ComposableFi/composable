import BigNumber from "bignumber.js";
import { Box, Button, Stack, Typography, useTheme } from "@mui/material";
import {
  callIf,
  formatNumber,
  fromPerbill,
  humanDateDiff,
  toChainIdUnit,
  unwrapNumberOrHex,
} from "shared";
import { AlertBox, BigNumberInput } from "@/components";
import { RadioButtonGroup } from "@/components/Molecules/RadioButtonGroup";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { FutureDatePaper } from "@/components/Atom/FutureDatePaper";
import { WarningAmberRounded } from "@mui/icons-material";
import { FC, useEffect, useState } from "react";
import { useStore } from "@/stores/root";
import { ApiPromise } from "@polkadot/api";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { RewardPool } from "@/stores/defi/polkadot/stakingRewards/slice";
import { getSigner, useExecutor } from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
import { EventRecord } from "@polkadot/types/interfaces/system";
import { SnackbarKey, useSnackbar } from "notistack";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

function transformRewardPool(rewardPoolsWrapped: any): RewardPool {
  return {
    owner: rewardPoolsWrapped,
    assetId: rewardPoolsWrapped.assetId.toString(),
    rewards: rewardPoolsWrapped.rewards,
    totalShares: unwrapNumberOrHex(rewardPoolsWrapped.totalShares.toString()),
    claimedShares: unwrapNumberOrHex(
      rewardPoolsWrapped.claimedShares.toString()
    ),
    endBlock: unwrapNumberOrHex(rewardPoolsWrapped.endBlock.toString()),
    lock: {
      ...rewardPoolsWrapped.lock,
      durationPresets: Object.fromEntries(
        Object.entries(rewardPoolsWrapped.lock.durationPresets).map(
          ([duration, multiplier]) => [
            duration,
            fromPerbill(multiplier as string),
          ]
        )
      ),
    },
    shareAssetId: rewardPoolsWrapped.shareAssetId.toString(),
    financialNftAssetId: rewardPoolsWrapped.financialNftAssetId.toString(),
  } as unknown as RewardPool;
}

async function fetchRewardPools(api: ApiPromise, assetId: number) {
  const rewardPoolsWrapped: any = (
    await api.query.stakingRewards.rewardPools(api.createType("u128", assetId))
  ).toJSON();

  if (!rewardPoolsWrapped) return null;

  return transformRewardPool(rewardPoolsWrapped);
}

function formatDurationOption(duration: string, multiplier: BigNumber) {
  const future = new Date();
  future.setSeconds(future.getSeconds() + parseInt(duration));
  const [diff, label] = humanDateDiff(new Date(), future);

  return `${diff} ${label} (${multiplier.toFixed(2).toString()}%)`;
}

type DurationOption = {
  [key in number]: string;
};

export const StakeTabContent: FC = () => {
  const theme = useTheme();
  const [lockablePICA, setLockablePICA] = useState<BigNumber>(new BigNumber(0));
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const { meta } = useStore(
    (state) => state.substrateBalances.picasso.assets.pica
  );
  const balance = useStore(
    (state) => state.substrateBalances.picasso.native.balance
  );
  const setRewardPool = useStore((state) => state.setRewardPool);
  const assetId = meta.supportedNetwork.picasso || 1;
  const picaRewardPool = useStore((state) => state.rewardPools[assetId]);

  const { parachainApi } = usePicassoProvider();

  const executor = useExecutor();

  useEffect(() => {
    callIf(parachainApi, (api) =>
      fetchRewardPools(api, assetId).then((pool) =>
        callIf(pool, (poolToStore) => setRewardPool(assetId, poolToStore))
      )
    );
  }, [parachainApi, setRewardPool]);

  const options: Array<{
    label: string;
    value: string;
  }> = Object.entries(picaRewardPool.lock.durationPresets).reduce(
    (acc, [duration, multiplier]) => [
      ...acc,
      {
        label: formatDurationOption(duration, multiplier),
        value: duration,
      },
    ],
    [] as any
  );

  const [lockPeriod, setLockPeriod] = useState<string>("");
  const match = (option?: DurationOption) => lockPeriod === option;
  const account = useSelectedAccount();

  const setValidation = () => {};
  return (
    <Stack sx={{ marginTop: theme.spacing(9) }} gap={4}>
      <Stack gap={1.5}>
        <Box
          display="flex"
          width="100%"
          justifyContent="space-between"
          alignItems="center"
        >
          <Typography variant="inputLabel">Amount to lock</Typography>
          <Box display="flex" gap={1}>
            <Typography variant="inputLabel" color="text.secondary">
              Balance:
            </Typography>
            <Typography variant="inputLabel">
              {formatNumber(balance)}&nbsp;
              {meta.symbol}
            </Typography>
          </Box>
        </Box>
        <BigNumberInput
          isValid={setValidation}
          setter={setLockablePICA}
          maxValue={balance}
          value={lockablePICA}
          tokenId={meta.assetId}
          maxDecimals={18}
        />
      </Stack>
      {/*  Radiobutton groups*/}
      <RadioButtonGroup<DurationOption>
        label="Lock period (multiplier)"
        tooltip="Lock period (multiplier)"
        options={options}
        value={lockPeriod}
        isMatch={match}
        onChange={(v: any) => setLockPeriod(v)}
        sx={{
          marginTop: theme.spacing(4),
        }}
      />
      {/* Unlock date */}
      <TextWithTooltip tooltip="Unlock date">Unlock date</TextWithTooltip>
      <FutureDatePaper duration={lockPeriod} />
      <AlertBox status="warning" icon={<WarningAmberRounded color="warning" />}>
        <Typography variant="body2">Warning</Typography>
        <Typography variant="inputLabel" color="text.secondary">
          Your {meta.symbol} will be locked until the expiry date.
        </Typography>
      </AlertBox>
      <Button
        fullWidth
        onClick={async () => {
          let snackbarKey: SnackbarKey | undefined;
          if (executor && parachainApi && account) {
            const signer = await getSigner(APP_NAME, account.address);
            console.log("initiating staking protocol");
            await executor.execute(
              parachainApi.tx.stakingRewards.stake(
                assetId.toString(),
                parachainApi.createType(
                  "u128",
                  toChainIdUnit(lockablePICA).toString()
                ),
                parachainApi.createType("u64", lockPeriod.toString())
              ),
              account.address,
              parachainApi,
              signer,
              (txHash: string) => {
                snackbarKey = enqueueSnackbar("Processing stake on the chain", {
                  variant: "info",
                  isClosable: true,
                  persist: true,
                  url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
                });
              },
              (txHash: string, events: EventRecord[]) => {
                closeSnackbar(snackbarKey);
                enqueueSnackbar(
                  `Successfully staked ${lockablePICA
                    .toFixed()
                    .toString()} PICA`,
                  {
                    variant: "success",
                    isClosable: true,
                    persist: true,
                    url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
                  }
                );
              },
              (errorMessage: string) => {
                closeSnackbar(snackbarKey);
                enqueueSnackbar(
                  "An error occurred while processing transaction",
                  {
                    variant: "error",
                    isClosable: true,
                    persist: true,
                    description: errorMessage,
                  }
                );
              }
            );
          }
        }}
        variant="contained"
        color="primary"
        disabled={!lockablePICA.isGreaterThan(0) || !lockPeriod}
      >
        <Typography variant="button">Lock and mint</Typography>
      </Button>
    </Stack>
  );
};

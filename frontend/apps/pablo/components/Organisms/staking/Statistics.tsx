import { Label } from "@/components/Atoms";
import {
  Box,
  Grid,
  TooltipProps as MuiTooltipProps,
  Typography,
  useTheme,
} from "@mui/material";
import { GridProps } from "@mui/system";
import { BoxWrapper } from "../BoxWrapper";
import { TokenValue } from "@/components/Molecules";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { useTotalXTokensIssued } from "@/defi/hooks/stakingRewards/useTotalXTokensIssued";
import { StakingRewardPool } from "@/defi/types";
import {
  useAverageLockTimeAndMultiplier,
  useStakingRewardsPoolApy,
} from "@/defi/hooks/stakingRewards";
import {
  calculatePeriod,
  createDurationPresetLabel,
} from "@/defi/utils/stakingRewards/durationPresets";
import millify from "millify";
import { useMemo } from "react";
import { useAssets } from "@/defi/hooks";
import BigNumber from "bignumber.js";

const threeColumnPageSize = {
  xs: 12,
  md: 4,
};

const twoColumnPageSize = {
  xs: 12,
  md: 6,
};

const defaultFlexBoxProps = {
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  gap: 3,
};

const defaultTokenValueProps = {
  gap: 6,
  LabelProps: { color: "primary.dark" },
  ValueProps: { color: "primary.dark" },
};

type ItemProps = {
  label: string;
  value: string;
  TooltipProps?: Omit<MuiTooltipProps, "children">;
};
const Item: React.FC<ItemProps> = ({ label, TooltipProps, value }) => {
  return (
    <BoxWrapper textAlign="center">
      <Label
        justifyContent="center"
        mb={0}
        label={label}
        TypographyProps={{
          variant: "body1",
          color: "text.secondary",
        }}
        TooltipProps={TooltipProps}
      />
      <Typography variant="h6" mt={0.5}>
        {value}
      </Typography>
    </BoxWrapper>
  );
};

export const StakingStatistics: React.FC<
  GridProps & { stakingRewardPool?: StakingRewardPool, rewardPoolId?: string }
> = ({ stakingRewardPool, rewardPoolId, ...gridProps }) => {
  const theme = useTheme();

  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const xTokensMinted = useTotalXTokensIssued({
    api: parachainApi,
    shareAssetId: stakingRewardPool?.shareAssetId,
  });

  const apy = useStakingRewardsPoolApy(rewardPoolId);
  const totalApy = useMemo(() => {
    return Object.keys(apy).reduce((v, i) => {
      return v.plus(apy[i]);
    }, new BigNumber(0));
  }, [apy]);

  const { averageLockMultiplier, averageLockTime, totalValueLocked } =
    useAverageLockTimeAndMultiplier();
  let _totalValeLocked = millify(totalValueLocked.toNumber());

  const assets = useAssets(
    stakingRewardPool ? Object.keys(stakingRewardPool.rewards) : []
  );

  const apyTooltip = useMemo(() => {
    return (
      <Box {...defaultFlexBoxProps} p={3}>
        {assets.map((asset) => {
          const assetApy = apy[asset.getPicassoAssetId() as string];
          return (
            <TokenValue
              token={asset}
              value={`${assetApy}%`}
              {...defaultTokenValueProps}
            />
          );
        })}
      </Box>
    );
  }, [assets, apy]);

  return (
    <Grid container spacing={3} {...gridProps}>
      <Grid item {...threeColumnPageSize}>
        <Item
          label="Total Value locked"
          value={_totalValeLocked}
          TooltipProps={{ title: "Total value locked" }}
        />
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Item
          label="Total xPABLO APY"
          value={`${totalApy}%`}
          TooltipProps={{
            title: apyTooltip,
          }}
        />
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Item
          label="Total xPABLO minted"
          value={xTokensMinted.toFormat(DEFAULT_UI_FORMAT_DECIMALS)}
          TooltipProps={{ title: "Total xPABLO Minted" }}
        />
      </Grid>
      <Grid item {...twoColumnPageSize}>
        <Item
          label="Average lock multiplier"
          value={`${averageLockMultiplier.toString()}x`}
          TooltipProps={{ title: "Average lock multiplier" }}
        />
      </Grid>
      <Grid item {...twoColumnPageSize}>
        <Item
          label="Average lock time"
          value={`${createDurationPresetLabel(
            calculatePeriod(averageLockTime)
          )}`}
          TooltipProps={{ title: "Average lock time" }}
        />
      </Grid>
    </Grid>
  );
};

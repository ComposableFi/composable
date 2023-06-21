import { Label } from "@/components/Atoms";
import {
  Box,
  Grid,
  TooltipProps as MuiTooltipProps,
  Typography,
} from "@mui/material";
import { GridProps } from "@mui/system";
import { BoxWrapper } from "../BoxWrapper";
import { TokenValue } from "@/components/Molecules";
import { DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { StakingRewardPool } from "@/defi/types";
import {
  calculatePeriod,
  createDurationPresetLabel,
} from "@/defi/utils/stakingRewards/durationPresets";
import millify from "millify";
import { FC, useMemo } from "react";
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
const Item: FC<ItemProps> = ({ label, TooltipProps, value }) => {
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

export const StakingStatistics: FC<
  GridProps & { stakingRewardPool?: StakingRewardPool; rewardPoolId?: string }
> = ({ stakingRewardPool, rewardPoolId, ...gridProps }) => {
  const xTokensMinted = new BigNumber(0);
  const totalApy = new BigNumber(0);

  const averageLockMultiplier = 0;
  const averageLockTime = 0;
  const totalValueLocked = new BigNumber(0);
  let _totalValeLocked = millify(totalValueLocked.toNumber());

  const apyTooltip = useMemo(() => {
    return (
      <Box {...defaultFlexBoxProps} p={3}>
        {[].map((asset) => {
          const assetApy = new BigNumber(0);
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
  }, []);

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

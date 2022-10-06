import { Label } from "@/components/Atoms";
import { TOKENS } from "@/defi/Tokens";
import { useAppSelector } from "@/hooks/store";
import {
  Box,
  useTheme,
  Grid,
  Typography,
  TooltipProps as MuiTooltipProps,
} from "@mui/material";
import { GridProps } from "@mui/system";
import { BoxWrapper } from "../BoxWrapper";
import { TokenValue } from "@/components/Molecules";
import { useStakingRewardsSlice } from "@/store/stakingRewards/stakingRewards.slice";
import { useParachainApi } from "substrate-react";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { useTotalXTokensIssued } from "@/defi/hooks/stakingRewards/useTotalXTokensIssued";
import { StakingRewardPool } from "@/defi/types";
import { useAverageLockTimeAndMultiplier } from "@/defi/hooks/stakingRewards";
import { calculatePeriod, createDurationPresetLabel } from "@/defi/utils/stakingRewards/durationPresets";

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

export const StakingStatistics: React.FC<GridProps & { stakingRewardPool?: StakingRewardPool }> = ({ stakingRewardPool, ...gridProps }) => {
  const theme = useTheme();

  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  const xTokensMinted = useTotalXTokensIssued({
    api: parachainApi,
    shareAssetId: stakingRewardPool?.shareAssetId
  });

  const { averageLockMultiplier, averageLockTime, totalValueLocked } = useAverageLockTimeAndMultiplier();

  const {
    totalChaosApy,
    totalKsmApy,
    totalPicaApy,
    totalPabloApy,
  } = useAppSelector((state) => state.polkadot.stakingOverview);

  const totalApyTooltip = (
    <Box {...defaultFlexBoxProps} p={3}>
      <TokenValue
        token={TOKENS.ksm}
        value={`${totalKsmApy}%`}
        {...defaultTokenValueProps}
      />
      <TokenValue
        token={TOKENS.pica}
        value={`${totalPicaApy}%`}
        {...defaultTokenValueProps}
      />
      <TokenValue
        token={TOKENS.pablo}
        value={`${totalPabloApy}%`}
        {...defaultTokenValueProps}
      />
    </Box>
  );

  return (
    <Grid container spacing={8} {...gridProps}>
      <Grid item {...threeColumnPageSize}>
        <Item
          label="Total Value locked"
          value={totalValueLocked}
          TooltipProps={{ title: "Total value locked" }}
        />
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Item
          label="Total CHAOS APY"
          value={`${totalChaosApy}%`}
          TooltipProps={{
            title: totalApyTooltip,
          }}
        />
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Item
          label="Total xPABLO minted"
          value={xTokensMinted.toFormat(DEFAULT_UI_FORMAT_DECIMALS)}
          TooltipProps={{title: "Total CHAOS Minted"}}
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
          value={`${createDurationPresetLabel(calculatePeriod(averageLockTime))}`}
          TooltipProps={{ title: "Average lock time" }}
        />
      </Grid>
    </Grid>
  );
};

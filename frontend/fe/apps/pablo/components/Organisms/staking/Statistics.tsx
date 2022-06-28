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
  LabelProps: {color: "primary.dark"},
  ValueProps: {color: "primary.dark"},
};

type ItemProps = {
  label: string;
  value: string;
  TooltipProps?: Omit<MuiTooltipProps, "children">;
}
const Item: React.FC<ItemProps> = ({
  label,
  TooltipProps,
  value,
}) => {
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
  )
};

export const StakingStatistics: React.FC<GridProps> = ({
  ...gridProps
}) => {
  const theme = useTheme();

  const {
    totalPBLOLocked,
    totalChaosApy,
    totalKsmApy,
    totalPicaApy,
    totalPabloApy,
    totalChaosMinted,
    averageLockMultiplier,
    averageLockTime,
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
  )

  return (
    <Grid container spacing={8} {...gridProps}>
      <Grid item {...threeColumnPageSize}>
        <Item
          label="Total PBLO locked"
          value={totalPBLOLocked.toFormat()}
          TooltipProps={{title: "Total value locked"}}
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
          label="Total CHAOS minted"
          value={totalChaosMinted.toFormat()}
          TooltipProps={{title: "Total CHAOS Minted"}}
        />
      </Grid>
      <Grid item {...twoColumnPageSize}>
        <Item
          label="Average lock multiplier"
          value={averageLockMultiplier.toString()}
          TooltipProps={{title: "Average lock multiplier"}}
        />
      </Grid>
      <Grid item {...twoColumnPageSize}>
        <Item
          label="Average lock time"
          value={`${averageLockTime} days`}
          TooltipProps={{title: "Average lock time"}}
        />
      </Grid>
    </Grid>
  )
};

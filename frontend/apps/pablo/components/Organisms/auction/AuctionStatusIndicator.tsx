import {
  Box,
  BoxProps,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";
import FiberManualRecordIcon from '@mui/icons-material/FiberManualRecord';
import { getHumanizedDateDiff, PabloLiquidityBootstrappingPool } from "shared";
import { useCallback } from "react";
import { useAuctionTiming } from "@/defi/hooks";

export type AuctionStatusIndicatorProps = {
  auction: PabloLiquidityBootstrappingPool,
  labelWithDuration?: boolean,
  label?: string,
  LabelProps?: TypographyProps,
} & BoxProps;

export const AuctionStatusIndicator: React.FC<AuctionStatusIndicatorProps> = ({
  auction,
  labelWithDuration = false,
  label,
  LabelProps,
  ...rest
}) => {
  const theme = useTheme();
  const { isActive, isEnded, willStart, startTimestamp, endTimestamp } = useAuctionTiming(auction);

  const getLabel = useCallback(() => {
    if (willStart) {
      if (!labelWithDuration) {
        return "Starting Soon";
      } else {
        let dateDiff = getHumanizedDateDiff(
          Date.now(),
          startTimestamp
        )

        return `Starts in ${dateDiff}`
      }
    } else if (isActive) {
      if (!labelWithDuration) {
        return "Active";
      } else {
        let dateDiff = getHumanizedDateDiff(
          Date.now(),
          endTimestamp
        )

        return `Ends in ${dateDiff}`
      }
    } else if (isEnded) {
      if (!labelWithDuration) {
        return "Ended";
      } else {
        let dateDiff = getHumanizedDateDiff(
          Date.now(),
          endTimestamp
        )

        return `Ended ${dateDiff}`
      }
    }
  }, [willStart, isActive, isEnded, labelWithDuration, startTimestamp, endTimestamp])

  return (
    <Box display="flex" alignItems="center" gap={1.5} {...rest}>
      <FiberManualRecordIcon
        sx={{
          color: (
            isActive
              ? theme.palette.success.main
              : (
                isEnded
                  ? theme.palette.error.main
                  : theme.palette.warning.main
              )
          ),
        }}
      />
      <Typography variant="body1" {...LabelProps}>
        {getLabel()}
      </Typography>
    </Box>
  );
}

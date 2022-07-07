import {
  Box, 
  BoxProps, 
  Typography, 
  TypographyProps, 
  useTheme, 
} from "@mui/material";
import FiberManualRecordIcon from '@mui/icons-material/FiberManualRecord';
import { getHumanizedDateDiff } from "@/utils/date";
import { LiquidityBootstrappingPool } from "@/defi/types";

export type AuctionStatusIndicatorProps = {
  auction: LiquidityBootstrappingPool,
  label?: string,
  LabelProps?: TypographyProps,
} & BoxProps;

export const AuctionStatusIndicator: React.FC<AuctionStatusIndicatorProps> = ({
  auction,
  label,
  LabelProps,
  ...rest
}) => {

  const theme = useTheme();
  const currentTimestamp = Date.now();
  const isActive: boolean = auction.sale.start <= currentTimestamp 
                    && auction.sale.end >= currentTimestamp;
  const isEnded: boolean = auction.sale.end < currentTimestamp;

  const getLabel = () => {
    if (!!label) {
      return label;
    }

    return (
      isActive
        ? "Ends in " + getHumanizedDateDiff(Date.now(), auction.sale.end)
        : (
            isEnded
              ? "Ended " + getHumanizedDateDiff(Date.now(), auction.sale.end) + " ago"
              : "Starts in " + getHumanizedDateDiff(Date.now(), auction.sale.start)
        )
    )
  }

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
        { getLabel() }
      </Typography>
    </Box>
  );
}
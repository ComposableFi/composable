import { BaseAsset, PairAsset } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { getNetwork } from "@/defi/Networks";
import { BondDetails } from "@/defi/types";
import { ArrowRightAlt } from "@mui/icons-material";
import {
  Box,
  BoxProps,
  Typography,
  TypographyProps,
  Theme,
  useTheme,
  alpha,
} from "@mui/material";
import TimerOutlinedIcon from "@mui/icons-material/TimerOutlined";
import { ISupplySummary } from "../../../store/bonds/bonds.types";

const containerBoxProps = (theme: Theme) => ({
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  p: 4,
  borderRadius: 1.5,
  sx: {
    background: theme.palette.gradient.secondary,
    border: `1px solid ${alpha(
      theme.palette.common.white,
      theme.custom.opacity.light
    )}`,
  },
});

const itemBoxProps: BoxProps = {
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  gap: 3.5,
};

const itemTitleProps: TypographyProps = {
  variant: "body1",
  fontWeight: "600",
  color: "text.secondary",
};

export type SupplySummaryProps = {
  bond: BondDetails;
  summary: ISupplySummary;
} & BoxProps;

export const SupplySummary: React.FC<SupplySummaryProps> = ({
  bond,
  summary,
  ...boxProps
}) => {
  const theme = useTheme();
  const pablo = getToken("pablo");
  const ethereum = getNetwork(1);

  return (
    <Box {...containerBoxProps(theme)} {...boxProps}>
      <Box
        display="flex"
        justifyContent="center"
        alignItems="center"
        gap={5.25}
      >
        <Box {...itemBoxProps}>
          <Typography {...itemTitleProps}>Supply</Typography>
          {"base" in summary.principalAsset ? (
            <PairAsset
              assets={[
                {
                  icon: summary.principalAsset.base.icon,
                  label: summary.principalAsset.base.symbol,
                },
                {
                  icon: summary.principalAsset.quote.icon,
                  label: summary.principalAsset.quote.symbol,
                },
              ]}
              iconOnly
              iconSize={36}
            />
          ) : (
            <BaseAsset
              label={summary.principalAsset.symbol}
              icon={summary.principalAsset.icon}
            />
          )}
          <Typography variant="body1">
            {"base" in summary.principalAsset
              ? `LP ${summary.principalAsset.base.symbol}-${summary.principalAsset.quote.symbol}`
              : summary.principalAsset.symbol}
          </Typography>
        </Box>
        <ArrowRightAlt sx={{ color: "text.secondary" }} />
        <Box {...itemBoxProps}>
          <Typography {...itemTitleProps}>Receive</Typography>
          <BaseAsset icon={pablo.icon} iconSize={36} />
          <Typography variant="body1">
            {`${pablo.symbol} - `}
            <Typography variant="body1" fontWeight="600" component="span">
              {`${summary.roi}%`}
            </Typography>
          </Typography>
        </Box>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Vesting period</Typography>
        <TimerOutlinedIcon sx={{ width: 36, height: 36 }} />
        <Typography variant="body1">{summary.vestingPeriod}</Typography>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Market Price</Typography>
        <Box display="flex" justifyContent="center" alignItems="center">
          {`$${summary.marketPriceInUSD}`}
        </Box>
        <Typography variant="body1">{pablo.symbol}</Typography>
      </Box>
    </Box>
  );
};

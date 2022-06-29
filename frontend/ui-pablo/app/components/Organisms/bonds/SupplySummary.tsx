import { BaseAsset, PairAsset } from "@/components/Atoms";
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
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { MockedAsset } from "@/store/assets/assets.types";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";

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
  bond: SelectedBondOffer;
} & BoxProps;

export const SupplySummary: React.FC<SupplySummaryProps> = ({
  bond,
  ...boxProps
}) => {
  const theme = useTheme();
  const { principalAsset, rewardAsset } = bond;
  const rewardPriceInUSD = useUSDPriceByAssetId(
    rewardAsset ? rewardAsset.network[DEFAULT_NETWORK_ID] : "-"
  );

  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;

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
          {baseAsset && quoteAsset ? (
            <PairAsset
              assets={[
                {
                  icon: baseAsset.icon,
                  label: baseAsset.symbol,
                },
                {
                  icon: quoteAsset.icon,
                  label: quoteAsset.symbol,
                },
              ]}
              iconOnly
              iconSize={36}
            />
          ) : simplePrincipalAsset ? (
            <BaseAsset
              label={simplePrincipalAsset.symbol}
              icon={simplePrincipalAsset.icon}
              LabelProps={{ variant: "h4" }}
              iconSize={36}
            />
          ) : null}
          <Typography variant="body1">
            {baseAsset && quoteAsset
              ? `LP ${baseAsset.symbol}-${quoteAsset.symbol}`
              : simplePrincipalAsset
              ? `${simplePrincipalAsset.symbol}`
              : "-"}
          </Typography>
        </Box>
        <ArrowRightAlt sx={{ color: "text.secondary" }} />
        <Box {...itemBoxProps}>
          <Typography {...itemTitleProps}>Receive</Typography>
          {rewardAsset && (
            <BaseAsset
              label={(rewardAsset as MockedAsset).symbol}
              icon={(rewardAsset as MockedAsset).icon}
              LabelProps={{ variant: "h4" }}
              iconSize={36}
            />
          )}
          <Typography variant="body1">
            {rewardAsset && `${rewardAsset.symbol}`}
            <Typography variant="body1" fontWeight="600" component="span">
              {`${bond.roi}%`}
            </Typography>
          </Typography>
        </Box>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Vesting period</Typography>
        <TimerOutlinedIcon sx={{ width: 36, height: 36 }} />
        <Typography variant="body1">{bond.vestingPeriod}</Typography>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Market Price</Typography>
        <Box display="flex" justifyContent="center" alignItems="center">
          {`$${rewardPriceInUSD}`}
        </Box>
        <Typography variant="body1">{rewardAsset?.symbol}</Typography>
      </Box>
    </Box>
  );
};

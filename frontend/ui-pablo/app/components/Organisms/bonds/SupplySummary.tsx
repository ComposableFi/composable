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
} & BoxProps;

export const SupplySummary: React.FC<SupplySummaryProps> = ({
  bond,
  ...boxProps
}) => {
  const theme = useTheme();
  const token1 = getToken(bond.tokenId1);
  const token2 = getToken(bond.tokenId2);
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
          <PairAsset
            assets={[{ icon: token1.icon }, { icon: token2.icon }]}
            iconOnly
            iconSize={36}
          />
          <Typography variant="body1">
            {`LP ${token1.symbol}-${token2.symbol}`}
          </Typography>
        </Box>
        <ArrowRightAlt sx={{ color: "text.secondary" }} />
        <Box {...itemBoxProps}>
          <Typography {...itemTitleProps}>Receive</Typography>
          <BaseAsset icon={pablo.icon} iconSize={36} />
          <Typography variant="body1">
            {`${pablo.symbol} - `}
            <Typography variant="body1" fontWeight="600" component="span">
              {`${bond.roi}%`}
            </Typography>
          </Typography>
        </Box>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>Vesting period</Typography>
        <TimerOutlinedIcon sx={{ width: 36, height: 36 }} />
        <Typography variant="body1">{`${bond.vesting_term} days`}</Typography>
      </Box>

      <Box {...itemBoxProps}>
        <Typography {...itemTitleProps}>
          Discount Price / Market Price
        </Typography>
        <Box display="flex" justifyContent="center" alignItems="center">
          <BaseAsset
            icon={ethereum.logo}
            label={ethereum.name}
            LabelProps={{
              variant: "h6",
            }}
            iconSize={36}
          />
        </Box>
        <Typography variant="body1">{pablo.symbol}</Typography>
      </Box>
    </Box>
  );
};

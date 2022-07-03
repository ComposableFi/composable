import { Label, PairAsset } from "@/components/Atoms";
import { MockedAsset } from "@/store/assets/assets.types";
import { alpha, Box, BoxProps, Typography, useTheme } from "@mui/material";
import BigNumber from "bignumber.js";

type YourPositionProps = {
  noTitle?: boolean;
  noDivider?: boolean;
  token1: MockedAsset | undefined;
  token2: MockedAsset | undefined;
  pooledAmount1: BigNumber;
  pooledAmount2: BigNumber;
  amount: BigNumber;
  share: BigNumber;
} & BoxProps;

export const YourPosition: React.FC<YourPositionProps> = ({
  noTitle,
  noDivider,
  token1,
  token2,
  pooledAmount1,
  pooledAmount2,
  amount,
  share,
  ...rest
}) => {
  const theme = useTheme();

  return (
    <Box
      borderTop={
        noDivider
          ? undefined
          : `1px solid ${alpha(
              theme.palette.common.white,
              theme.custom.opacity.main
            )}`
      }
      {...rest}
    >
      {!noTitle && (
        <Typography variant="h6" mt={4}>
          Your position
        </Typography>
      )}
      <Label
        mt={noTitle ? 3 : 4}
        BalanceProps={{
          balance: amount.toString(),
          BalanceTypographyProps: {
            variant: "body1",
            fontWeight: "bold",
          },
        }}
      >
        {token1 && token2 && (
          <PairAsset
            assets={[
              {
                icon: token2.icon,
                label: token2.symbol,
              },
              {
                icon: token1.icon,
                label: token1.symbol,
              },
            ]}
            separator="/"
          />
        )}
      </Label>

      <Label
        mt={3}
        label="Share of pool"
        TypographyProps={{ variant: "body1" }}
        BalanceProps={{
          balance: `${share.toFixed(4)}%`,
          BalanceTypographyProps: {
            variant: "body1",
            fontWeight: "bold",
          },
        }}
      />

      <Label
        mt={3}
        label={`Pooled ${token1?.symbol}`}
        TypographyProps={{ variant: "body1" }}
        BalanceProps={{
          balance: pooledAmount1.toString(),
          BalanceTypographyProps: {
            variant: "body1",
            fontWeight: "bold",
          },
        }}
      />

      <Label
        mt={3}
        label={`Pooled ${token2?.symbol}`}
        TypographyProps={{ variant: "body1" }}
        BalanceProps={{
          balance: pooledAmount2.toString(),
          BalanceTypographyProps: {
            variant: "body1",
            fontWeight: "bold",
          },
        }}
      />
    </Box>
  );
};

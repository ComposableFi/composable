import BigNumber from "bignumber.js";
import { Token } from "tokens";
import { AssetRatio, formatNumber, SubstrateNetworkId } from "shared";
import { Box, Typography, useTheme } from "@mui/material";

export function StakeInputLabel(props: {
  amount: BigNumber;
  pica: Token & {
    chainId: {
      picasso: BigNumber | null;
      karura: string | null;
      kusama: string | null;
      statemine: string | null;
    };
    ratio: Record<SubstrateNetworkId, AssetRatio | null>;
    decimals: Record<SubstrateNetworkId, number | null>;
    existentialDeposit: Record<SubstrateNetworkId, BigNumber | null>;
  };
}) {
  const theme = useTheme();
  return (
    <Box
      display="flex"
      width="100%"
      justifyContent="space-between"
      alignItems="center"
    >
      <Typography variant="inputLabel">Enter amount to lock</Typography>
      <Box display="flex" gap={1}>
        <Typography variant="inputLabel">
          Balance:
        </Typography>
        <Typography variant="body2" fontSize="1rem">
          {formatNumber(props.amount)}&nbsp;
          {props.pica.symbol}
        </Typography>
      </Box>
    </Box>
  );
}

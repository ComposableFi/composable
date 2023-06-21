import { alpha, Box, useTheme } from "@mui/material";
import { SwapVertRounded } from "@mui/icons-material";
import { useSwaps } from "@/defi/hooks";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { FC } from "react";

export const SwapVertAsset: FC<{
  selectedAccount?: InjectedAccountWithMeta;
}> = ({ selectedAccount }) => {
  const theme = useTheme();

  const { flipAssetSelection } = useSwaps({ selectedAccount });
  return (
    <Box mt={4} textAlign="center">
      <Box
        width={56}
        height={56}
        borderRadius="50%"
        display="flex"
        border={`2px solid ${theme.palette.primary.main}`}
        justifyContent="center"
        alignItems="center"
        margin="auto"
        sx={{
          cursor: "pointer",
          "&:hover": {
            background: alpha(theme.palette.primary.light, 0.1),
          },
        }}
      >
        <SwapVertRounded onClick={flipAssetSelection} />
      </Box>
    </Box>
  );
};

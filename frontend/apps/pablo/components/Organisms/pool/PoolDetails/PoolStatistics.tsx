import { alpha, Box, Typography, useTheme } from "@mui/material";
import { PoolDetailsProps } from "./index";
import { FC } from "react";
import { usePoolRatio } from "@/defi/hooks/pools/usePoolRatio";
import { HighlightBox } from "@/components/Atoms/HighlightBox";

export const PoolStatistics: FC<PoolDetailsProps> = ({ pool, ...boxProps }) => {
  const { poolTVL } = usePoolRatio(pool);
  const theme = useTheme();
  return (
    <Box {...boxProps}>
      <HighlightBox
        variant="outlined"
        sx={{
          border: `1px solid ${alpha(theme.palette.common.white, 0.1)}`,
          background: alpha(theme.palette.common.white, 0.05),
        }}
      >
        <Typography variant="h5" textAlign="left" mb={4}>
          Pool value
        </Typography>
        <Box
          sx={{
            height: theme.spacing(18),
            minHeight: theme.spacing(18),
            display: "flex",
            alignItems: "start",
            flexDirection: "column",
            justifyContent: "flex-start",
          }}
          gap={2}
        >
          <Typography variant="body1" textAlign="left">
            ${poolTVL.toFormat(2)}
          </Typography>
        </Box>
      </HighlightBox>
    </Box>
  );
};

import {
  alpha,
  Box,
  Button,
  Paper,
  Stack,
  Theme,
  Typography,
  useTheme,
} from "@mui/material";
import { FC } from "react";
import { TokenAsset } from "@/components";

const boxStyles = (theme: Theme) => ({
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-start",
  border: `1px solid ${alpha(theme.palette.common.white, 0.3)}`,
  padding: theme.spacing(3),
  borderRadius: `${theme.shape.borderRadius}px`,
});

export const ClaimableRewards: FC<{
  onClaimButtonClick: () => void;
}> = ({ onClaimButtonClick }) => {
  const theme = useTheme();
  const claimable = "23,309 PICA";
  const usdValue = "(~$34,567)";

  return (
    <Paper sx={{ padding: theme.spacing(6) }}>
      <Stack gap={6}>
        <Typography variant="h6">Claimable $PICA Rewards</Typography>
        <Box
          display="flex"
          alignItems="center"
          justifyContent="space-between"
          width="100%"
          sx={{
            p: 4,
            borderRadius: 1,
            border: `1px solid ${alpha(
              theme.palette.common.white,
              theme.custom.opacity.light
            )}`,
          }}
        >
          <Box>
            <TokenAsset tokenId={"pica"} label="PICA" />
          </Box>
          <Box display="flex" alignItems="center" gap={1}>
            <Typography variant="body2">{claimable}</Typography>
            <Typography
              variant="body2"
              sx={{
                color: alpha(theme.palette.common.white, 0.6),
              }}
            >
              {usdValue}
            </Typography>
          </Box>
        </Box>
        <Button
          variant="outlined"
          color="primary"
          fullWidth
          onClick={onClaimButtonClick}
        >
          Claim rewards
        </Button>
      </Stack>
    </Paper>
  );
};

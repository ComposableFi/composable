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
import { useStore } from "@/stores/root";

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
  const { pica, pablo, angl } = useStore(
    ({ staking }) => staking.claimableRewards
  );

  const theme = useTheme();

  return (
    <Paper sx={{ padding: theme.spacing(6) }}>
      <Stack gap={6}>
        <Typography variant="h6">Claimable Rewards</Typography>
        <Box
          display="flex"
          alignItems="center"
          justifyContent="space-between"
          width="100%"
          gap={2}
        >
          <Box sx={boxStyles} gap={2} width="100%">
            <div>
              <TokenAsset tokenId={"pica"} label="PICA" />
            </div>
            <Typography variant="body2">{pica.toFixed()}</Typography>
          </Box>
          <Box sx={boxStyles} gap={2} width="100%">
            <div>
              <TokenAsset tokenId={"pablo"} label="PABLO" />
            </div>
            <Typography variant="body2">{pablo.toFixed()}</Typography>
          </Box>
          <Box sx={boxStyles} gap={2} width="100%">
            <div>
              <TokenAsset tokenId={"angl"} label="ANGL" />
            </div>
            <Typography variant="body2">{angl.toFixed()}</Typography>
          </Box>
        </Box>
        <Button
          variant="outlined"
          color="primary"
          fullWidth
          onClick={onClaimButtonClick}
        >
          Claim all
        </Button>
      </Stack>
    </Paper>
  );
};

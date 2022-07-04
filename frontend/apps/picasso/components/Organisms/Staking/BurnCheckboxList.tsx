import {
  Box,
  Button,
  Checkbox,
  Stack,
  Typography,
  useTheme
} from "@mui/material";
import { AlertBox, TokenAsset } from "@/components";
import { formatNumberWithSymbol } from "shared";
import { WarningAmberRounded } from "@mui/icons-material";
import { useStore } from "@/stores/root";
import { FC } from "react";

export const BurnCheckboxList: FC<{
  openBurnModal: () => void;
  openRenewModal: () => void;
  onSelectUnstakeToken: (id: string) => void;
  unstakeTokenId?: string;
}> = ({
  openBurnModal,
  openRenewModal,
  onSelectUnstakeToken,
  unstakeTokenId
}) => {
  const openPositions = useStore(({ staking }) => staking.openPositions);
  const theme = useTheme();
  return (
    <Stack gap={4} marginTop={9}>
      {openPositions.map(item => (
        <>
          <Button
            variant="outlined"
            fullWidth
            onClick={() => {
              onSelectUnstakeToken(item.id);
            }}
            sx={{
              padding: theme.spacing(1.5, 2),
              height: "auto"
            }}
          >
            <Box
              width="100%"
              display="flex"
              justifyContent="space-between"
              alignItems="center"
            >
              <Stack direction="row" gap={1}>
                <Checkbox checked={unstakeTokenId === item.id} />
                <TokenAsset tokenId={"pica"} label={item.id} />
              </Stack>
              <Stack direction="row" gap={1}>
                <Typography variant="body2">
                  {item.value.toFixed()} (
                  {formatNumberWithSymbol(item.usdValue, "$")})
                </Typography>
              </Stack>
            </Box>
          </Button>
        </>
      ))}
      <AlertBox status="warning" icon={<WarningAmberRounded color="warning" />}>
        <Typography variant="body2">Slash warning</Typography>
        <Typography variant="inputLabel" color="text.secondary">
          If you withdraw now you will get rekt with less PICA.
        </Typography>
      </AlertBox>
      <Box
        gap={2}
        sx={{
          display: "flex",
          direction: {
            sm: "column",
            md: "row"
          }
        }}
      >
        <Button
          variant="contained"
          color="primary"
          fullWidth
          onClick={openRenewModal}
        >
          Renew
        </Button>
        <Button
          variant="outlined"
          color="primary"
          fullWidth
          onClick={openBurnModal}
        >
          Burn and unstake
        </Button>
      </Box>
    </Stack>
  );
};

import { Modal, TokenAsset } from "@/components";
import { Box, Button, Paper, Stack, Typography } from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { FC } from "react";
import { useStore } from "@/stores/root";
import { formatNumber } from "shared";
import { burnUnstake } from "@/stores/defi/staking";

export const BurnModal: FC<{ open: boolean; onClose: () => void }> = ({
  open,
  onClose
}) => {
  const { withdrawablePica, initialPicaDeposit } = useStore(
    ({ staking }) => staking
  );

  const handleBurnUnstake = () => {
    onClose();
    burnUnstake();
  };

  return (
    <Modal open={open} dismissible onClose={onClose} maxWidth="md">
      <Stack gap={4}>
        <Typography variant="h5" textAlign="center" marginBottom={4}>
          Burn and unstake your position
        </Typography>
        <Box
          sx={{
            flexDirection: {
              sm: "column",
              md: "row"
            }
          }}
          display="flex"
          width="100%"
          alignItems="flex-start"
          justifyContent="space-between"
          gap={4}
        >
          <Stack gap={1.5} width="100%">
            <TextWithTooltip
              TypographyProps={{
                variant: "inputLabel"
              }}
              tooltip="Withdrawable PICA"
            >
              Withdrawable PICA
            </TextWithTooltip>
            <Paper sx={{ position: "relative" }}>
              <Box
                sx={{
                  position: "absolute",
                  left: "1rem",
                  top: "50%",
                  transform: "translateY(-50%)"
                }}
              >
                <TokenAsset tokenId={"pica"} iconOnly />
              </Box>
              <Typography variant="body2" textAlign="center">
                {formatNumber(withdrawablePica)}
              </Typography>
            </Paper>
          </Stack>
          <Stack gap={1.5} width="100%">
            <TextWithTooltip
              TypographyProps={{
                variant: "inputLabel"
              }}
              tooltip="Withdrawable PICA"
            >
              Initial PICA deposit
            </TextWithTooltip>
            <Paper sx={{ position: "relative" }}>
              <Box
                sx={{
                  position: "absolute",
                  left: "1rem",
                  top: "50%",
                  transform: "translateY(-50%)"
                }}
              >
                <TokenAsset tokenId={"pica"} iconOnly />
              </Box>
              <Typography
                variant="body2"
                color="text.secondary"
                textAlign="center"
              >
                {formatNumber(initialPicaDeposit)}
              </Typography>
            </Paper>
          </Stack>
        </Box>
        <Button
          variant="contained"
          color="primary"
          fullWidth
          onClick={handleBurnUnstake}
        >
          Burn and unstake
        </Button>
      </Stack>
    </Modal>
  );
};

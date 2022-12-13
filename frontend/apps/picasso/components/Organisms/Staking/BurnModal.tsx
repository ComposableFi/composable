import { Modal, TokenAsset } from "@/components";
import { Box, Button, Paper, Stack, Typography } from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { FC, useMemo } from "react";
import { callbackGate, formatNumber, subscanExtrinsicLink } from "shared";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { getSigner, useExecutor } from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
import { SnackbarKey, useSnackbar } from "notistack";
import { useStakingRewards } from "@/defi/polkadot/hooks/useStakingRewards";
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";

export const BurnModal: FC<{
  open: boolean;
  onClose: () => void;
  selectedToken: [string, string];
}> = ({ open, onClose, selectedToken }) => {
  const { parachainApi } = usePicassoProvider();
  const account = useSelectedAccount();
  const executor = useExecutor();
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const { stakingPortfolio, refresh } = useStakingRewards();
  const [fnftCollectionId, fnftInstanceId] = selectedToken;

  const handleBurnUnstake = () => {
    let snackbarKey: SnackbarKey | undefined;
    callbackGate(
      async (api, acc, exec) => {
        const signer = await getSigner(APP_NAME, acc.address);
        await exec.execute(
          api.tx.stakingRewards.unstake(...selectedToken),
          acc.address,
          api,
          signer,
          (txHash: string) => {
            snackbarKey = enqueueSnackbar("Processing transaction", {
              variant: "info",
              isClosable: true,
              persist: true,
              url: subscanExtrinsicLink("picasso", txHash),
            });
          },
          (txHash: string) => {
            closeSnackbar(snackbarKey);
            enqueueSnackbar(`Successfully claimed`, {
              variant: "success",
              isClosable: true,
              persist: true,
              url: subscanExtrinsicLink("picasso", txHash),
            });
            refresh();
            onClose();
          },
          (errorMessage: string) => {
            closeSnackbar(snackbarKey);
            enqueueSnackbar(
              "An error occurred while processing the transaction. The process was canceled.",
              {
                variant: "error",
                isClosable: true,
                persist: true,
                description: errorMessage,
              }
            );
            onClose();
          }
        );
      },
      parachainApi,
      account,
      executor
    );
  };

  const currentPortfolio = Object.values(stakingPortfolio).find(
    (portfolio) =>
      portfolio.collectionId === fnftCollectionId &&
      portfolio.instanceId === fnftInstanceId
  );
  const { isExpired } = useExpiredPortfolio(currentPortfolio);

  const withdrawablePica = useMemo(() => {
    if (!currentPortfolio) return 0;

    if (!isExpired) {
      return currentPortfolio.stake.minus(
        currentPortfolio.unlockPenalty
          .dividedBy(100)
          .multipliedBy(currentPortfolio.stake)
      );
    }
    return currentPortfolio.stake;
  }, [currentPortfolio, isExpired]);

  if (selectedToken.join("").length === 0 || !currentPortfolio) {
    return null;
  }

  const initialPICA = currentPortfolio.stake;

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
              md: "row",
            },
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
                variant: "inputLabel",
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
                  transform: "translateY(-50%)",
                }}
              >
                <TokenAsset tokenId={"pica"} iconOnly />
              </Box>
              <Typography variant="body2" textAlign="center">
                {withdrawablePica.toFixed()}
              </Typography>
            </Paper>
          </Stack>
          <Stack gap={1.5} width="100%">
            <TextWithTooltip
              TypographyProps={{
                variant: "inputLabel",
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
                  transform: "translateY(-50%)",
                }}
              >
                <TokenAsset tokenId={"pica"} iconOnly />
              </Box>
              <Typography
                variant="body2"
                color="text.secondary"
                textAlign="center"
              >
                {formatNumber(initialPICA)}
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

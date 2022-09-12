import { Modal, TokenAsset } from "@/components";
import { Box, Button, Paper, Stack, Typography } from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { FC, useEffect, useState } from "react";
import { callbackGate, formatNumber } from "shared";
import BigNumber from "bignumber.js";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { getSigner, useExecutor } from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
import { EventRecord } from "@polkadot/types/interfaces/system";
import { SnackbarKey, useSnackbar } from "notistack";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { fetchStakingRewardPosition } from "@/defi/polkadot/pallets/StakingRewards";

// TODO: positionId should be fetched from subsquid or other sources
const positionId = new BigNumber(4);

export const BurnModal: FC<{ open: boolean; onClose: () => void }> = ({
  open,
  onClose,
}) => {
  const { parachainApi } = usePicassoProvider();
  const [position, setPosition] = useState({
    unlockPenalty: new BigNumber(0),
    share: new BigNumber(0),
    stake: new BigNumber(0),
  });
  const withdrawablePica = position.stake.minus(
    position.stake.multipliedBy(position.unlockPenalty)
  );
  const account = useSelectedAccount();
  const executor = useExecutor();
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();

  const handleBurnUnstake = () => {
    let snackbarKey: SnackbarKey | undefined;
    callbackGate(
      async (api, acc, exec) => {
        const signer = await getSigner(APP_NAME, acc.address);
        await exec.execute(
          api.tx.stakingRewards.unstake(positionId.toString()),
          acc.address,
          api,
          signer,
          (txHash: string) => {
            snackbarKey = enqueueSnackbar("Processing transaction", {
              variant: "info",
              isClosable: true,
              persist: true,
              url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
            });
          },
          (txHash: string, events: EventRecord[]) => {
            closeSnackbar(snackbarKey);
            enqueueSnackbar(`Successfully claimed`, {
              variant: "success",
              isClosable: true,
              persist: true,
              url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
            });
            onClose();
          },
          (errorMessage: string) => {
            closeSnackbar(snackbarKey);
            enqueueSnackbar("An error occurred while processing transaction", {
              variant: "error",
              isClosable: true,
              persist: true,
              description: errorMessage,
            });
            onClose();
          }
        );
      },
      parachainApi,
      account,
      executor
    );
  };

  useEffect(() => {
    callbackGate(
      (api) => fetchStakingRewardPosition(api, positionId, setPosition),
      parachainApi
    );
  }, [parachainApi]);

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
                {formatNumber(withdrawablePica)}
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
                {formatNumber(position.stake)}
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

import { Modal, TokenAsset } from "@/components";
import { Box, Button, Paper, Slider, Stack, Typography } from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { FC } from "react";
import { callbackGate, formatNumber, subscanExtrinsicLink } from "shared";
import { usePicassoAccount } from "@/defi/polkadot/hooks";
import {
  useExecutor,
  usePendingExtrinsic,
  usePicassoProvider,
  useSigner,
} from "substrate-react";
import { SnackbarKey, useSnackbar } from "notistack";
import { useStakingRewards } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewards";
import { useStore } from "@/stores/root";
import { getFnftKey } from "@/defi/polkadot/pallets/StakingRewards";
import { getPicassoTokenById } from "@/stores/defi/polkadot/tokens/utils";

export const SplitModal: FC<{
  open: boolean;
  onClose: () => void;
  selectedToken: [string, string];
}> = ({ open, onClose, selectedToken }) => {
  const { parachainApi } = usePicassoProvider();
  const account = usePicassoAccount();
  const executor = useExecutor();
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const { stakingPortfolio } = useStakingRewards();
  const [fnftCollectionId, fnftInstanceId] = selectedToken;
  const signer = useSigner();
  const isPendingSplit = usePendingExtrinsic(
    "split",
    "stakingRewards",
    account?.address ?? "-"
  );
  const ratio = useStore((store) => store.ui.stakingRewards.ratio);
  const setRatio = useStore((store) => store.ui.setRatio);
  const currentPortfolio = stakingPortfolio.get(
    getFnftKey(fnftCollectionId, fnftInstanceId)
  );

  if (selectedToken.join("").length === 0 || !currentPortfolio) {
    return null;
  }

  const handleClick = () => {
    let snackbarKey: SnackbarKey | undefined;
    callbackGate(
      async (api, acc, exec, _signer) => {
        await exec.execute(
          api.tx.stakingRewards.split(...selectedToken, ratio * 10000),
          acc.address,
          api,
          _signer,
          (txHash: string) => {
            snackbarKey = enqueueSnackbar(
              `Splitting xPICA ${selectedToken[1]}...`,
              {
                variant: "info",
                isClosable: true,
                persist: true,
                url: subscanExtrinsicLink("picasso", txHash),
              }
            );
          },
          (txHash: string) => {
            closeSnackbar(snackbarKey);
            useStore.setState((state) => {
              const fnftKey = getFnftKey(...selectedToken);
              state.claimableRewards[fnftKey] = [];
              state.stakingPortfolio.delete(fnftKey);
              state.stakingPositions.delete(fnftKey);
              state.ui.setAgreedSlash(false);
            });
            enqueueSnackbar(`Successfully split fnft.`, {
              variant: "success",
              isClosable: true,
              persist: true,
              url: subscanExtrinsicLink("picasso", txHash),
            });
            useStore.getState().ui.setStakingTab(0);
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
      executor,
      signer
    );
  };
  const initialPICA = currentPortfolio.stake;
  const shareAsset = getPicassoTokenById(currentPortfolio.shareAssetId);

  return (
    <Modal open={open} dismissible onClose={onClose} maxWidth="md">
      <Stack gap={4}>
        <Typography variant="h5" textAlign="center" marginBottom={4}>
          Split your current position
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
              tooltip="The initial staked PICA amount"
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
        <Slider
          min={1}
          max={99}
          value={ratio}
          onChange={(_, v) => {
            if (typeof v === "number") setRatio(v);
          }}
          valueLabelFormat={(value) => {
            return `${value}%`;
          }}
          valueLabelDisplay="auto"
        />
        <Stack direction="row" gap={2} width="100%">
          <Paper
            sx={{
              width: "100%",
            }}
          >
            <Stack
              direction="row"
              justifyContent="space-between"
              alignItems="center"
            >
              <Typography variant="body2">
                {shareAsset && (
                  <TokenAsset
                    tokenId={shareAsset.id}
                    label={`${shareAsset.symbol} ${currentPortfolio.instanceId}`}
                  />
                )}
              </Typography>
              <Typography variant="inputLabel">
                {initialPICA.multipliedBy(ratio).div(100).toFormat()} PICA
              </Typography>
            </Stack>
          </Paper>
          <Paper
            sx={{
              width: "100%",
            }}
          >
            <Stack
              direction="row"
              justifyContent="space-between"
              alignItems="center"
            >
              <Typography variant="body2">
                {shareAsset && (
                  <TokenAsset
                    tokenId={shareAsset.id}
                    label={`${shareAsset.symbol} ~`}
                  />
                )}
              </Typography>
              <Typography variant="inputLabel">
                {initialPICA
                  .multipliedBy(100 - ratio)
                  .div(100)
                  .toFormat()}{" "}
                PICA
              </Typography>
            </Stack>
          </Paper>
        </Stack>
        <Button
          variant="contained"
          color="primary"
          fullWidth
          onClick={handleClick}
          disabled={isPendingSplit}
        >
          Split position
        </Button>
      </Stack>
    </Modal>
  );
};

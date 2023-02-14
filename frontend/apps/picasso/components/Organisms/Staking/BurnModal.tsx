import { Modal, TokenAsset } from "@/components";
import {
  Box,
  Button,
  Checkbox,
  CircularProgress,
  FormControlLabel,
  FormGroup,
  Paper,
  Slider,
  Stack,
  Typography,
} from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import React, { FC, useMemo } from "react";
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
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";
import { useStore } from "@/stores/root";
import { getFnftKey } from "@/defi/polkadot/pallets/StakingRewards";
import { getPicassoTokenById } from "@/stores/defi/polkadot/tokens/utils";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";
import BigNumber from "bignumber.js";

export const BurnModal: FC<{
  open: boolean;
  onClose: () => void;
  selectedToken: [string, string];
}> = ({ open, onClose, selectedToken }) => {
  const { stakingPortfolio } = useStakingRewards();
  const shouldUnstakeAll = useStore(
    (store) => store.ui.stakingRewards.shouldUnstakeAll
  );
  const [fnftCollectionId, fnftInstanceId] = selectedToken;

  const currentPortfolio = stakingPortfolio.get(
    getFnftKey(fnftCollectionId, fnftInstanceId)
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
  const shareAsset = getPicassoTokenById(currentPortfolio.shareAssetId);
  const initialPICA = currentPortfolio.stake;

  return (
    <Modal open={open} dismissible onClose={onClose} maxWidth="md">
      <Stack gap={4}>
        <Typography variant="h5" textAlign="center" marginBottom={4}>
          Burn and unstake your position
        </Typography>
        {shouldUnstakeAll && (
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
                tooltip="Staked PICA that is not locked up and can be unstaked"
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
        )}
        {shareAsset && (
          <PartialUnstakeSection
            shareAsset={shareAsset}
            currentPortfolio={currentPortfolio}
            initialPICA={initialPICA}
          />
        )}
        {shareAsset && (
          <UnstakeButtonSection
            shareAsset={shareAsset}
            onClose={onClose}
            selectedToken={selectedToken}
          />
        )}
      </Stack>
    </Modal>
  );
};

const PartialUnstakeSection = ({
  shareAsset,
  currentPortfolio,
  initialPICA,
}: {
  shareAsset: TokenMetadata;
  currentPortfolio: PortfolioItem;
  initialPICA: BigNumber;
}) => {
  const shouldUnstakeAll = useStore(
    (store) => store.ui.stakingRewards.shouldUnstakeAll
  );
  const setUnstakeAll = useStore((store) => store.ui.setUnstakeAll);
  const handleChange = (_: any, v: boolean) => setUnstakeAll(v);
  const ratio = useStore((store) => store.ui.stakingRewards.ratio);
  const setRatio = useStore((store) => store.ui.setRatio);
  const { isExpired } = useExpiredPortfolio(currentPortfolio);
  const partialWithdrawalAmount = useMemo(() => {
    if (!currentPortfolio) return new BigNumber(0);
    const postRatioAmount = initialPICA.div(100).multipliedBy(ratio);
    const shouldHavePenalty = !isExpired && !currentPortfolio.multiplier.eq(1);
    return shouldHavePenalty
      ? postRatioAmount.minus(
          currentPortfolio.unlockPenalty
            .dividedBy(100)
            .multipliedBy(postRatioAmount)
        )
      : postRatioAmount;
  }, [currentPortfolio, initialPICA, isExpired, ratio]);

  return (
    <Stack gap={4}>
      {!shouldUnstakeAll && (
        <>
          <Stack gap={2}>
            <TextWithTooltip
              TypographyProps={{
                variant: "inputLabel",
              }}
              tooltip="A portion of the fnft will be unstaked"
            >
              Withdrawal:
            </TextWithTooltip>
            <Slider
              disabled={shouldUnstakeAll}
              min={1}
              max={99}
              value={ratio}
              onChange={(_, v) => {
                if (typeof v === "number") setRatio(v);
              }}
              valueLabelFormat={(value) => {
                return `${value}%`;
              }}
              valueLabelDisplay="on"
            />
          </Stack>
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
                  <TokenAsset
                    tokenId={shareAsset.id}
                    label={`${shareAsset.symbol} ${currentPortfolio.instanceId}`}
                  />
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
                <Typography variant="body2">Unstake amount:</Typography>
                <Typography variant="inputLabel">
                  {`${partialWithdrawalAmount.toFormat()} PICA`}
                </Typography>
              </Stack>
            </Paper>
          </Stack>
        </>
      )}
      <FormGroup>
        <FormControlLabel
          control={
            <Checkbox
              checked={shouldUnstakeAll}
              value={shouldUnstakeAll}
              onChange={handleChange}
            />
          }
          label="Unstake all"
        />
      </FormGroup>
    </Stack>
  );
};

const UnstakeButtonSection = ({
  shareAsset,
  selectedToken,
  onClose,
}: {
  shareAsset: TokenMetadata;
  onClose: () => void;
  selectedToken: [string, string];
}) => {
  const { parachainApi } = usePicassoProvider();
  const account = usePicassoAccount();
  const executor = useExecutor();
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const signer = useSigner();
  const isPendingBurn = usePendingExtrinsic(
    "unstake",
    "stakingRewards",
    account?.address ?? "-"
  );
  const isPendingPartialUnstake = usePendingExtrinsic(
    "batch",
    "utility",
    account?.address ?? "-"
  );
  const shouldUnstakeAll = useStore(
    (store) => store.ui.stakingRewards.shouldUnstakeAll
  );
  const ratio = useStore((store) => store.ui.stakingRewards.ratio);
  const buttonLabel = shouldUnstakeAll
    ? "Burn and unstake"
    : "Partially unstake";

  const handleFormSubmit = () => {
    let snackbarKey: SnackbarKey | undefined;
    callbackGate(
      async (api, acc, exec, _signer) => {
        const eventHandlers = {
          onReady: (message: string) => (txHash: string) => {
            snackbarKey = enqueueSnackbar(message, {
              variant: "info",
              isClosable: true,
              persist: true,
              url: subscanExtrinsicLink("picasso", txHash),
            });
          },
          onFinalize: (message: string) => (txHash: string) => {
            closeSnackbar(snackbarKey);
            useStore.setState((state) => {
              const fnftKey = getFnftKey(...selectedToken);
              state.claimableRewards[fnftKey] = [];
              state.stakingPortfolio.delete(fnftKey);
              state.stakingPositions.delete(fnftKey);
              state.ui.setAgreedSlash(false);
            });
            enqueueSnackbar(message, {
              variant: "success",
              isClosable: true,
              persist: true,
              url: subscanExtrinsicLink("picasso", txHash),
            });
            useStore.getState().ui.setStakingTab(0);
            onClose();
          },
          onError: (errorMessage: string) => {
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
          },
        };

        if (shouldUnstakeAll) {
          await exec.execute(
            api.tx.stakingRewards.unstake(...selectedToken),
            acc.address,
            api,
            _signer,
            eventHandlers.onReady(
              `Unstaking ${shareAsset.symbol}  ${selectedToken[1]}`
            ),
            eventHandlers.onFinalize(`Successfully unstaked`),
            eventHandlers.onError
          );
        } else {
          // Apply partial unstaking
          await exec.execute(
            api.tx.utility.batch([
              api.tx.stakingRewards.split(...selectedToken, ratio * 10000),
              api.tx.stakingRewards.unstake(...selectedToken),
            ]),
            acc.address,
            api,
            _signer,
            eventHandlers.onReady(
              `Unstaking ${ratio}% of ${shareAsset.symbol} ${selectedToken[1]}`
            ),
            eventHandlers.onFinalize(`Successfully unstaked.`),
            eventHandlers.onError
          );
        }
      },
      parachainApi,
      account,
      executor,
      signer
    );
  };
  return (
    <Button
      variant="contained"
      color="primary"
      fullWidth
      onClick={handleFormSubmit}
      disabled={isPendingBurn || isPendingPartialUnstake}
    >
      {isPendingBurn || isPendingPartialUnstake ? (
        <CircularProgress variant="indeterminate" size={24} />
      ) : (
        buttonLabel
      )}
    </Button>
  );
};

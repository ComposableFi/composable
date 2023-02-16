import { Modal, TokenAsset } from "@/components";
import {
  alpha,
  Box,
  Button,
  CircularProgress,
  Paper,
  Slider,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import React, { FC, useMemo } from "react";
import { callbackGate, subscanExtrinsicLink } from "shared";
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
import { TokenWithUSD } from "@/components/Organisms/Staking/TokenWithUSD";
import { StakeRemainingRelativeDate } from "@/components/Organisms/Staking/StakeRemainingRelativeDate";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";

export const BurnModal: FC<{
  open: boolean;
  onClose: () => void;
  selectedToken: [string, string];
}> = ({ open, onClose, selectedToken }) => {
  const { stakingPortfolio, pica } = useStakingRewards();
  const [fnftCollectionId, fnftInstanceId] = selectedToken;

  const currentPortfolio = stakingPortfolio.get(
    getFnftKey(fnftCollectionId, fnftInstanceId)
  );

  if (selectedToken.join("").length === 0 || !currentPortfolio) {
    return null;
  }
  const shareAsset = getPicassoTokenById(currentPortfolio.shareAssetId);
  const initialPICA = currentPortfolio.stake;

  if (!shareAsset) return null;

  return (
    <Modal open={open} dismissible onClose={onClose} maxWidth="md">
      <Stack gap={4}>
        <Typography variant="h5" textAlign="center" marginBottom={4}>
          Burn and unstake your position
        </Typography>
        <CurrentPosition
          xToken={shareAsset}
          xTokenLabel={`${shareAsset.symbol} ${currentPortfolio.instanceId}`}
          picaToken={pica}
          portfolio={currentPortfolio}
          stakedPrice={initialPICA}
        />
        {shareAsset && (
          <PartialUnstakeSection
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
  currentPortfolio,
  initialPICA,
}: {
  currentPortfolio: PortfolioItem;
  initialPICA: BigNumber;
}) => {
  const { pica } = useStakingRewards();
  const picaPrice = usePicaPriceDiscovery();
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
  const withdrawalPrice = partialWithdrawalAmount.multipliedBy(picaPrice);

  return (
    <Stack gap={4}>
      <Stack gap={2}>
        <Typography variant="inputLabel">
          Select how much you would like to withdraw
        </Typography>
        <Typography variant="h6">{ratio}%</Typography>
        <Slider
          min={0}
          max={100}
          value={ratio}
          onChange={(_, v) => {
            if (typeof v === "number") setRatio(v);
          }}
          valueLabelFormat={(value) => {
            return `${value}%`;
          }}
          valueLabelDisplay="off"
        />
      </Stack>
      <Stack direction="row" gap={2} width="100%">
        <Stack gap={1.5} width="100%">
          <Typography variant="inputLabel">Amount you are unstaking</Typography>
          <Paper
            sx={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
            }}
          >
            <TokenAsset tokenId={"pica"} iconOnly sx={{ width: "inherit" }} />
            <TokenWithUSD
              symbol={pica.symbol}
              amount={partialWithdrawalAmount.toFormat(4)}
              price={withdrawalPrice.toFormat(2)}
            />
          </Paper>
        </Stack>
      </Stack>
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
  const ratio = useStore((store) => store.ui.stakingRewards.ratio);
  const shouldUnstakeAll = ratio === 100;
  const buttonLabel = shouldUnstakeAll ? "Unstake all" : "Partially unstake";

  const handleFormSubmit = () => {
    let snackbarKey: SnackbarKey | undefined;
    callbackGate(
      async (api, acc, exec, _signer) => {
        const eventHandlers = {
          onReady:
            (message: string, description?: string) => (txHash: string) => {
              snackbarKey = enqueueSnackbar(message, {
                description,
                variant: "info",
                isClosable: true,
                persist: true,
                url: subscanExtrinsicLink("picasso", txHash),
              });
            },
          onFinalize:
            (message: string, description?: string) => (txHash: string) => {
              closeSnackbar(snackbarKey);
              useStore.setState((state) => {
                const fnftKey = getFnftKey(...selectedToken);
                state.claimableRewards[fnftKey] = [];
                state.stakingPortfolio.delete(fnftKey);
                state.stakingPositions.delete(fnftKey);
                state.ui.setAgreedSlash(false);
              });
              enqueueSnackbar(message, {
                description,
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

        try {
          if (shouldUnstakeAll) {
            await exec.execute(
              api.tx.stakingRewards.unstake(...selectedToken),
              acc.address,
              api,
              _signer,
              eventHandlers.onReady(
                "Processing unstake",
                `Unstaking ${shareAsset.symbol}  ${selectedToken[1]}`
              ),
              eventHandlers.onFinalize(
                `Unstake complete`,
                `Successfully unstaked ${shareAsset.symbol}  ${selectedToken[1]}`
              ),
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
                "Processing partial unstake",
                `Unstaking ${ratio}% of ${shareAsset.symbol} ${selectedToken[1]}`
              ),
              eventHandlers.onFinalize(
                `Successfully unstaked.`,
                `Unstaked ${ratio}% of ${shareAsset.symbol} ${selectedToken[1]}`
              ),
              eventHandlers.onError
            );
          }
        } catch (e) {
          if (e instanceof Error) {
            eventHandlers.onError(e.message);
          }
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
      disabled={isPendingBurn || isPendingPartialUnstake || ratio === 0}
    >
      {isPendingBurn || isPendingPartialUnstake ? (
        <CircularProgress variant="indeterminate" size={24} />
      ) : (
        buttonLabel
      )}
    </Button>
  );
};

const CurrentPosition = ({
  xToken,
  xTokenLabel,
  picaToken,
  portfolio,
  stakedPrice,
}: {
  xToken: TokenMetadata;
  xTokenLabel: string;
  picaToken: TokenMetadata;
  portfolio: PortfolioItem;
  stakedPrice: BigNumber;
}) => {
  const theme = useTheme();
  return (
    <Stack gap={1.5}>
      <Typography variant="inputLabel">Current position</Typography>
      <Box
        sx={{
          padding: theme.spacing(1.5, 2),
          height: "auto",
          borderRadius: `${theme.shape.borderRadius}px`,
          border: `2px solid ${alpha(theme.palette.common.white, 0.3)}`,
        }}
      >
        <Stack
          width="100%"
          justifyContent="space-between"
          alignItems="center"
          direction="row"
        >
          <Stack direction="row" gap={1}>
            <TokenAsset tokenId={xToken.id} label={xTokenLabel} />
          </Stack>
          <Stack direction="row" gap={2} alignItems="center">
            <TokenWithUSD
              symbol={picaToken.symbol}
              amount={portfolio.stake.toFormat()}
              price={stakedPrice.toFormat(2)}
            />
            <StakeRemainingRelativeDate portfolio={portfolio} />
          </Stack>
        </Stack>
      </Box>
    </Stack>
  );
};

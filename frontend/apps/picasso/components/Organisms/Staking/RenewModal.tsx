import { BigNumberInput, Modal, TokenAsset } from "@/components";
import { FC, useState } from "react";
import { Box, Button, Paper, Stack, Typography } from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import {
  callbackGate,
  formatNumber,
  subscanExtrinsicLink,
  toChainIdUnit,
} from "shared";
import { DurationOption } from "@/defi/polkadot/pallets/StakingRewards";
import { useStakingRewards } from "@/defi/polkadot/hooks/useStakingRewards";
import BigNumber from "bignumber.js";
import { useStore } from "@/stores/root";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { getSigner, useExecutor } from "substrate-react";
import { SnackbarKey, useSnackbar } from "notistack";
import { APP_NAME } from "@/defi/polkadot/constants";

export const RenewModal: FC<{
  open: boolean;
  onClose: () => void;
  selectedToken: [string, string];
}> = ({ open, onClose, selectedToken }) => {
  const [extendPeriod, setExtendPeriod] = useState<DurationOption | undefined>(
    undefined
  );

  const pica = useStore(({ substrateTokens }) => substrateTokens.tokens.pica);
  const native = useStore(
    ({ substrateBalances }) => substrateBalances.balances.picasso.pica.balance
  );
  const [extendAmount, setExtendAmount] = useState<BigNumber>(new BigNumber(0));
  const { parachainApi, stakingPortfolio, refresh } = useStakingRewards();
  const [isValid, setValid] = useState(true);
  const [fnftCollectionId, fnftInstanceId] = selectedToken;
  const account = useSelectedAccount();
  const executor = useExecutor();
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const currentPortfolio = stakingPortfolio.find(
    (portfolio) =>
      portfolio.collectionId === fnftCollectionId &&
      portfolio.instanceId === fnftInstanceId
  );

  if (!currentPortfolio) {
    return null;
  }
  const handleRenew = () => {
    if (!isValid) {
      return;
    }

    let snackbarKey: SnackbarKey | undefined;
    callbackGate(
      async (api, acc, executor) => {
        const signer = await getSigner(APP_NAME, acc.address);
        await executor.execute(
          (api.tx.stakingRewards.extend as any)(
            selectedToken[0],
            selectedToken[1],
            api.createType("u128", toChainIdUnit(extendAmount).toString())
          ),
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
            enqueueSnackbar(`Successfully staked`, {
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
    refresh();
    onClose();
  };

  const initialPicaDeposit = currentPortfolio.stake;

  return (
    <Modal open={open} dismissible onClose={onClose} maxWidth="md">
      <Stack gap={4}>
        <Typography variant="h5" textAlign="center" marginBottom={4}>
          Renew staking period
        </Typography>
        <Stack gap={1.5}>
          <TextWithTooltip
            TypographyProps={{
              variant: "inputLabel",
            }}
            tooltip="Initial PICA deposit "
          >
            Initial PICA deposit
          </TextWithTooltip>
          <Paper
            sx={{
              position: "relative",
            }}
          >
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
              textAlign="center"
              variant="body2"
              color="text.secondary"
            >
              {formatNumber(initialPicaDeposit)}
            </Typography>
          </Paper>
        </Stack>
        <Box>
          <TextWithTooltip
            tooltip={"This amount will be added to your current stake"}
          >
            Enter Amount
          </TextWithTooltip>
          <BigNumberInput
            buttonLabel={"Max"}
            ButtonProps={{
              onClick: () => {
                setExtendAmount(native);
              },
            }}
            isValid={setValid}
            setter={setExtendAmount}
            maxValue={native}
            value={extendAmount}
            tokenId={pica.id}
            maxDecimals={12}
          />
        </Box>
        <Button
          disabled={extendAmount.eq(0) || !isValid}
          variant="contained"
          color="primary"
          fullWidth
          onClick={handleRenew}
        >
          Add stake to period
        </Button>
      </Stack>
    </Modal>
  );
};

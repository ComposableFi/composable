import React, { useState } from "react";
import { Modal, ModalProps } from "@/components/Molecules";
import { BaseAsset, CircularProgress, Label } from "@/components/Atoms";
import {
  alpha,
  Box,
  Button,
  Divider,
  IconButton,
  Typography,
  useTheme,
} from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";
import BigNumber from "bignumber.js";
import { useRemoveLiquidityState } from "@/store/removeLiquidity/hooks";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { useParachainApi, useSelectedAccount, useExecutor, getSigner, useSigner } from "substrate-react";
import { useRouter } from "next/router";
import { toChainUnits } from "@/defi/utils";
import { setUiState } from "@/store/ui/ui.slice";
import { Asset } from "shared";

export type ConfirmingModalProps = {
  baseAsset: Asset;
  quoteAsset: Asset;
  price1: BigNumber;
  price2: BigNumber;
  amount1: BigNumber;
  amount2: BigNumber;
  lpBalance: BigNumber;
  percentage: BigNumber;
  setConfirmed?: (confirmed: boolean) => any;
} & ModalProps;

export const ConfirmingModal: React.FC<ConfirmingModalProps> = ({
  baseAsset,
  quoteAsset,
  price1,
  price2,
  amount1,
  amount2,
  lpBalance,
  percentage,
  setConfirmed,
  ...rest
}) => {
  // WIP
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const signer = useSigner();
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const router = useRouter();
  const executor = useExecutor();
  const { poolId } = useRemoveLiquidityState();

  const theme = useTheme();

  const [confirming, setConfirming] = useState<boolean>(false);

  const onCloseHandler = () => {
    setUiState({ isConfirmingModalOpen: false });
  };

  const confirmRemoveHandler = async () => {
    // WIP
    if (
      parachainApi &&
      signer !== undefined && executor &&
      baseAsset &&
      quoteAsset &&
      selectedAccount
    ) {
      try {
        const lpRemoveAmount = toChainUnits(lpBalance).times(percentage);
        executor.execute(
          parachainApi.tx.pablo.removeLiquidity(
            parachainApi.createType("u128", poolId), // Pool ID
            parachainApi.createType("u128", lpRemoveAmount.dp(0).toString()), // LP Receive
            parachainApi.createType("u128", 0), // Min Base
            parachainApi.createType("u128", 0) // Min Quote
          ),
          selectedAccount.address,
          parachainApi,
          signer,
          (_txHash: string) => {
            setConfirming(true);
          },
          (txHash: string, events) => {
            console.log("Finalized ", txHash);
            setUiState({ isConfirmingModalOpen: false });
            setConfirming(false);
            router.push("/pool/select/" + poolId);
          },
          (txError) => {
            console.log("Error ", txError);
            setUiState({ isConfirmingModalOpen: false });
            setConfirming(false);
          }
        );
      } catch (err) {
        console.log(err);
        setUiState({ isConfirmingModalOpen: false });
        setConfirming(false);
      }
    }
  };

  return (
    <Modal onClose={onCloseHandler} {...rest}>
      {!confirming && (
        <Box
          sx={{
            background: theme.palette.gradient.secondary,
            width: 550,
            [theme.breakpoints.down("sm")]: {
              width: "100%",
            },
            borderRadius: 1,
            padding: theme.spacing(4),
            boxShadow: `-1px -1px ${alpha(
              theme.palette.common.white,
              theme.custom.opacity.light
            )}`,
          }}
        >
          <Box
            display="flex"
            alignItems="center"
            justifyContent="space-between"
          >
            <Typography variant="body1">You will receive</Typography>
            <IconButton onClick={onCloseHandler}>
              <CloseIcon />
            </IconButton>
          </Box>

          <Label
            mt={4}
            label={`${amount1}`}
            TypographyProps={{
              variant: "h6",
            }}
            BalanceProps={{
              title: <BaseAsset icon={baseAsset.getIconUrl()} pr={1} />,
              balance: `${baseAsset.getSymbol()}`,
              BalanceTypographyProps: {
                variant: "body1",
              },
            }}
          />

          <Typography variant="h6" mt={2}>
            +
          </Typography>

          <Label
            mt={2}
            label={`${amount2}`}
            TypographyProps={{
              variant: "h6",
            }}
            BalanceProps={{
              title: <BaseAsset icon={quoteAsset.getIconUrl()} pr={1} />,
              balance: `${quoteAsset.getSymbol()}`,
              BalanceTypographyProps: {
                variant: "body1",
              },
            }}
          />

          <Typography variant="body2" mt={4} textAlign="center" paddingX={4.25}>
            Output is estimated. If the price changes by more than 5% your
            transaction will revert.
          </Typography>

          <Box mt={4}>
            <Divider
              sx={{
                borderColor: alpha(
                  theme.palette.common.white,
                  theme.custom.opacity.main
                ),
              }}
            />
          </Box>

          <Label
            mt={4}
            label={`Price`}
            BalanceProps={{
              balance: `1 ${quoteAsset.getSymbol()} = ${price1} ${baseAsset.getSymbol()}`,
              BalanceTypographyProps: {
                variant: "body1",
              },
            }}
          />

          <Label
            mt={2}
            label=""
            BalanceProps={{
              balance: `1 ${baseAsset.getSymbol()} = ${price2} ${quoteAsset.getSymbol()}`,
              BalanceTypographyProps: {
                variant: "body1",
              },
            }}
          />

          <Box mt={4}>
            <Button
              variant="contained"
              size="large"
              fullWidth
              onClick={confirmRemoveHandler}
            >
              Confirm
            </Button>
          </Box>
        </Box>
      )}

      {confirming && (
        <Box
          textAlign="center"
          sx={{
            width: 550,
            [theme.breakpoints.down("sm")]: {
              width: "100%",
            },
            padding: theme.spacing(3),
          }}
        >
          <Box display="flex" justifyContent="center">
            <CircularProgress size={96} />
          </Box>
          <Typography variant="h5" mt={8}>
            Waiting for confirmation
          </Typography>
          <Typography variant="subtitle1" mt={2} color="text.secondary">
            Removing {`${percentage.times(lpBalance)}`} {baseAsset.getSymbol()}/
            {quoteAsset.getSymbol()}
          </Typography>
          <Typography
            variant="body1"
            mt={2}
            sx={{
              color: alpha(
                theme.palette.common.white,
                theme.custom.opacity.main
              ),
            }}
          >
            Confirm this transaction in your wallet
          </Typography>
        </Box>
      )}
    </Modal>
  );
};

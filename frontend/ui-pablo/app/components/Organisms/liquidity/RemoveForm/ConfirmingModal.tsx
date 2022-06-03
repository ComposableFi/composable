import React, { useState } from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label, BaseAsset } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { TokenId } from "@/defi/types";
import {
  alpha,
  Box,
  IconButton,
  Typography,
  useTheme,
  Button,
  Divider,
} from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";

import { useDispatch } from "react-redux";
import {
  closeConfirmingModal, setMessage,
} from "@/stores/ui/uiSlice";
import { useAppSelector } from "@/hooks/store";
import BigNumber from "bignumber.js";
import { CircularProgress } from "@/components/Atoms";

export type ConfirmingModalProps = {
  amount1: BigNumber,
  amount2: BigNumber,
  setConfirmed?: (confirmed: boolean) => any,
} & ModalProps;

export const ConfirmingModal: React.FC<ConfirmingModalProps> = ({
  amount1,
  amount2,
  setConfirmed,
  ...rest
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const [confirming, setConfirming] = useState<boolean>(false);

  const {
    tokenId1,
    tokenId2,
    price1,
    price2,
  } = useAppSelector((state) => state.pool.currentLiquidity);

  const token1 = getToken(tokenId1 as TokenId);
  const token2 = getToken(tokenId2 as TokenId);

  const onConfirmHandler = () => {
    setConfirming(true);
    setTimeout(() => {
      dispatch(setMessage(
        {
          text: `Remove ${amount1} ${token1.symbol} and ${amount2} ${token2.symbol}`,
          link: "/",
          severity: "success",
        }
      ));
      setConfirmed && setConfirmed(true);
    }, 2000);
  };

  const onCloseHandler = () => {
    dispatch(closeConfirmingModal())
  };

  return (
    <Modal
      onClose={onCloseHandler}
      {...rest}
    >
      {!confirming && (
        <Box
          sx={{
            background: theme.palette.gradient.secondary,
            width: 550,
            [theme.breakpoints.down('sm')]: {
              width: '100%',
            },
            borderRadius: 1,
            padding: theme.spacing(4),
            boxShadow: `-1px -1px ${alpha(theme.palette.common.white, theme.custom.opacity.light)}`,
          }}
        >
          <Box
            display="flex"
            alignItems="center"
            justifyContent="space-between"
          >
            <Typography variant="body1">
              You will recieve
            </Typography>
            <IconButton
              onClick={onCloseHandler}
            >
              <CloseIcon />
            </IconButton>
          </Box>

          <Label
            mt={4}
            label={`${amount1}`}
            TypographyProps={{
              variant: 'h6'
            }}
            BalanceProps={{
              title: <BaseAsset icon={token1.icon} pr={1} />,
              balance: `${token1.symbol}`,
              BalanceTypographyProps: {
                variant: "body1",
              },
            }}
          />

          <Typography variant="h6" mt={2}>+</Typography>

          <Label
            mt={2}
            label={`${amount2}`}
            TypographyProps={{
              variant: 'h6'
            }}
            BalanceProps={{
              title: <BaseAsset icon={token2.icon} pr={1} />,
              balance: `${token2.symbol}`,
              BalanceTypographyProps: {
                variant: "body1",
              },
            }}
          />

          <Typography variant="body2" mt={4} textAlign="center" paddingX={4.25}>
            Output is estimated. If the price changes by more than 5% your transaction will revert.
          </Typography>

          <Box mt={4}>
            <Divider
              sx={{
                borderColor: alpha(theme.palette.common.white, theme.custom.opacity.main),
              }} />
          </Box>

          <Label
            mt={4}
            label={`Price`}
            BalanceProps={{
              balance: `1 ${token2.symbol} = ${price1} ${token1.symbol}`,
              BalanceTypographyProps: {
                variant: "body1",
              },
            }}
          />

          <Label
            mt={2}
            label=""
            BalanceProps={{
              balance: `1 ${token1.symbol} = ${price2} ${token2.symbol}`,
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
              onClick={onConfirmHandler}
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
            [theme.breakpoints.down('sm')]: {
              width: '100%',
            },
            padding: theme.spacing(3)
          }}
        >
          <Box display="flex" justifyContent="center">
            <CircularProgress size={96} />
          </Box>
          <Typography variant="h5" mt={8}>
            Waiting for confirmation
          </Typography>
          <Typography variant="subtitle1" mt={2} color="text.secondary">
            Removing {`${amount1}`} {token1.symbol} and {`${amount2}`} {token2.symbol}
          </Typography>
          <Typography
            variant="body1"
            mt={2}
            sx={{
              color: alpha(theme.palette.common.white, theme.custom.opacity.main),
            }}
          >
            Confirm this transaction in your wallet
          </Typography>
        </Box>
      )}
    </Modal>
  );
};


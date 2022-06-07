import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label, BaseAsset } from "@/components/Atoms";
import { 
  alpha,
  Box,
  IconButton,
  Typography,
  useTheme,
  Button, 
} from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";

import { useDispatch } from "react-redux";
import {  
  closeConfirmSupplyModal, 
  openPreviewSupplyModal,
} from "@/stores/ui/uiSlice";
import { useAppSelector } from "@/hooks/store";
import BigNumber from "bignumber.js";
import { AssetMetadata } from "@/defi/polkadot/Assets";

export interface SupplyModalProps {
  baseAsset: AssetMetadata | null;
  quoteAsset: AssetMetadata | null;
  baseAmount: BigNumber;
  quoteAmount: BigNumber;
  lpReceiveAmount: BigNumber;
  priceBaseInQuote: BigNumber;
  priceQuoteInBase: BigNumber;
  share: BigNumber;
}

export const ConfirmSupplyModal: React.FC<SupplyModalProps & ModalProps> = ({
  baseAsset,
  quoteAsset,
  baseAmount,
  quoteAmount,
  lpReceiveAmount,
  priceBaseInQuote,
  priceQuoteInBase,
  share,
  ...rest
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const {
    amount,
  } = useAppSelector((state) => state.pool.currentSupply);

  const confirmSupply = () => {
    dispatch(closeConfirmSupplyModal());
    dispatch(openPreviewSupplyModal());
  };

  return (
    <Modal
      onClose={() => dispatch(closeConfirmSupplyModal())}
      {...rest}
    >
      <Box
        sx={{
          background: theme.palette.gradient.secondary,
          width: 550,
          [theme.breakpoints.down('sm')]: {
            width: '100%',
          },
          borderRadius: 1,
          padding: theme.spacing(3),
          boxShadow: `-1px -1px ${alpha(theme.palette.common.white, theme.custom.opacity.light)}`,
        }}
      >
        <Box
          display="flex"
          alignItems="center"
          justifyContent="space-between"
        >
          <Typography variant="body1">
            You will receive
          </Typography>
          <IconButton 
            onClick={() => dispatch(closeConfirmSupplyModal())}
          >
            <CloseIcon />
          </IconButton>
        </Box>

        <Typography variant="h5" mt={1.75}>
          {`${amount}`}
        </Typography>

        <Typography variant="body1" color="text.secondary" mt={1.75}>
          {`LP ${baseAsset?.symbol}/${quoteAsset?.symbol} Tokens`}
        </Typography>

        <Typography variant="body2" mt={4} textAlign="center" paddingX={4.25}>
          Output is estimated. If the price changes by more than 5% your transaction will revert.
        </Typography>

        <Box
          mt={4}
          borderTop={`1px solid ${alpha(theme.palette.common.white, theme.custom.opacity.main)}`}
        />

        <Label
          mt={4}
          label={`Pooled ${baseAsset?.symbol}`}
          BalanceProps={{
            title: <BaseAsset icon={baseAsset?.icon} pr={priceBaseInQuote.toNumber()} />,
            balance: `${baseAmount}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Pooled ${quoteAsset?.symbol}`}
          BalanceProps={{
            title: <BaseAsset icon={quoteAsset?.icon} pr={priceQuoteInBase.toNumber()} />,
            balance: `${quoteAmount}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Price`}
          BalanceProps={{
            balance: `1 ${baseAsset?.symbol} = ${priceBaseInQuote} ${quoteAsset?.symbol}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label=""
          BalanceProps={{
            balance: `1 ${quoteAsset?.symbol} = ${priceQuoteInBase} ${baseAsset?.symbol}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Share of pool`}
          BalanceProps={{
            balance: `${share.toFixed(4)}%`,
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
            onClick={confirmSupply}
          >
            Confirm supply
          </Button>
        </Box>      
      </Box>
    </Modal>  
  );
};


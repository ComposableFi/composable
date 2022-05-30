import React, { useMemo } from "react";
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
} from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";

import { useDispatch } from "react-redux";
import {
  closePreviewSupplyModal,
  openConfirmingSupplyModal,
} from "@/stores/ui/uiSlice";
import { setCurrentSupply } from "@/stores/defi/pool";
import { YourPosition } from "../YourPosition";
import { SupplyModalProps } from "./ConfirmSupplyModal";

export const PreviewSupplyModal: React.FC<SupplyModalProps & ModalProps> = ({ 
  baseAsset,
  baseAmount,
  quoteAsset,
  quoteAmount,
  priceBaseInQuote,
  priceQuoteInBase,
  lpReceiveAmount,
  share,
  ...rest }) => {
  const theme = useTheme();
  const dispatch = useDispatch();

  const confirmSupply = () => {
    dispatch(closePreviewSupplyModal());
    dispatch(openConfirmingSupplyModal());

    setTimeout(() => {
      dispatch(setCurrentSupply({ confirmed: true }));
    }, 2000);
  };

  return (
    <Modal
      onClose={() => dispatch(closePreviewSupplyModal())}
      {...rest}
      PaperProps={{
        sx: {
          "& > .MuiBox-root": {
            height: "auto",
          },
        },
      }}
    >
      <Box
        sx={{
          background: theme.palette.gradient.secondary,
          boxShadow: `-1px -1px ${alpha(
            theme.palette.common.white,
            theme.custom.opacity.light
          )}`,
          width: 550,
          [theme.breakpoints.down("sm")]: {
            width: "100%",
          },
          borderRadius: 1,
          padding: theme.spacing(3),
          marginBottom: theme.spacing(4),
          marginTop: theme.spacing(4),
        }}
      >
        <Box display="flex" alignItems="center" justifyContent="space-between">
          <Typography variant="body1">You will recieve</Typography>
          <IconButton onClick={() => dispatch(closePreviewSupplyModal())}>
            <CloseIcon />
          </IconButton>
        </Box>

        <Typography variant="h5" mt={1.75}>
          {`${lpReceiveAmount}`}
        </Typography>

        <Typography variant="body1" color="text.secondary" mt={1.75}>
          {`LP ${baseAsset?.symbol}/${quoteAsset?.symbol} Tokens`}
        </Typography>

        <Typography variant="body2" mt={4} textAlign="center" paddingX={4.25}>
          Output is estimated. If the price changes by more than 5% your
          transaction will revert.
        </Typography>

        <Box
          mt={4}
          borderTop={`1px solid ${alpha(
            theme.palette.common.white,
            theme.custom.opacity.main
          )}`}
        />

        <Label
          mt={4}
          label={`Pooled ${baseAsset?.symbol}`}
          BalanceProps={{
            title: <BaseAsset icon={baseAsset?.icon} pr={1} />,
            balance: `${quoteAmount}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Pooled ${quoteAsset?.symbol}`}
          BalanceProps={{
            title: <BaseAsset icon={quoteAsset?.icon} pr={1} />,
            balance: `${baseAmount}`,
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
            balance: `${share}%`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Box
          display="flex"
          justifyContent="space-between"
          alignItems="center"
          mt={4}
          gap={2}
        >
          <Box width="50%">
            <Button
              variant="contained"
              size="large"
              fullWidth
              onClick={confirmSupply}
            >
              Auto bond
            </Button>
          </Box>

          <Box width="50%">
            <Button
              variant="text"
              size="large"
              fullWidth
              onClick={() => dispatch(closePreviewSupplyModal())}
            >
              No, thanks
            </Button>
          </Box>
        </Box>

        {quoteAsset && baseAsset ? (
          <YourPosition
            noTitle={false}
            tokenId1={quoteAsset.assetId}
            tokenId2={baseAsset.assetId}
            pooledAmount1={quoteAmount}
            pooledAmount2={baseAmount}
            amount={lpReceiveAmount}
            share={share}
            mt={4}
          />
        ) : null}
      </Box>
    </Modal>
  );
};

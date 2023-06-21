import React, { FC } from "react";
import { Modal, ModalProps } from "@/components/Molecules";
import { BaseAsset, Label } from "@/components/Atoms";
import {
  alpha,
  Box,
  Button,
  IconButton,
  Typography,
  useTheme,
} from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";
import { SwapSummary } from "./SwapSummary";
import KeyboardArrowDownIcon from "@mui/icons-material/KeyboardArrowDown";
import BigNumber from "bignumber.js";
import { Asset } from "shared";
import { useAppSettingsSlice } from "@/store/appSettings/slice";
import { setUiState } from "@/store/ui/ui.slice";
import { PoolConfig } from "@/store/pools/types";

export type PreviewModalProps = {
  setConfirmed?: (confirmed: boolean) => any;
  baseAsset: Asset | undefined;
  quoteAsset: Asset | undefined;
  baseAssetAmount: BigNumber;
  quoteAmount: BigNumber;
  feeCharged: BigNumber;
  minimumReceived: {
    asset: Asset | undefined;
    value: BigNumber;
  };
  spotPrice: BigNumber;
  priceImpact: BigNumber;
  selectedPool: PoolConfig;
  onConfirmSwap: () => void;
} & ModalProps;

export const PreviewModal: FC<PreviewModalProps> = ({
  setConfirmed,
  baseAsset,
  quoteAsset,
  quoteAmount,
  baseAssetAmount,
  minimumReceived,
  feeCharged,
  spotPrice,
  priceImpact,
  onConfirmSwap,
  selectedPool,
  ...modalProps
}) => {
  const theme = useTheme();

  const confirmSwap = () => {
    setUiState({
      isSwapPreviewModalOpen: false,
      isConfirmingModalOpen: true,
    });
    onConfirmSwap();
  };

  const slippage = useAppSettingsSlice().transactionSettings.tolerance;

  return (
    <Modal
      onClose={() => setUiState({ isSwapPreviewModalOpen: false })}
      {...modalProps}
    >
      <Box
        sx={{
          background: theme.palette.gradient.secondary,
          width: 560,
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
        <Box display="flex" alignItems="center" justifyContent="space-between">
          <Typography variant="h6">Confirm swap</Typography>
          <IconButton
            onClick={() => setUiState({ isSwapPreviewModalOpen: false })}
          >
            <CloseIcon sx={{ color: "text.secondary" }} />
          </IconButton>
        </Box>

        <Label
          mt={4}
          BalanceProps={{
            title: quoteAsset?.getSymbol(),
            TitleTypographyProps: {
              variant: "body1",
              color: "text.primary",
            },
          }}
        >
          <BaseAsset
            icon={quoteAsset?.getIconUrl()}
            label={quoteAmount.toFixed()}
            LabelProps={{ variant: "body1" }}
          />
        </Label>

        <Box mt={4}>
          <IconButton
            size="medium"
            sx={{
              background: alpha(
                theme.palette.primary.light,
                theme.custom.opacity.light
              ),
            }}
          >
            <KeyboardArrowDownIcon />
          </IconButton>
        </Box>

        <Label
          mt={4}
          BalanceProps={{
            title: baseAsset?.getSymbol(),
            TitleTypographyProps: {
              variant: "body1",
              color: "text.primary",
            },
          }}
        >
          <BaseAsset
            icon={baseAsset?.getIconUrl()}
            label={baseAssetAmount.toFixed()}
            LabelProps={{ variant: "body1" }}
          />
        </Label>

        <Typography variant="body2" mt={4} textAlign="center" paddingX={4.75}>
          Output is estimated. If the price changes by more than {slippage}%
          your transaction will revert.
        </Typography>

        <Box
          mt={4}
          borderTop={`2px solid ${alpha(
            theme.palette.common.white,
            theme.custom.opacity.light
          )}`}
        />

        <SwapSummary
          pool={selectedPool}
          mt={4}
          priceImpact={priceImpact}
          quoteAssetAmount={quoteAmount}
          baseAsset={baseAsset}
          quoteAsset={quoteAsset}
          minimumReceived={minimumReceived}
          baseAssetAmount={baseAssetAmount}
          feeCharged={feeCharged}
          spotPrice={spotPrice}
        />

        <Box mt={4}>
          <Button variant="contained" fullWidth onClick={confirmSwap}>
            Confirm Swap
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};

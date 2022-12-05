import { DropdownCombinedBigNumberInput } from "@/components";
import { useMobile } from "@/hooks/responsive";
import {
  alpha,
  Box,
  Button,
  Tooltip,
  Typography,
  useTheme,
} from "@mui/material";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useEffect, useMemo, useState } from "react";
import { BoxProps } from "@mui/system";
import { InfoOutlined, Settings, SwapVertRounded } from "@mui/icons-material";
import { TransactionSettings } from "../TransactionSettings";
import { SwapSummary } from "./SwapSummary";
import { SwapRoute } from "./SwapRoute";
import { PreviewModal } from "./PreviewModal";
import { ConfirmingModal } from "./ConfirmingModal";
import { useDotSamaContext, usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { useSwaps } from "@/defi/hooks/swaps/useSwaps";
import { usePabloSwap } from "@/defi/hooks/swaps/usePabloSwap";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { setUiState, useUiSlice } from "@/store/ui/ui.slice";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";

const SwapForm: React.FC<BoxProps> = ({ ...boxProps }) => {
  const isMobile = useMobile();
  const theme = useTheme();

  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { extensionStatus } = useDotSamaContext();

  const {
    balance1,
    balance2,
    changeAsset,
    selectedAssetOneId,
    selectedAssetTwoId,
    selectedAssetOne,
    selectedAssetTwo,
    assetListOne,
    assetListTwo,
    assetOneAmount,
    assetTwoAmount,
    onChangeTokenAmount,
    minimumReceived,
    feeCharged,
    spotPrice,
    valid,
    asset1PriceUsd,
    asset2PriceUsd,
    setAssetOneInputValid,
    setAssetTwoInputValid,
    assetOneInputValid,
    flipAssetSelection,
    percentageToSwap,
    priceImpact,
    inputMode,
    setInputMode
  } = useSwaps({ selectedAccount });

  const initiateSwapTx = usePabloSwap({
    baseAssetId: selectedAssetTwoId,
    quoteAssetId: selectedAssetOneId,
    quoteAmount: assetOneAmount,
    minimumReceived,
  });

  const isConfirmingModalOpen = usePendingExtrinsic(
    "exchange",
    "dexRouter",
    selectedAccount?.address ?? "-"
  )

  const { isSwapPreviewModalOpen } = useUiSlice();
  const [isConfirmed, setIsConfirmed] = useState<boolean>(false);

  const handleButtonClick = () => {
    if (extensionStatus !== "connected") {
      setUiState({ isPolkadotModalOpen: true });
    } else {
      setUiState({ isSwapPreviewModalOpen: true });
    }
  };

  const buttonText = useMemo(() => {
    if (extensionStatus !== "connected") {
      return "Connect wallet";
    }
    return "Swap";
  }, [extensionStatus]);

  useEffect(() => {
    if (isConfirmed) {
      setIsConfirmed(false);
      setUiState({ isConfirmingModalOpen: false })
    }
  }, [isConfirmed]);

  const onSettingHandler = () => {
    setUiState({ isTransactionSettingsModalOpen: true });
  };

  return (
    <HighlightBox margin="auto" {...boxProps}>
      <Box display="flex" justifyContent="space-between" alignItems="center">
        <Typography variant="h6">Swap</Typography>
        <Settings
          sx={{
            color: alpha(
              theme.palette.common.white,
              theme.custom.opacity.darker
            ),
            "&:hover": {
              color: theme.palette.common.white,
            },
            cursor: "pointer",
          }}
          onClick={onSettingHandler}
        />
      </Box>

      <Box mt={4}>
        <DropdownCombinedBigNumberInput
          isAnchorable
          maxValue={balance1.multipliedBy(percentageToSwap / 100)}
          setValid={setAssetOneInputValid}
          noBorder
          value={assetOneAmount}
          onMouseDown={() => {
            setInputMode(1)
          }}
          setValue={inputMode === 1 ? onChangeTokenAmount : undefined}
          buttonLabel={assetOneInputValid ? "Max" : undefined}
          referenceText={
            assetOneInputValid ? `${percentageToSwap}%` : undefined
          }
          ReferenceTextProps={{
            onClick: () => { },
            sx: {
              cursor: "pointer",
              "&:hover": {
                color: theme.palette.primary.main,
              },
            },
          }}
          ButtonProps={{
            onClick: () => {
              const balanceLimit = balance1.multipliedBy(
                percentageToSwap / 100
              );
              onChangeTokenAmount(balanceLimit)
            },
          }}
          CombinedSelectProps={{
            value: selectedAssetOneId,
            setValue: (val) => {
              changeAsset("quote", val);
            },
            dropdownModal: true,
            dropdownForceWidth: 320,
            forceHiddenLabel: isMobile ? true : false,
            renderShortLabel: true,
            options: [
              {
                value: "none",
                label: "Select",
                icon: undefined,
                disabled: true,
                hidden: true,
              },
              ...assetListOne,
            ],
            borderLeft: false,
            minWidth: isMobile ? undefined : 150,
            searchable: true,
          }}
          LabelProps={{
            label: "From",
            BalanceProps: selectedAssetOne
              ? {
                title: <AccountBalanceWalletIcon color="primary" />,
                balance: balance1.toFixed(4),
              }
              : undefined,
          }}
        />
      </Box>

      {valid && (
        <Typography variant="body2" mt={1.5}>
          {`≈$${assetOneAmount.multipliedBy(asset1PriceUsd)}`}
        </Typography>
      )}

      <Box mt={4} textAlign="center">
        <Box
          width={56}
          height={56}
          borderRadius="50%"
          display="flex"
          border={`2px solid ${theme.palette.primary.main}`}
          justifyContent="center"
          alignItems="center"
          margin="auto"
          sx={{
            cursor: "pointer",
            "&:hover": {
              background: alpha(theme.palette.primary.light, 0.1),
            },
          }}
        >
          <SwapVertRounded
            onClick={() => {
              flipAssetSelection();
            }}
          />
        </Box>
      </Box>

      <Box mt={4}>
        <DropdownCombinedBigNumberInput
          isAnchorable
          maxValue={balance2}
          setValid={setAssetTwoInputValid}
          noBorder
          value={assetTwoAmount}
          onMouseDown={() => {
            setInputMode(2)
          }}
          setValue={inputMode === 2 ? onChangeTokenAmount : undefined}
          ButtonProps={{
            onClick: () => { },
          }}
          CombinedSelectProps={{
            value: selectedAssetTwoId,
            setValue: (val) => {
              changeAsset("base", val);
            },
            dropdownModal: true,
            dropdownForceWidth: 320,
            forceHiddenLabel: isMobile ? true : false,
            renderShortLabel: true,
            options: [
              {
                value: "none",
                label: "Select",
                icon: undefined,
                disabled: true,
                hidden: true,
              },
              ...assetListTwo,
            ],
            borderLeft: false,
            minWidth: isMobile ? undefined : 150,
            searchable: true,
          }}
          LabelProps={{
            label: "To",
            BalanceProps: selectedAssetTwo
              ? {
                title: <AccountBalanceWalletIcon color="primary" />,
                balance: balance2.toFixed(4),
              }
              : undefined,
          }}
        />
      </Box>

      {valid && (
        <Typography variant="body2" mt={1.5}>
          {`≈$${assetTwoAmount.multipliedBy(asset2PriceUsd)}`}
        </Typography>
      )}
      <Box
        mt={4}
        display="flex"
        justifyContent="center"
        alignItems="center"
        gap={2}
        height={26}
      >
        {selectedAssetOne && selectedAssetTwo && (
          <>
            <Typography variant="body2">
              1 {selectedAssetTwo.getSymbol()} = {spotPrice.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}{" "}
              {selectedAssetOne.getSymbol()}
            </Typography>
            <Tooltip
              title={`1 ${selectedAssetOne?.getSymbol()} = ${spotPrice.toFixed(DEFAULT_UI_FORMAT_DECIMALS)} ${selectedAssetTwo.getSymbol()
                }`}
              placement="top"
            >
              <InfoOutlined sx={{ color: theme.palette.primary.main }} />
            </Tooltip>
          </>
        )}
      </Box>

      <Box mt={4}>
        <Button
          onClick={handleButtonClick}
          variant="contained"
          fullWidth
          disabled={extensionStatus === "connected" && !valid}
        >
          {buttonText}
        </Button>
      </Box>

      {valid && (
        <SwapSummary
          priceImpact={priceImpact}
          mt={4}
          spotPrice={spotPrice}
          baseAssetAmount={assetTwoAmount}
          quoteAsset={selectedAssetOne}
          baseAsset={selectedAssetTwo}
          quoteAssetAmount={assetOneAmount}
          minimumReceived={minimumReceived}
          feeCharged={feeCharged}
        />
      )}

      {valid && (
        <>
          <SwapRoute
            mt={4}
            quoteAsset={selectedAssetOne}
            baseAsset={selectedAssetTwo}
          />
          <PreviewModal
            priceImpact={priceImpact}
            onConfirmSwap={initiateSwapTx}
            minimumReceived={minimumReceived}
            baseAssetAmount={assetTwoAmount}
            quoteAmount={assetOneAmount}
            quoteAsset={selectedAssetOne}
            baseAsset={selectedAssetTwo}
            open={isSwapPreviewModalOpen}
            setConfirmed={setIsConfirmed}
            spotPrice={spotPrice}
            feeCharged={feeCharged}
          />
          <ConfirmingModal open={isConfirmingModalOpen} />
        </>
      )}

      <TransactionSettings />
    </HighlightBox>
  );
};

export default SwapForm;

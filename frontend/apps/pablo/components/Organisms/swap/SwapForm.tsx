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
import { useAppSelector } from "@/hooks/store";
import { useDispatch } from "react-redux";
import { InfoOutlined, Settings, SwapVertRounded } from "@mui/icons-material";
import {
  closeConfirmingModal,
  openSwapPreviewModal,
  openTransactionSettingsModal,
  setMessage,
} from "@/stores/ui/uiSlice";
import { TransactionSettings } from "../TransactionSettings";
import { SwapSummary } from "./SwapSummary";
import { SwapRoute } from "./SwapRoute";
import { PreviewModal } from "./PreviewModal";
import { ConfirmingModal } from "./ConfirmingModal";
import { useDotSamaContext } from "substrate-react";
import { useSwaps } from "@/defi/hooks/swaps/useSwaps";
import _ from "lodash";
import { usePabloSwap } from "@/defi/hooks/swaps/usePabloSwap";
import useStore from "@/store/useStore";
import { HighlightBox } from "@/components/Atoms/HighlightBox";

const SwapForm: React.FC<BoxProps> = ({ ...boxProps }) => {
  const isMobile = useMobile();
  const theme = useTheme();
  const dispatch = useDispatch();
  const { openPolkadotModal } = useStore();

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
    assetTwoInputValid,
    flipAssetSelection,
    isProcessing,
    percentageToSwap,
    priceImpact,
  } = useSwaps();

  const initiateSwapTx = usePabloSwap({
    baseAssetId: selectedAssetTwoId,
    quoteAssetId: selectedAssetOneId,
    quoteAmount: assetOneAmount,
    minimumReceived,
  });

  const onConfirmSwap = async () => {
    initiateSwapTx()
      .then(() => {
        dispatch(closeConfirmingModal());
      })
      .catch((err) => {
        console.error(err);
        dispatch(closeConfirmingModal());
      });
  };

  const isSwapPreviewModalOpen = useAppSelector(
    (state) => state.ui.isSwapPreviewModalOpen
  );
  const isConfirmingModalOpen = useAppSelector(
    (state) => state.ui.isConfirmingModalOpen
  );
  const [isConfirmed, setIsConfirmed] = useState<boolean>(false);

  const handleButtonClick = () => {
    if (extensionStatus !== "connected") {
      openPolkadotModal();
    } else {
      dispatch(openSwapPreviewModal());
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
      dispatch(
        setMessage({
          text: "Transaction successful",
          link: "/",
          severity: "success",
        })
      );
      setIsConfirmed(false);
      dispatch(closeConfirmingModal());
    }
  }, [isConfirmed, dispatch]);

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

  const debouncedTokenAmountUpdate = _.debounce(onChangeTokenAmount, 1000);

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
          setValue={(val) => {
            if (isProcessing) return;
            debouncedTokenAmountUpdate("quote", val);
          }}
          InputProps={{
            disabled: isProcessing,
          }}
          buttonLabel={assetOneInputValid ? "Max" : undefined}
          referenceText={
            assetOneInputValid ? `${percentageToSwap}%` : undefined
          }
          ReferenceTextProps={{
            onClick: () => {},
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
              if (!isProcessing && balanceLimit.gt(0)) {
                debouncedTokenAmountUpdate("quote", balanceLimit);
              }
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
          setValue={(val) => {
            if (isProcessing) return;
            debouncedTokenAmountUpdate("base", val);
          }}
          InputProps={{
            disabled: isProcessing,
          }}
          ButtonProps={{
            onClick: () => {},
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
              1 {selectedAssetTwo.symbol} = {spotPrice.toFixed()}{" "}
              {selectedAssetOne.symbol}
            </Typography>
            <Tooltip
              title={`1 ${selectedAssetOne?.symbol} = ${spotPrice.toFixed()} ${
                selectedAssetTwo.symbol
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
            onConfirmSwap={onConfirmSwap}
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

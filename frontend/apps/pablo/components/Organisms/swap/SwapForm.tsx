import { DropdownCombinedBigNumberInput } from "@/components";
import { useMobile } from "@/hooks/responsive";
import { Box, Tooltip, Typography, useTheme } from "@mui/material";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { FC, useEffect, useState } from "react";
import { BoxProps } from "@mui/system";
import { InfoOutlined } from "@mui/icons-material";
import { TransactionSettings } from "../TransactionSettings";
import { SwapSummary } from "./SwapSummary";
import { SwapRoute } from "./SwapRoute";
import { PreviewModal } from "./PreviewModal";
import { ConfirmingModal } from "./ConfirmingModal";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { useSwaps } from "@/defi/hooks/swaps/useSwaps";
import { usePabloSwap } from "@/defi/hooks/swaps/usePabloSwap";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { setUiState, useUiSlice } from "@/store/ui/ui.slice";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { SettingsModalHandler } from "./SettingsModalHandler";
import { SwapVertAsset } from "@/components/Organisms/swap/SwapVertAsset";
import { SwapButton } from "@/components/Organisms/swap/SwapButton";
import BigNumber from "bignumber.js";

const SwapForm: FC<BoxProps> = ({ ...boxProps }) => {
  const isMobile = useMobile();
  const theme = useTheme();

  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  const {
    balance1,
    balance2,
    changeAsset,
    selectedAssetOneId,
    selectedAssetTwoId,
    selectedAssetOne,
    selectedAssetTwo,
    assetList,
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
    percentageToSwap,
    priceImpact,
    inputMode,
    setInputMode,
    pabloPool,
  } = useSwaps({ selectedAccount });

  const initiateSwapTx = usePabloSwap({
    pool: pabloPool,
    baseAsset: pabloPool?.config.assets.find(
      (a) => a.getPicassoAssetId() === selectedAssetTwoId
    ),
    quoteAsset: pabloPool?.config.assets.find(
      (b) => b.getPicassoAssetId() === selectedAssetOneId
    ),
    quoteAmount: assetOneAmount,
    minimumReceived: minimumReceived.value,
  });

  const isConfirmingModalOpen = usePendingExtrinsic(
    "pablo",
    "swap",
    selectedAccount?.address ?? "-"
  );

  const { isSwapPreviewModalOpen } = useUiSlice();
  const [isConfirmed, setIsConfirmed] = useState<boolean>(false);

  useEffect(() => {
    if (isConfirmed) {
      setIsConfirmed(false);
      setUiState({ isConfirmingModalOpen: false });
    }
  }, [isConfirmed]);

  return (
    <HighlightBox margin="auto" {...boxProps}>
      <Box display="flex" justifyContent="space-between" alignItems="center">
        <Typography variant="h6">Swap</Typography>
        <SettingsModalHandler />
      </Box>

      <Box mt={4}>
        <DropdownCombinedBigNumberInput
          isAnchorable
          maxValue={balance1}
          setValid={setAssetOneInputValid}
          noBorder
          value={assetOneAmount.decimalPlaces(
            selectedAssetOne?.getDecimals(DEFAULT_NETWORK_ID) || 12
          )}
          onMouseDown={() => {
            setInputMode(1);
          }}
          setValue={inputMode === 1 ? onChangeTokenAmount : undefined}
          buttonLabel={assetOneInputValid ? "Max" : undefined}
          referenceText={
            assetOneInputValid ? `${percentageToSwap}%` : undefined
          }
          ReferenceTextProps={{
            onClick: () => {
              onChangeTokenAmount(
                balance1.multipliedBy(percentageToSwap / 100)
              );
            },
            sx: {
              cursor: "pointer",
              "&:hover": {
                color: theme.palette.primary.main,
              },
            },
          }}
          ButtonProps={{
            onClick: () => {
              const balanceLimit = balance1.multipliedBy(percentageToSwap);
              onChangeTokenAmount(balanceLimit);
            },
          }}
          CombinedSelectProps={{
            value: selectedAssetOneId,
            setValue: (val) => {
              changeAsset("quote", val);
            },
            dropdownModal: true,
            dropdownForceWidth: 320,
            forceHiddenLabel: isMobile,
            renderShortLabel: true,
            options: [
              {
                value: "none",
                label: "Select",
                icon: undefined,
                disabled: true,
                hidden: true,
              },
              ...assetList.filter(
                (asset) => asset.value !== selectedAssetTwoId
              ),
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
          {`≈$${assetOneAmount.multipliedBy(asset1PriceUsd).toFormat(4)}`}
        </Typography>
      )}

      <SwapVertAsset selectedAccount={selectedAccount} />
      <Box mt={4}>
        <DropdownCombinedBigNumberInput
          isAnchorable
          maxValue={balance2}
          setValid={setAssetTwoInputValid}
          noBorder
          value={assetTwoAmount.decimalPlaces(
            selectedAssetTwo?.getDecimals(DEFAULT_NETWORK_ID) || 12
          )}
          onMouseDown={() => {
            setInputMode(2);
          }}
          setValue={inputMode === 2 ? onChangeTokenAmount : undefined}
          referenceText={
            assetTwoInputValid ? `${percentageToSwap}%` : undefined
          }
          ReferenceTextProps={{
            onClick: () => {
              onChangeTokenAmount(
                balance2.multipliedBy(percentageToSwap / 100)
              );
            },
            sx: {
              cursor: "pointer",
              "&:hover": {
                color: theme.palette.primary.main,
              },
            },
          }}
          buttonLabel={assetTwoInputValid ? "Max" : undefined}
          ButtonProps={{
            onClick: () => {
              onChangeTokenAmount(balance2);
            },
          }}
          CombinedSelectProps={{
            value: selectedAssetTwoId,
            setValue: (val) => {
              changeAsset("base", val);
            },
            dropdownModal: true,
            dropdownForceWidth: 320,
            forceHiddenLabel: isMobile,
            renderShortLabel: true,
            options: [
              {
                value: "none",
                label: "Select",
                icon: undefined,
                disabled: true,
                hidden: true,
              },
              ...assetList.filter(
                (asset) => asset.value !== selectedAssetOneId
              ),
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
          {`≈$${assetTwoAmount.multipliedBy(asset2PriceUsd).toFormat(4)}`}
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
              1 {selectedAssetTwo.getSymbol()} ={" "}
              {spotPrice.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}{" "}
              {selectedAssetOne.getSymbol()}
            </Typography>
            <Tooltip
              title={`1 ${selectedAssetOne?.getSymbol()} = ${new BigNumber(1)
                .div(spotPrice)
                .toFixed(
                  DEFAULT_UI_FORMAT_DECIMALS
                )} ${selectedAssetTwo.getSymbol()}`}
              placement="top"
            >
              <InfoOutlined sx={{ color: theme.palette.primary.main }} />
            </Tooltip>
          </>
        )}
      </Box>

      <SwapButton />

      {valid && pabloPool && (
        <SwapSummary
          pool={pabloPool}
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

      {valid && pabloPool && (
        <>
          <SwapRoute
            mt={4}
            quoteAsset={selectedAssetOne}
            baseAsset={selectedAssetTwo}
          />
          <PreviewModal
            selectedPool={pabloPool}
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

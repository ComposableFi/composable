import { DropdownCombinedBigNumberInput, BigNumberInput } from "@/components";
import { useMobile } from "@/hooks/responsive";
import {
  Box,
  Button,
  Typography,
  useTheme,
  alpha,
  Tooltip,
} from "@mui/material";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useCallback, useEffect, useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import { BoxProps } from "@mui/system";
import { useAppSelector } from "@/hooks/store";
import { useDispatch } from "react-redux";
import { InfoOutlined, Settings, SwapVertRounded } from "@mui/icons-material";
import {
  closeConfirmingModal,
  openPolkadotModal,
  openSwapPreviewModal,
  openTransactionSettingsModal,
  setMessage,
} from "@/stores/ui/uiSlice";
import { TransactionSettings } from "../TransactionSettings";
import { SwapSummary } from "./SwapSummary";
import { SwapRoute } from "./SwapRoute";
import { PreviewModal } from "./PreviewModal";
import { ConfirmingModal } from "./ConfirmingModal";
import { useDotSamaContext, useParachainApi } from "substrate-react";
import useStore from "@/store/useStore";
import { AssetId } from "@/defi/polkadot/types";
import { debounce } from "lodash";
import { calculateSwap } from "@/defi/utils/pablo/swaps";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useAssetBalance, useUSDPriceByAssetId } from "@/store/assets/hooks";

const SwapForm: React.FC<BoxProps> = ({ ...boxProps }) => {
  const isMobile = useMobile();
  const theme = useTheme();
  const dispatch = useDispatch();
  
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { extensionStatus } = useDotSamaContext();
  
  const { swaps, setUiAssetSelectionSwaps, invertAssetSelectionSwaps, supportedAssets } = useStore();
  const [valid1, setValid1] = useState<boolean>(false);
  const [valid2, setValid2] = useState<boolean>(false);

  const token1PriceInUSD = useUSDPriceByAssetId(swaps.ui.baseAssetSelected);
  const token2PriceInUSD = useUSDPriceByAssetId(swaps.ui.quoteAssetSelected);

  const balance1 = useAssetBalance(DEFAULT_NETWORK_ID, swaps.ui.baseAssetSelected);
  const balance2 = useAssetBalance(DEFAULT_NETWORK_ID, swaps.ui.quoteAssetSelected);

  const { baseAssetSelected, quoteAssetSelected } = useMemo(() => {
    let baseAssetSelected = supportedAssets.find(i => i.network[DEFAULT_NETWORK_ID] === swaps.ui.baseAssetSelected)
    let quoteAssetSelected = supportedAssets.find(i => i.network[DEFAULT_NETWORK_ID] === swaps.ui.quoteAssetSelected)
    return {
      baseAssetSelected,
      quoteAssetSelected
    }
  }, [supportedAssets, swaps.ui]);

  const assetList1 = useMemo(() => {
    return supportedAssets
      .filter((i) => {
        if (!baseAssetSelected) return true;
        if (baseAssetSelected.name !== i.name) return true;
      })
      .map((asset) => ({
        value: asset.network[DEFAULT_NETWORK_ID],
        label: asset.name,
        shortLabel: asset.symbol,
        icon: asset.icon,
      }));
  }, [supportedAssets, baseAssetSelected]);

  const assetList2 = useMemo(() => {
    return supportedAssets
      .filter((i) => {
        if (!quoteAssetSelected) return true;
        if (quoteAssetSelected.name !== i.name) return true;
      })
      .map((asset) => ({
        value: asset.network[DEFAULT_NETWORK_ID],
        label: asset.name,
        shortLabel: asset.symbol,
        icon: asset.icon,
      }));
  }, [supportedAssets, quoteAssetSelected]);

  const percentageToSwap = useAppSelector(
    (state) => state.swap.percentageToSwap
  );

  const slippage = useAppSelector(
    (state) => state.settings.transactionSettings.tolerance
  );

  const spotPriceBn = useMemo(() => {
    return new BigNumber(swaps.poolVariables.spotPrice);
  }, [swaps.poolVariables]);

  const [baseAssetAmount, setBaseAssetAmount] = useState(new BigNumber(0));
  const [quoteAssetAmount, setQuoteAssetAmount] = useState(new BigNumber(0));
  const [minimumReceived, setMinimumReceived] = useState(new BigNumber(0));
  const [priceImpact, setPriceImpact] = useState(new BigNumber(0));

  useEffect(() => {
    setIsProcessing(true);

    if (swaps.dexRouter.dexRoute.length === 0) {
      setQuoteAssetAmount(new BigNumber(0));
      setBaseAssetAmount(new BigNumber(0));
    }
  }, [swaps.dexRouter.dexRoute]);

  const isSwapPreviewModalOpen = useAppSelector(
    (state) => state.ui.isSwapPreviewModalOpen
  );
  const isConfirmingModalOpen = useAppSelector(
    (state) => state.ui.isConfirmingModalOpen
  );
  const [isConfirmed, setIsConfirmed] = useState<boolean>(false);

  const setAssetId = (side: "base" | "quote") => (
    assetId: string | "none"
  ) => {
    setUiAssetSelectionSwaps(side, assetId);
  };

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

  const validToken1 = swaps.ui.quoteAssetSelected !== "none";
  const validToken2 = swaps.ui.baseAssetSelected !== "none";
  const validTokens = validToken1 && validToken2;

  const handleSwap = () => {
    dispatch(closeConfirmingModal());
    dispatch(openSwapPreviewModal());
  };

  const valid =
    valid1 && valid2 && validTokens && swaps.dexRouter.dexRoute.length >= 1;

  const handleButtonClick = () => {
    if (extensionStatus !== "connected") {
      dispatch(openPolkadotModal());
    } else {
      handleSwap();
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
  }, [isConfirmed]);

  const [isProcessing, setIsProcessing] = useState(false);

  const onSwapAmountInput = (swapAmount: {
    value: BigNumber;
    side: "quote" | "base";
  }) => {
    setIsProcessing(true);
    const { ui, dexRouter } = swaps;
    
    if (
      parachainApi &&
      ui.baseAssetSelected !== "none" &&
      ui.quoteAssetSelected !== "none" &&
      dexRouter.dexRoute.length
    ) {
      const { value, side } = swapAmount;
      if (side === "base") {
        setBaseAssetAmount(swapAmount.value);
      } else {
        setQuoteAssetAmount(swapAmount.value);
      }

      const { baseAssetSelected, quoteAssetSelected } = ui;

      const exchangeParams = {
        quoteAmount: value,
        baseAssetId: baseAssetSelected,
        quoteAssetId: quoteAssetSelected,
        side: side,
        slippage,
      };

      calculateSwap(
        parachainApi,
        exchangeParams,
        swaps.poolConstants
      ).then((impact) => {
        swapAmount.side === "base"
          ? setQuoteAssetAmount(new BigNumber(impact.tokenOutAmount))
          : setBaseAssetAmount(new BigNumber(impact.tokenOutAmount));
        setMinimumReceived(new BigNumber(impact.minimumRecieved));
        setPriceImpact(new BigNumber(impact.priceImpact));

        setTimeout(() => {
          setIsProcessing(false);
        }, 500);
      });
    }
  };

  const handler = debounce(onSwapAmountInput, 1000);

  return (
    <Box
      borderRadius={1.33}
      margin="auto"
      sx={{
        width: "100%",
        height: "100%",
        padding: theme.spacing(4),
        [theme.breakpoints.down("sm")]: {
          padding: theme.spacing(2),
        },
        background: theme.palette.gradient.secondary,
        border: `1px solid ${alpha(theme.palette.common.white, 0.1)}`,
        boxShadow: `-1px -1px ${alpha(
          theme.palette.common.white,
          theme.custom.opacity.light
        )}`,
      }}
      {...boxProps}
    >
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
          setValid={setValid1}
          noBorder
          value={quoteAssetAmount}
          onMouseDown={(evt) => setIsProcessing(false)}
          setValue={(val) => {
            if (isProcessing) return;
            handler({
              value: val,
              side: "quote",
            });
          }}
          InputProps={{
            disabled: !validToken1,
          }}
          buttonLabel={validToken1 ? "Max" : undefined}
          referenceText={validToken1 ? `${percentageToSwap}%` : undefined}
          ReferenceTextProps={{
            onClick: () =>
            handler({
              value: balance1.multipliedBy(0.5),
              side: "quote"
            }),
            sx: {
              cursor: "pointer",
              "&:hover": {
                color: theme.palette.primary.main,
              },
            },
          }}
          ButtonProps={{
            onClick: () => {
              handler({
                value: balance1,
                side: "quote"
              })
            },
          }}
          CombinedSelectProps={{
            value: swaps.ui.quoteAssetSelected,
            setValue: setAssetId("quote"),
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
              ...assetList1,
            ],
            borderLeft: false,
            minWidth: isMobile ? undefined : 150,
            searchable: true,
          }}
          LabelProps={{
            label: "From",
            BalanceProps: validToken1
              ? {
                  title: <AccountBalanceWalletIcon color="primary" />,
                  balance: balance1.toFixed(4),
                }
              : undefined,
          }}
        />
      </Box>

      {valid1 && (
        <Typography variant="body2" mt={1.5}>
          {`≈$${quoteAssetAmount.multipliedBy(token1PriceInUSD)}`}
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
          <SwapVertRounded onClick={invertAssetSelectionSwaps} />
        </Box>
      </Box>

      <Box mt={4}>
        <DropdownCombinedBigNumberInput
          isAnchorable
          maxValue={balance2}
          setValid={setValid2}
          noBorder
          value={baseAssetAmount}
          onMouseDown={(evt) => setIsProcessing(false)}
          setValue={(val) => {
            if (isProcessing) return;
            handler({
              value: val,
              side: "base",
            });
          }}
          InputProps={{
            disabled: !validToken2,
          }}
          ButtonProps={{
            onClick: () => {
              handler({
                value: balance2,
                side: "base"
              })
            },
          }}
          CombinedSelectProps={{
            value: swaps.ui.baseAssetSelected,
            setValue: setAssetId("base"),
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
              ...assetList2,
            ],
            borderLeft: false,
            minWidth: isMobile ? undefined : 150,
            searchable: true,
          }}
          LabelProps={{
            label: "To",
            BalanceProps: validToken2
              ? {
                  title: <AccountBalanceWalletIcon color="primary" />,
                  balance: balance2.toFixed(4),
                }
              : undefined,
          }}
        />
      </Box>

      {valid2 && (
        <Typography variant="body2" mt={1.5}>
          {`≈$${baseAssetAmount.multipliedBy(token2PriceInUSD)}`}
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
        {validTokens && (
          <>
            <Typography variant="body2">
              1 {quoteAssetSelected?.symbol} = {spotPriceBn.toFixed()}{" "}
              {baseAssetSelected?.decimals}
            </Typography>
            <Tooltip
              title={`1 ${
                quoteAssetSelected?.symbol
              } = ${spotPriceBn.toFixed()} ${baseAssetSelected?.symbol}`}
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
          disabled={!valid}
        >
          {buttonText}
        </Button>
      </Box>

      {valid && (
        <SwapSummary
          mt={4}
          poolType={swaps.poolConstants.poolType}
          baseAssetAmount={baseAssetAmount}
          quoteAsset={quoteAssetSelected}
          baseAsset={baseAssetSelected}
          quoteAssetAmount={quoteAssetAmount}
          minimumReceived={minimumReceived}
          priceImpact={priceImpact.toNumber()}
          fee={new BigNumber(swaps.poolConstants.fee).div(100)}
        />
      )}

      {valid && (
        <>
          <SwapRoute
            mt={4}
            quoteAsset={quoteAssetSelected}
            baseAsset={baseAssetSelected}
          />
          <PreviewModal
            minimumReceived={minimumReceived}
            baseAssetAmount={baseAssetAmount}
            quoteAssetAmount={quoteAssetAmount}
            quoteAsset={quoteAssetSelected}
            baseAsset={baseAssetSelected}
            open={isSwapPreviewModalOpen}
            setConfirmed={setIsConfirmed}
          />
          <ConfirmingModal open={isConfirmingModalOpen} />
        </>
      )}

      <TransactionSettings />
    </Box>
  );
};

export default SwapForm;

import {
  alpha,
  Box,
  BoxProps,
  Button,
  Typography,
  useTheme,
} from "@mui/material";
import { Settings } from "@mui/icons-material";
import { useState } from "react";
import {
  DropdownCombinedBigNumberInput,
  BigNumberInput,
  TransactionSettings,
} from "@/components";
import { useMobile } from "@/hooks/responsive";
import { getFullHumanizedDateDiff, PabloLiquidityBootstrappingPool } from "shared";
import { ConfirmingModal } from "../swap/ConfirmingModal";
import { DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { useAuctionBuyForm } from "@/defi/hooks/auctions/useAuctionBuyForm";
import { useDotSamaContext } from "substrate-react";
import { usePabloSwap } from "@/defi/hooks/swaps/usePabloSwap";
import { setUiState } from "@/store/ui/ui.slice";
import _ from "lodash";
import { useAssetIdOraclePrice } from "@/defi/hooks";
import { useAuctionTiming } from "@/defi/hooks/auctions/useAuctionTiming";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";

export type BuyFormProps = {
  auction: PabloLiquidityBootstrappingPool;
} & BoxProps;

export const BuyForm: React.FC<BuyFormProps> = ({ auction, ...rest }) => {
  const { extensionStatus } = useDotSamaContext();
  const theme = useTheme();
  const isMobile = useMobile();
  const [manualUpdateMode, setManualUpdateMode] = useState<1 | 2>(1);
  const {
    balanceBase,
    balanceQuote,
    baseAmount,
    quoteAmount,
    baseAsset,
    quoteAsset,
    onChangeTokenAmount,
    isPendingBuy,
    isValidBaseInput,
    isValidQuoteInput,
    setIsValidBaseInput,
    setIsValidQuoteInput,
    isBuyButtonDisabled,
    selectedAuction,
    minimumReceived
  } = useAuctionBuyForm();

  const priceUSDBase = useAssetIdOraclePrice(
    baseAsset ? baseAsset.getPicassoAssetId() as string : "none"
  );
  const priceUSDQuote = useAssetIdOraclePrice(
    quoteAsset ? quoteAsset.getPicassoAssetId() as string : "none"
  );

  const { isActive, isEnded, startTimestamp } = useAuctionTiming(auction);

  const initiateBuyTx = usePabloSwap({
    baseAssetId: selectedAuction?.getPair().getBaseAsset().toString() ?? "-",
    quoteAssetId: selectedAuction?.getPair().getQuoteAsset().toString() ?? "-",
    quoteAmount,
    minimumReceived,
    swapOrigin: "Auction"
  });

  const onSettingHandler = () => {
    setUiState({ isTransactionSettingsModalOpen: true })
  };

  return (
    <Box
      position="relative"
      sx={{
        background: theme.palette.gradient.secondary,
        borderRadius: 1,
        padding: theme.spacing(4),
        [theme.breakpoints.down("md")]: {
          padding: theme.spacing(2),
        },
      }}
      {...rest}
    >
      <Box visibility={isActive ? undefined : "hidden"}>
        <Box display="flex" justifyContent="end" alignItems="center">
          <Settings
            sx={{
              marginBottom: theme.spacing(2),
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
        <DropdownCombinedBigNumberInput
          maxValue={balanceQuote}
          setValid={setIsValidQuoteInput}
          noBorder
          onMouseDown={_evt => {
            if (manualUpdateMode !== 1) setManualUpdateMode(1)
          }}
          value={quoteAmount}
          setValue={(value) => {
            manualUpdateMode === 1 ?
              onChangeTokenAmount("quote", value) : undefined
          }}
          buttonLabel={"Max"}
          ButtonProps={{
            onClick: () => {
              onChangeTokenAmount("quote", balanceQuote);
            },
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={{
            value: quoteAsset?.getPicassoAssetId() as string ?? "",
            dropdownModal: true,
            forceHiddenLabel: isMobile ? true : false,
            options: [
              {
                value: "none",
                label: "Select",
                icon: undefined,
                disabled: true,
                hidden: true,
              },
              ...(quoteAsset
                ? [
                  {
                    value: quoteAsset.getPicassoAssetId(),
                    icon: quoteAsset.getIconUrl(),
                    label: quoteAsset.getSymbol(),
                  },
                ]
                : []),
            ],
            borderLeft: false,
            minWidth: isMobile ? undefined : 150,
            searchable: true,
          }}
          LabelProps={{
            label: "Currency",
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${balanceQuote.toFixed(4)}`,
            },
          }}
        />
        {isValidQuoteInput && (
          <Typography variant="body2" mt={1.5}>
            {`≈$${quoteAmount.multipliedBy(priceUSDQuote).toFixed(DEFAULT_UI_FORMAT_DECIMALS)}`}
          </Typography>
        )}
      </Box>
      <Box
        mt={4}
        textAlign="center"
        visibility={isActive ? undefined : "hidden"}
      >
        <Box
          width={56}
          height={56}
          borderRadius="50%"
          display="flex"
          justifyContent="center"
          alignItems="center"
          margin="auto"
          sx={{
            background: alpha(
              theme.palette.primary.main,
              theme.custom.opacity.light
            ),
          }}
        >
          <ExpandMoreIcon />
        </Box>
      </Box>
      <Box mt={4} visibility={isActive ? undefined : "hidden"}>
        <BigNumberInput
          onMouseDown={_evt => {
            if (manualUpdateMode !== 2) setManualUpdateMode(2)
          }}
          value={baseAmount}
          setValue={(value) => {
            manualUpdateMode === 2 ?
              onChangeTokenAmount("base", value) : undefined
          }}
          maxValue={balanceBase}
          setValid={setIsValidBaseInput}
          EndAdornmentAssetProps={{
            assets: baseAsset
              ? [
                {
                  icon: baseAsset.getIconUrl(),
                  label: baseAsset.getSymbol(),
                },
              ]
              : [],
          }}
          LabelProps={{
            label: "Launch token",
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${balanceBase.toFixed(4)}`,
            },
          }}
        />
        {isValidBaseInput && (
          <Typography variant="body2" mt={1.5}>
            {`≈$${baseAmount.multipliedBy(priceUSDBase)}`}
          </Typography>
        )}
      </Box>

      <Box mt={4}>
        {extensionStatus === "connected" && (
          <Button
            variant="contained"
            fullWidth
            disabled={isPendingBuy || isBuyButtonDisabled}
            onClick={initiateBuyTx}
          >
            Buy {baseAsset?.getSymbol() ?? ""}
          </Button>
        )}

        {extensionStatus !== "connected" && (
          <Button
            variant="contained"
            fullWidth
            onClick={() => {
              setUiState({ isPolkadotModalOpen: true })
            }}
          >
            Connect wallet
          </Button>
        )}
      </Box>
      {!isActive && (
        <Box
          height="100%"
          width="100%"
          position="absolute"
          sx={{
            bottom: 0,
            left: 0,
            right: 0,
            borderRadius: 1,
            backgroundColor: alpha(
              theme.palette.common.white,
              theme.custom.opacity.lightest
            ),
            backdropFilter: "blur(8px)",
            padding: theme.spacing(4),
          }}
          textAlign="center"
        >
          {isEnded ? (
            <>
              <Typography variant="subtitle1" fontWeight={600}>
                The LBP has ended
              </Typography>
              <Typography variant="body1" mt={1.5}>
                Check the lists for more
              </Typography>
              <Typography variant="body1">upcoming LBP.</Typography>
            </>
          ) : (
            <>
              <Typography variant="subtitle1" fontWeight={600}>
                The LBP has not started
              </Typography>
              <Typography variant="body1" mt={1.5}>
                The LBP starts in{" "}
                {getFullHumanizedDateDiff(Date.now(), startTimestamp)}.
              </Typography>
              <Typography variant="body1">
                Swapping will be enabling by the
              </Typography>
              <Typography variant="body1">
                LBP creator at start time.
              </Typography>
            </>
          )}
        </Box>
      )}
      <ConfirmingModal open={isPendingBuy} />
      <TransactionSettings />
    </Box>
  );
};

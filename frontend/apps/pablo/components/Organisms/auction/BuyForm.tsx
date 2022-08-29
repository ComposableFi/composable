import {
  alpha,
  Box,
  BoxProps,
  Button,
  Typography,
  useTheme,
} from "@mui/material";
import { Settings } from "@mui/icons-material";
import { useCallback } from "react";
import {
  DropdownCombinedBigNumberInput,
  BigNumberInput,
  TransactionSettings,
} from "@/components";
import { useMobile } from "@/hooks/responsive";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import { useAppDispatch } from "@/hooks/store";
import {
  openPolkadotModal,
  openTransactionSettingsModal,
} from "@/stores/ui/uiSlice";
import { getFullHumanizedDateDiff } from "shared";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { ConfirmingModal } from "../swap/ConfirmingModal";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useBuyForm } from "@/defi/hooks/auctions/useBuyForm";
import _ from "lodash";
import { useDotSamaContext } from "substrate-react";
import { usePabloSwap } from "@/defi/hooks/swaps/usePabloSwap";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";

export type BuyFormProps = {
  auction: LiquidityBootstrappingPool;
} & BoxProps;

export const BuyForm: React.FC<BuyFormProps> = ({ auction, ...rest }) => {
  const { extensionStatus } = useDotSamaContext();
  const theme = useTheme();
  const isMobile = useMobile();
  const currentTimestamp = Date.now();

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
    minimumReceived,
    isProcessing,
  } = useBuyForm();

  const priceUSDBase = useUSDPriceByAssetId(
    baseAsset ? baseAsset.network[DEFAULT_NETWORK_ID] : "none"
  );
  const priceUSDQuote = useUSDPriceByAssetId(
    quoteAsset ? quoteAsset.network[DEFAULT_NETWORK_ID] : "none"
  );

  const isActive: boolean =
    auction.sale.start <= currentTimestamp &&
    auction.sale.end >= currentTimestamp;
  const isEnded: boolean = auction.sale.end < currentTimestamp;

  const dispatch = useAppDispatch();

  const debouncedTokenAmountUpdate = _.debounce(onChangeTokenAmount, 1000);

  const initiateBuyTx = usePabloSwap({
    baseAssetId: selectedAuction.pair.base.toString(),
    quoteAssetId: selectedAuction.pair.quote.toString(),
    quoteAmount,
    minimumReceived,
  });

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

  const handleBuy = useCallback(async () => {
    await initiateBuyTx();
  }, [initiateBuyTx]);

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
          disabled={isProcessing}
          value={quoteAmount}
          setValue={(value) => {
            if (isProcessing) return;
            debouncedTokenAmountUpdate("quote", value);
          }}
          buttonLabel={"Max"}
          ButtonProps={{
            onClick: () => {
              debouncedTokenAmountUpdate("quote", balanceQuote);
            },
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={{
            value: quoteAsset ? quoteAsset.network[DEFAULT_NETWORK_ID] : "",
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
                      value: quoteAsset.network[DEFAULT_NETWORK_ID],
                      icon: quoteAsset.icon,
                      label: quoteAsset.symbol,
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
            {`≈$${quoteAmount.multipliedBy(priceUSDQuote)}`}
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
          borderRadius={9999}
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
          disabled={isProcessing}
          value={baseAmount}
          setValue={(value) => {
            if (isProcessing) return;
            debouncedTokenAmountUpdate("base", value);
          }}
          maxValue={balanceBase}
          setValid={setIsValidBaseInput}
          EndAdornmentAssetProps={{
            assets: baseAsset
              ? [
                  {
                    icon: baseAsset.icon,
                    label: baseAsset.symbol,
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
            disabled={isPendingBuy || isBuyButtonDisabled || isProcessing}
            onClick={() => handleBuy()}
          >
            Buy {baseAsset ? baseAsset.symbol : ""}
          </Button>
        )}

        {extensionStatus !== "connected" && (
          <Button
            variant="contained"
            fullWidth
            onClick={() => dispatch(openPolkadotModal())}
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
                {getFullHumanizedDateDiff(Date.now(), auction.sale.start)}.
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

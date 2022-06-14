import {
  alpha,
  Box,
  BoxProps,
  Button,
  Typography,
  useTheme,
} from "@mui/material";
import { useCallback, useEffect, useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import { DropdownCombinedBigNumberInput, BigNumberInput } from "@/components";
import { useMobile } from "@/hooks/responsive";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import { useAppDispatch } from "@/hooks/store";
import { openPolkadotModal } from "@/stores/ui/uiSlice";
import { getFullHumanizedDateDiff } from "@/utils/date";
import {
  useDotSamaContext,
  useExecutor,
  useParachainApi,
  usePendingExtrinsic,
  useSelectedAccount,
} from "substrate-react";
import { LiquidityBootstrappingPool } from "@/store/pools/pools.types";
import { getAssetById } from "@/defi/polkadot/Assets";
import { getSigner } from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
import useStore from "@/store/useStore";
import { onSwapAmountChange } from "@/updaters/swaps/utils";
import { debounce } from "lodash";
import { ConfirmingModal } from "../swap/ConfirmingModal";
import { useSnackbar } from "notistack";
import { toChainUnits } from "@/utils/defi";

export type BuyFormProps = {
  auction: LiquidityBootstrappingPool;
} & BoxProps;

export const BuyForm: React.FC<BuyFormProps> = ({ auction, ...rest }) => {
  const { balances } = useStore();
  const { extensionStatus } = useDotSamaContext();
  const { parachainApi } = useParachainApi("picasso");
  const selectedAccount = useSelectedAccount("picasso");
  const { enqueueSnackbar } = useSnackbar();
  const theme = useTheme();
  const isMobile = useMobile();
  const executor = useExecutor();
  const currentTimestamp = Date.now();

  const isActive: boolean =
    auction.sale.start <= currentTimestamp &&
    auction.sale.end >= currentTimestamp;
  const isEnded: boolean = auction.sale.end < currentTimestamp;

  const isPendingBuy = usePendingExtrinsic(
    "exchange",
    "dexRouter",
    selectedAccount ? selectedAccount.address : ""
  );

  const baseAsset = useMemo(() => {
    return getAssetById(auction.networkId, auction.pair.base);
  }, [auction]);
  const quoteAsset = useMemo(() => {
    return getAssetById(auction.networkId, auction.pair.quote);
  }, [auction]);

  const [balanceQuote, setBalanceQuote] = useState(new BigNumber(0));
  useEffect(() => {
    const asset = getAssetById("picasso", auction.pair.quote);
    if (asset) {
      setBalanceQuote(new BigNumber(balances[asset.assetId].picasso));
    } else {
      setBalanceQuote(new BigNumber(0));
    }
  }, [balances, baseAsset, auction.pair.quote]);

  const [balanceBase, setBalanceBase] = useState(new BigNumber(0));
  useEffect(() => {
    const asset = getAssetById("picasso", auction.pair.base);
    if (asset) {
      setBalanceBase(new BigNumber(balances[asset.assetId].picasso));
    } else {
      setBalanceBase(new BigNumber(0));
    }
  }, [balances, baseAsset, auction.pair.quote]);

  const [valid1, setValid1] = useState<boolean>(false);
  const [valid2, setValid2] = useState<boolean>(false);

  const dispatch = useAppDispatch();

  const buttonDisabled = useMemo(() => {
    return extensionStatus !== "connected" || !valid1 || !valid2;
  }, [extensionStatus, valid1, valid2]);

  const [baseAssetAmount, setBaseAmount] = useState(new BigNumber(0));
  const [quoteAssetAmount, setQuoteAmount] = useState(new BigNumber(0));
  const [minReceive, setMinReceive] = useState(new BigNumber(0));
  const [disableHandler, setDisableHandler] = useState(false);

  const onSwapAmountInput = (swapAmount: {
    value: BigNumber;
    side: "quote" | "base";
  }) => {
    setDisableHandler(true);

    if (parachainApi && baseAsset && quoteAsset) {
      const { value, side } = swapAmount;
      if (side === "base") {
        setBaseAmount(swapAmount.value);
      } else {
        setQuoteAmount(swapAmount.value);
      }

      const exchangeParams = {
        quoteAmount: value,
        baseAssetId: baseAsset.assetId,
        quoteAssetId: quoteAsset.assetId,
        side: side,
        slippage: 0.1,
      };

      onSwapAmountChange(parachainApi, exchangeParams, {
        poolAccountId: "",
        poolIndex: auction.poolId,
        fee: auction.feeConfig.feeRate.toString(),
        poolType: "LiquidityBootstrapping",
        pair: auction.pair,
        lbpConstants: undefined,
      }).then((impact) => {
        swapAmount.side === "base"
          ? setQuoteAmount(new BigNumber(impact.tokenOutAmount))
          : setBaseAmount(new BigNumber(impact.tokenOutAmount));
        setMinReceive(new BigNumber(impact.minimumRecieved));
        setTimeout(() => setDisableHandler(false), 1000);
      });
    }
  };
  const handler = debounce(onSwapAmountInput, 1000);

  const handleBuy = useCallback(async () => {
    if (parachainApi && selectedAccount && executor) {

      const minRec = parachainApi.createType(
        "u128",
        toChainUnits(minReceive)
      );
      const amountParam = parachainApi.createType(
        "u128",
        toChainUnits(baseAssetAmount)
      );

      try {
        const signer = await getSigner(APP_NAME, selectedAccount.address);

        await executor
          .execute(
            parachainApi.tx.dexRouter.exchange(
              auction.pair,
              amountParam,
              minRec
            ),
            selectedAccount.address,
            parachainApi,
            signer,
            (txHash: string) => {
              enqueueSnackbar('Initiating Transaction');
            },
            (txHash: string, events) => {
              enqueueSnackbar('Transaction Finalized');
            }
          )
          .catch((err) => {
            enqueueSnackbar(err.message);
          });
      } catch (err: any) {
        enqueueSnackbar(err.message);
      }
    }
  }, [parachainApi, executor, selectedAccount, baseAsset, baseAssetAmount]);

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
        <DropdownCombinedBigNumberInput
          onMouseDown={(evt) => setDisableHandler(false)}
          maxValue={balanceQuote}
          setValid={setValid1}
          noBorder
          value={quoteAssetAmount}
          setValue={(value) => {
            if (disableHandler) return;
            handler({
              value,
              side: "quote",
            });
            // set Value
          }}
          buttonLabel={"Max"}
          ButtonProps={{
            onClick: () => {},
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={{
            value: quoteAsset,
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
              ...[
                {
                  value: quoteAsset,
                  icon: quoteAsset ? quoteAsset.icon : "",
                  label: quoteAsset ? quoteAsset.symbol : "",
                },
              ],
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
          onMouseDown={(evt) => setDisableHandler(false)}
          value={baseAssetAmount}
          setValue={(value) => {
            if (disableHandler) return;
            handler({
              value,
              side: "base",
            });
          }}
          maxValue={balanceBase}
          setValid={setValid2}
          EndAdornmentAssetProps={{
            assets: [
              {
                icon: baseAsset ? baseAsset.icon : "",
                label: baseAsset ? baseAsset.symbol : "",
              },
            ],
          }}
          LabelProps={{
            label: "Launch token",
            BalanceProps: {
              title: <AccountBalanceWalletIcon color="primary" />,
              balance: `${balanceBase.toFixed(4)}`,
            },
          }}
        />
      </Box>

      <Box mt={4}>
        {extensionStatus === "connected" && (
          <Button
            variant="contained"
            fullWidth
            disabled={isPendingBuy || buttonDisabled}
            onClick={() => handleBuy()}
          >
            Buy {baseAsset ? baseAsset.symbol : ""}
          </Button>
        )}

        {/* {extensionStatus === "connected" && !approved && (
          <Button
            variant="contained"
            fullWidth
            onClick={() => setApproved(true)}
          >
            {!isActive ? `Buy ${getToken(tokenId2).symbol}` : `Approve ${getToken(tokenId1).symbol} usage`}
          </Button>
        )} */}

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
    </Box>
  );
};

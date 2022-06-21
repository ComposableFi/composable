import React, { useEffect, useMemo } from "react";
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
  closeConfirmingModal,
  closeSwapPreviewModal,
  openConfirmingModal,
} from "@/stores/ui/uiSlice";
import {
  useParachainApi,
  useExtrinsics,
  useSelectedAccount,
  useExecutor,
  getSigner,
} from "substrate-react";
import { SwapSummary } from "./SwapSummary";
import KeyboardArrowDownIcon from "@mui/icons-material/KeyboardArrowDown";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { APP_NAME } from "@/defi/polkadot/constants";
import { useSnackbar } from "notistack";
import { useAppSelector } from "@/hooks/store";
import { MockedAsset } from "@/store/assets/assets.types";
import { DEFAULT_NETWORK_ID, toChainUnits } from "@/defi/utils";

export type PreviewModalProps = {
  setConfirmed?: (confirmed: boolean) => any;
  baseAsset: MockedAsset | undefined;
  quoteAsset: MockedAsset | undefined;
  quoteAssetAmount: BigNumber;
  baseAssetAmount: BigNumber;
  minimumReceived: BigNumber;
} & ModalProps;

export const PreviewModal: React.FC<PreviewModalProps> = ({
  setConfirmed,
  baseAsset,
  quoteAsset,
  quoteAssetAmount,
  baseAssetAmount,
  minimumReceived,
  ...modalProps
}) => {
  const theme = useTheme();
  const { enqueueSnackbar } = useSnackbar();
  const dispatch = useDispatch();

  const { parachainApi } = useParachainApi("picasso");
  const { swaps } = useStore();
  const connectedAccount = useSelectedAccount("picasso");
  const executor = useExecutor();

  const priceImpact = 0;

  const spotPrice = useMemo(() => {
    return new BigNumber(swaps.poolVariables.spotPrice);
  }, [swaps.poolVariables]);

  const onConfirmSwap = async () => {
    if (
      parachainApi &&
      connectedAccount &&
      executor &&
      baseAsset && quoteAsset
    ) {
      try {

        const qtAmont = toChainUnits(baseAssetAmount);
        const minRec = toChainUnits(minimumReceived);

        const signer = await getSigner(APP_NAME, connectedAccount.address);

        let pair = {
          base: +baseAsset.network[DEFAULT_NETWORK_ID],
          quote: +quoteAsset.network[DEFAULT_NETWORK_ID],
        };

        executor.execute(
          parachainApi.tx.dexRouter.exchange(
            pair,
            parachainApi.createType("u128", qtAmont.toFixed(0)),
            parachainApi.createType("u128", minRec.toFixed(0))
          ),
          connectedAccount.address,
          parachainApi,
          signer,
          (txHash: string) => {
            console.log("TX Started: ", txHash);
          },
          (txHash: string, events) => {
            console.log("TX Finalized: ", txHash);
            enqueueSnackbar('Transaction Finalized');
            setConfirmed && setConfirmed(true);
            dispatch(closeConfirmingModal());
          },
          (txError: string) => {
            enqueueSnackbar('Transaction Failed', {
              description: txError,
              isClosable: true,
              url: '',
            });
            console.error(txError);
            dispatch(closeConfirmingModal());
          }
        ).catch(err => {
          enqueueSnackbar('Transaction Failed', {
            description: err.message,
            isClosable: true,
            url: '',
          });
          dispatch(closeConfirmingModal());
          console.log(err);
        });
      } catch (err: any) {
        enqueueSnackbar('Transaction Failed', {
          description: err.message,
          isClosable: true,
          url: '',
        });
        dispatch(closeConfirmingModal());
        console.log(err);
      }
    }
  };

  const confirmSwap = () => {
    dispatch(closeSwapPreviewModal());
    dispatch(openConfirmingModal());

    onConfirmSwap();
  };

  const slippage = useAppSelector(
    (state) => state.settings.transactionSettings.tolerance
  );

  return (
    <Modal onClose={() => dispatch(closeSwapPreviewModal())} {...modalProps}>
      <Box
        sx={{
          background: theme.palette.gradient.secondary,
          width: 560,
          [theme.breakpoints.down("sm")]: {
            width: "100%",
          },
          borderRadius: 0.5,
          padding: theme.spacing(4),
          boxShadow: `-1px -1px ${alpha(
            theme.palette.common.white,
            theme.custom.opacity.light
          )}`,
        }}
      >
        <Box display="flex" alignItems="center" justifyContent="space-between">
          <Typography variant="h6">Confirm swap</Typography>
          <IconButton onClick={() => dispatch(closeSwapPreviewModal())}>
            <CloseIcon sx={{ color: "text.secondary" }} />
          </IconButton>
        </Box>

        <Label
          mt={4}
          BalanceProps={{
            title: quoteAsset?.symbol,
            TitleTypographyProps: {
              variant: "body1",
              color: "text.primary",
            },
          }}
        >
          <BaseAsset
            icon={quoteAsset?.icon}
            label={quoteAssetAmount.toFixed()}
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
            title: baseAsset?.symbol,
            TitleTypographyProps: {
              variant: "body1",
              color: "text.primary",
            },
          }}
        >
          <BaseAsset
            icon={baseAsset?.icon}
            label={baseAssetAmount.toFixed()}
            LabelProps={{ variant: "body1" }}
          />
        </Label>

        <Typography variant="body2" mt={4} textAlign="center" paddingX={4.75}>
          Output is estimated. If the price changes by more than {slippage}% your
          transaction will revert.
        </Typography>

        <Box
          mt={4}
          borderTop={`2px solid ${alpha(
            theme.palette.common.white,
            theme.custom.opacity.light
          )}`}
        />

        <SwapSummary
          mt={4}
          quoteAssetAmount={quoteAssetAmount}
          poolType={swaps.poolConstants.poolType}
          baseAsset={baseAsset}
          quoteAsset={quoteAsset}
          minimumReceived={minimumReceived}
          priceImpact={priceImpact}
          PriceImpactProps={{
            color: "success.main",
          }}
          baseAssetAmount={baseAssetAmount}
          fee={new BigNumber(swaps.poolConstants.fee).div(100)}
          price={spotPrice}
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

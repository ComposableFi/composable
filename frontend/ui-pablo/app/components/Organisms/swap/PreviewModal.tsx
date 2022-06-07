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
import { AssetId } from "@/defi/polkadot/types";
import { Assets } from "@/defi/polkadot/Assets";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { APP_NAME } from "@/defi/polkadot/constants";
import { useSnackbar } from "notistack";

export type PreviewModalProps = {
  setConfirmed?: (confirmed: boolean) => any;
  baseAssetId: AssetId | "none";
  quoteAssetId: AssetId | "none";
  quoteAssetAmount: BigNumber;
  baseAssetAmount: BigNumber;
  minimumReceived: BigNumber;
} & ModalProps;

export const PreviewModal: React.FC<PreviewModalProps> = ({
  setConfirmed,
  baseAssetId,
  quoteAssetId,
  quoteAssetAmount,
  baseAssetAmount,
  minimumReceived,
  ...modalProps
}) => {
  const theme = useTheme();
  const dispatch = useDispatch();
  const { enqueueSnackbar } = useSnackbar();
  const connectedAccount = useSelectedAccount("picasso");
  const executor = useExecutor();
  const { parachainApi } = useParachainApi("picasso");
  const { swaps } = useStore();

  const priceImpact = 0;

  const baseAsset = baseAssetId === "none" ? null : Assets[baseAssetId];
  const quoteAsset = quoteAssetId === "none" ? null : Assets[quoteAssetId];

  const spotPrice = useMemo(() => {
    return new BigNumber(swaps.poolVariables.spotPrice);
  }, [swaps.poolVariables]);

  const onConfirmSwap = async () => {
    const { ui } = swaps;
    const {baseAssetSelected, quoteAssetSelected} = ui;
    if (
      parachainApi &&
      connectedAccount &&
      executor &&
      baseAssetSelected !== "none" &&
      quoteAssetSelected !== "none"
    ) {
      try {
        const decimalsBase = new BigNumber(10).pow(
          Assets[quoteAssetSelected].decimals
        );

        const base = Assets[baseAssetSelected].supportedNetwork.picasso;
        const quote = Assets[quoteAssetSelected].supportedNetwork.picasso;

        const qtAmont = baseAssetAmount.times(decimalsBase);
        const minRec = minimumReceived.times(decimalsBase);

        const signer = await getSigner(APP_NAME, connectedAccount.address);

        let pair = {
          base,
          quote,
        };

        console.log('Minimum Recieve: ', minRec.toFixed(0))
        console.log('Quote Amount: ', qtAmont .toFixed(0))
        console.log('Exchange: ', quote === swaps.poolConstants.pair.quote, pair)

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
          Output is estimated. If the price changes by more than 5% your
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
          baseAssetId={baseAssetId}
          quoteAssetId={quoteAssetId}
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

import React from "react";
import { ModalProps, Modal } from "@/components/Molecules";
import { Label, BaseAsset } from "@/components/Atoms";
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
  closeConfirmingSupplyModal,
  closeConfirmSupplyModal, 
  openConfirmingSupplyModal, 
} from "@/stores/ui/uiSlice";
import { useAppSelector } from "@/hooks/store";
import BigNumber from "bignumber.js";
import { getSigner, useExecutor, useParachainApi, useSelectedAccount } from "substrate-react";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils/constants";
import { APP_NAME } from "@/defi/polkadot/constants";
import { useSnackbar } from "notistack";
import { resetAddLiquiditySlice, useAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import { useRouter } from "next/router";
import { MockedAsset } from "@/store/assets/assets.types";
import { toChainUnits } from "@/defi/utils";

export interface SupplyModalProps {
  assetOne: MockedAsset | undefined;
  assetTwo: MockedAsset | undefined;
  assetOneAmount: BigNumber;
  assetTwoAmount: BigNumber;
  lpReceiveAmount: BigNumber;
  priceOneInTwo: BigNumber;
  priceTwoInOne: BigNumber;
  share: BigNumber;
}

export const ConfirmSupplyModal: React.FC<SupplyModalProps & ModalProps> = ({
  assetOne,
  assetTwo,
  assetOneAmount,
  assetTwoAmount,
  lpReceiveAmount,
  priceOneInTwo,
  priceTwoInOne,
  share,
  ...rest
}) => {
  const { enqueueSnackbar } = useSnackbar();
  const theme = useTheme();
  const dispatch = useDispatch();
  const router = useRouter();

  const {parachainApi} = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const executor = useExecutor();

  const addLiquiditySlice = useAddLiquiditySlice();

  const onConfirmSupply = async () => {
    if (
      selectedAccount &&
      executor && parachainApi && selectedAccount &&
      assetOne !== null &&
      assetTwo !== null
      && addLiquiditySlice.pool) {
        dispatch(closeConfirmSupplyModal());
  
      let isReverse =
        addLiquiditySlice.pool.pair.base.toString() !== assetOne?.network.picasso;
      const bnBase = toChainUnits(isReverse ? assetTwoAmount : assetOneAmount);
      const bnQuote = toChainUnits(isReverse ? assetOneAmount : assetTwoAmount);

      const signer = await getSigner(APP_NAME, selectedAccount.address);

      executor.execute(
        parachainApi.tx.pablo.addLiquidity(addLiquiditySlice.pool.poolId, bnBase.toString(), bnQuote.toString(), 0, true),
        selectedAccount.address,
        parachainApi,
        signer,
        (txReady: string) => {
          dispatch(openConfirmingSupplyModal());
          console.log('txReady', txReady)
        },
        (txHash: string, events) => {
          console.log('Finalized TX: ', txHash)
          enqueueSnackbar('Added Liquidity: ' + txHash)
          dispatch(closeConfirmingSupplyModal());
          router.push('/pool/select/' + addLiquiditySlice.pool?.poolId)
          resetAddLiquiditySlice();
        },
        (errorMessage: string) => {
          console.log('Tx Error:', errorMessage)
          enqueueSnackbar('Tx Error: ' + errorMessage)
          dispatch(closeConfirmingSupplyModal());
        }
      ).catch(err => {
        dispatch(closeConfirmingSupplyModal());
        console.log('Tx Error:', err)
      })
    }
  }

  return (
    <Modal
      onClose={() => dispatch(closeConfirmSupplyModal())}
      {...rest}
    >
      <Box
        sx={{
          background: theme.palette.gradient.secondary,
          width: 550,
          [theme.breakpoints.down('sm')]: {
            width: '100%',
          },
          borderRadius: 1,
          padding: theme.spacing(3),
          boxShadow: `-1px -1px ${alpha(theme.palette.common.white, theme.custom.opacity.light)}`,
        }}
      >
        <Box
          display="flex"
          alignItems="center"
          justifyContent="space-between"
        >
          <Typography variant="body1">
            You will receive
          </Typography>
          <IconButton 
            onClick={() => dispatch(closeConfirmSupplyModal())}
          >
            <CloseIcon />
          </IconButton>
        </Box>

        <Typography variant="h5" mt={1.75}>
          {`${lpReceiveAmount.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}`}
        </Typography>

        <Typography variant="body1" color="text.secondary" mt={1.75}>
          {`LP ${assetOne?.symbol}/${assetTwo?.symbol} Tokens`}
        </Typography>

        <Typography variant="body2" mt={4} textAlign="center" paddingX={4.25}>
          Output is estimated. If the price changes by more than 5% your transaction will revert.
        </Typography>

        <Box
          mt={4}
          borderTop={`1px solid ${alpha(theme.palette.common.white, theme.custom.opacity.main)}`}
        />

        <Label
          mt={4}
          label={`Pooled ${assetOne?.symbol}`}
          BalanceProps={{
            title: <BaseAsset icon={assetOne?.icon} pr={priceOneInTwo.toNumber()} />,
            balance: `${assetOneAmount}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Pooled ${assetTwo?.symbol}`}
          BalanceProps={{
            title: <BaseAsset icon={assetTwo?.icon} pr={priceTwoInOne.toNumber()} />,
            balance: `${assetTwoAmount}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Price`}
          BalanceProps={{
            balance: `1 ${assetOne?.symbol} = ${priceOneInTwo} ${assetTwo?.symbol}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label=""
          BalanceProps={{
            balance: `1 ${assetTwo?.symbol} = ${priceTwoInOne} ${assetOne?.symbol}`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Label
          mt={2}
          label={`Share of pool`}
          BalanceProps={{
            balance: `${share.toFixed(4)}%`,
            BalanceTypographyProps: {
              variant: "body1",
            },
          }}
        />

        <Box mt={4}>
          <Button 
            variant="contained" 
            size="large"
            fullWidth
            onClick={onConfirmSupply}
          >
            Confirm supply
          </Button>
        </Box>      
      </Box>
    </Modal>  
  );
};

